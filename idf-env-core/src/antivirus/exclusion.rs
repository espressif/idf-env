use clap::{Arg, App};

#[cfg(windows)]
use crate::driver::windows;
use crate::config;

use walkdir::{WalkDir};

fn get_tool_files(tool_name: String, filter: String) -> Vec<String> {
    let tool_path = config::get_tool_path(tool_name);
    let mut result_list: Vec<String> = [].to_vec();
    for e in WalkDir::new(tool_path).into_iter().filter_map(|e| e.ok()) {
        let metadata = e.metadata().unwrap();
        if metadata.is_file() && e.file_name().to_string_lossy().ends_with(&filter) {
            // println!("{}", e.path().display());
            result_list.push(e.path().display().to_string().replace("/", "\\"));
        }
    }
    result_list
}

fn process_exclusion(operation: String, file_list:Vec<String>, chunk_size: usize) {
    let chunks = file_list.len() / chunk_size + 1;
    for chunk_index in 0..chunks {
        let start_index = chunk_index * chunk_size;
        let mut remaining_size = chunk_size;
        if start_index + chunk_size > file_list.len() {
            remaining_size = file_list.len() - start_index;
        }
        if remaining_size == 0 {
            continue
        }
        println!("Processing batch {}", chunk_index + 1);
        let path = file_list[start_index..start_index + remaining_size].join(",");
        let mut arguments: Vec<String> = [].to_vec();
        arguments.push(operation.clone());
        arguments.push("-ExclusionPath".to_string());
        arguments.push(path.clone());
        arguments.push("-ExclusionProcess".to_string());
        arguments.push(path.clone());

        #[cfg(windows)]
        windows::run("powershell".to_string(), arguments);
    }
    // thread::sleep(time::Duration::from_millis(100000));

}

fn add_exclusions(file_list:Vec<String>, chunk_size: usize) {
    process_exclusion("Add-MpPreference".to_string(), file_list, chunk_size);
}

fn nuke_exclusions() {
    #[cfg(windows)]
    windows::run_with_stdin("powershell".to_string(), "foreach ($Path in (Get-MpPreference).ExclusionPath) { Remove-MpPreference -ExclusionPath $Path }".to_string());
     #[cfg(windows)]
    windows::run_with_stdin("powershell".to_string(), "foreach ($Process in (Get-MpPreference).ExclusionProcess) { Remove-MpPreference -ExclusionProcess $Process }".to_string());
}

fn get_add_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    #[cfg(windows)]
    if !windows::is_app_elevated() {
        #[cfg(windows)]
        windows::run_self_elevated();
        return Ok(());
    }

    let chunk_size:usize = matches.value_of("chunk").unwrap().to_string().parse().unwrap();

    if matches.is_present("all") {
        let file_list = get_tool_files("".to_string(), ".exe".to_string());
        add_exclusions(file_list, chunk_size);
    }

    if matches.is_present("tool") {
        let tool_name = matches.value_of("tool").unwrap().to_string();
        let file_list = get_tool_files(tool_name, ".exe".to_string());
        add_exclusions(file_list, chunk_size);
    }

    if matches.is_present("path") {
        let file_list:Vec<String> = vec![matches.value_of("path").unwrap().to_string()];
        add_exclusions(file_list, chunk_size);
    }

    Ok(())
}

fn remove_exclusions(file_list:Vec<String>, chunk_size:usize) {
    process_exclusion("Remove-MpPreference".to_string(), file_list, chunk_size);
}

fn get_remove_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    #[cfg(windows)]
    if !windows::is_app_elevated() {
        #[cfg(windows)]
        windows::run_self_elevated();
        return Ok(());
    }

    let chunk_size:usize = matches.value_of("chunk").unwrap().to_string().parse().unwrap();

    if matches.is_present("all") {
        let file_list = get_tool_files("".to_string(), ".exe".to_string());
        remove_exclusions(file_list, chunk_size);
    }

    if matches.is_present("tool") {
        let tool_name = matches.value_of("tool").unwrap().to_string();
        let file_list = get_tool_files(tool_name, ".exe".to_string());
        remove_exclusions(file_list, chunk_size);
    }

    if matches.is_present("path") {
        let file_list:Vec<String> = vec![matches.value_of("path").unwrap().to_string()];
        remove_exclusions(file_list, chunk_size);
    }

    if matches.is_present("nuke") {
        println!("Deleting Absolutely ALL Exclusions");
        nuke_exclusions();
    }

    Ok(())
}


pub fn get_add_cmd<'a>() -> App<'a> {
    App::new("add")
        .about("Exclude path from scanning by antivirus")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Add path to exclusions")
                .takes_value(true)
        )
        .arg(
            Arg::new("tool")
                .short('t')
                .long("tool")
                .help("Name of ESP-IDF tool which should be excluded from antivirus scanning")
                .takes_value(true)
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Register all tools exclusions")
        )
        .arg(
            Arg::new("chunk")
                .short('c')
                .long("chunk")
                .help("Number of exclusions sent to antivirus in one batch")
                .default_value("20")
        )

        // .runner(|_args, matches| get_add_runner(_args, matches) )
}


pub fn get_remove_cmd<'a>() -> App<'a> {
    App::new("remove")
        .about("Remove excluded path to enable antivirus scanning")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Remove path from exclusions")
                .takes_value(true)
        )
        .arg(
            Arg::new("tool")
                .short('t')
                .long("tool")
                .help("Name of ESP-IDF tool which should be removed from antivirus exclusions")
                .takes_value(true)
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Remove registration of all tools from antivirus exclusions")

        )
        .arg(
            Arg::new("chunk")
                .short('c')
                .long("chunk")
                .help("Number of exclusions sent to antivirus in one batch")
                .default_value("20")
        )
        .arg(
            Arg::new("nuke")
                .short('x')
                .long("nuke")
                .help("Obliterate Absolutely ALL exclusions at once")
        )
        // .runner(|_args, matches| get_remove_runner(_args, matches) )
}


pub fn get_multi_cmd<'a>() -> App<'a> {
    App::new("exclusion")
        .about("Work with antivirus exclusions.")
        .subcommand(get_add_cmd())
        .subcommand(get_remove_cmd())
}
