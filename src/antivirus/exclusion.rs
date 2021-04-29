use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use std::process::Stdio;
use std::io::{self, Write};

use crate::driver::windows;

fn get_add_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {

    let path = matches.value_of("path").unwrap().to_string();

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
    Ok(())
}


fn get_remove_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {

    let path = matches.value_of("path").unwrap().to_string();

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
