use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use std::process::Stdio;
use std::io::{self, Write};

use crate::driver::windows;
use crate::config;

use walkdir::{WalkDir, DirEntry};

fn is_filter_match(entry: DirEntry, filter: &str) -> bool {
    println!("M{}", entry.path().display());
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("e"))
        .unwrap_or(false)
}

fn get_tool_files(tool_name: String, filter: String) -> Vec<String> {
    let tool_path = config::get_tool_path(tool_name);
    let mut result_list: Vec<String> = [].to_vec();
    for e in WalkDir::new(tool_path).into_iter().filter_map(|e| e.ok()) {
        let metadata = e.metadata().unwrap();
        if metadata.is_file() && e.file_name().to_string_lossy().ends_with(&filter) {
            // println!("{}", e.path().display());
            result_list.push(e.path().display().to_string());
        }
    }
    result_list
}

fn add_exclusions(path:String) {
    let mut arguments: Vec<String> = [].to_vec();
    arguments.push("Add-MpPreference".to_string());
    arguments.push("-ExclusionPath".to_string());
    arguments.push(path.clone());
    arguments.push("-ExclusionProcess".to_string());
    arguments.push(path.clone());

    let mut self_arguments: Vec<String> = [].to_vec();
    self_arguments.push("antivirus".to_string());
    self_arguments.push("exclusion".to_string());
    self_arguments.push("add".to_string());
    self_arguments.push("--path".to_string());
    self_arguments.push(path);

    println!("Registering exclusion: powershell {:?}", arguments);
    windows::run_self_elevated("powershell".to_string(), arguments, self_arguments);
}

fn get_add_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    if matches.is_present("all") {
        let file_list = get_tool_files("".to_string(), ".exe".to_string());
        let exclusion_paths = file_list.join(",");
        add_exclusions(exclusion_paths);
    }

    if matches.is_present("tool") {
        let tool_name = matches.value_of("tool").unwrap().to_string();
        let file_list = get_tool_files(tool_name, ".exe".to_string());
        let exclusion_paths = file_list.join(",");
        add_exclusions(exclusion_paths);
    }

    if matches.is_present("path") {
        let path = matches.value_of("path").unwrap().to_string();
        add_exclusions(path);
    }

    Ok(())
}

fn remove_exclusions(path:String) {
    let mut arguments: Vec<String> = [].to_vec();
    arguments.push("Remove-MpPreference".to_string());
    arguments.push("-ExclusionPath".to_string());
    arguments.push(path.clone());
    arguments.push("-ExclusionProcess".to_string());
    arguments.push(path.clone());

    let mut self_arguments: Vec<String> = [].to_vec();
    self_arguments.push("antivirus".to_string());
    self_arguments.push("exclusion".to_string());
    self_arguments.push("remove".to_string());
    self_arguments.push("--path".to_string());
    self_arguments.push(path);

    println!("Registering exclusion: powershell {:?}", arguments);
    windows::run_self_elevated("powershell".to_string(), arguments, self_arguments);
}

fn get_remove_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    if matches.is_present("all") {
        let file_list = get_tool_files("".to_string(), ".exe".to_string());
        let exclusion_paths = file_list.join(",");
        remove_exclusions(exclusion_paths);
    }

    if matches.is_present("tool") {
        let tool_name = matches.value_of("tool").unwrap().to_string();
        let file_list = get_tool_files(tool_name, ".exe".to_string());
        let exclusion_paths = file_list.join(",");
        remove_exclusions(exclusion_paths);
    }

    if matches.is_present("path") {
        let path = matches.value_of("path").unwrap().to_string();
        remove_exclusions(path);
    }

    Ok(())
}


pub fn get_add_cmd<'a>() -> Command<'a, str> {
    Command::new("add")
        .description("Exclude path from scanning by antivirus")
        .options(|app| {
            app.arg(
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .help("Add path to exclusions")
                    .takes_value(true)
            )
                .arg(
                    Arg::with_name("tool")
                        .short("t")
                        .long("tool")
                        .help("Name of ESP-IDF tool which should be excluded from antivirus scanning")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Register all tools exclusions")
                )
        })
        .runner(|_args, matches| get_add_runner(_args, matches) )
}


pub fn get_remove_cmd<'a>() -> Command<'a, str> {
    Command::new("remove")
        .description("Remove excluded path to enable antivirus scanning")
        .options(|app| {
            app.arg(
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .help("Remove path from exclusions")
                    .takes_value(true)
            )
                .arg(
                    Arg::with_name("tool")
                        .short("t")
                        .long("tool")
                        .help("Name of ESP-IDF tool which should be removed from antivirus exclusions")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Remove registration of all tools from antivirus exclusions")
                )
        })
        .runner(|_args, matches| get_remove_runner(_args, matches) )
}


pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        // .add_cmd(get_cmd())
        .add_cmd(get_add_cmd())
        .add_cmd(get_remove_cmd())
        .into_cmd("exclusion")

        // Optionally specify a description
        .description("Work with antivirus exclusions.");

    return multi_cmd;
}
