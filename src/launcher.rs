use clap::{Arg, ArgMatches};
use clap_nested::{Command, Commander, MultiCommand};

use std::{env, fs};
use crate::config::{get_tools_path, get_idf_id};

fn get_windows_terminal_fragments_path(title: &str) -> String {
    let local_app_data = env::var("LocalAppData").unwrap();
    format!("{}/Microsoft/Windows Terminal/Fragments/{}", local_app_data, title)
}

fn get_powershell_path() -> String {
    let windir = env::var("windir").unwrap();
    format!("{}/System32/WindowsPowerShell/v1.0/powershell.exe", windir)
}

fn get_add_runner(_args: &str, matches: &ArgMatches) -> std::result::Result<(), clap::Error> {
    let title = matches.value_of("title").unwrap();
    let idf_path = matches.value_of("idf-path").unwrap();
    let fragments_path = get_windows_terminal_fragments_path(title);
    let tools_path = get_tools_path();
    let idf_id = get_idf_id(idf_path);

    // After fresh installation of Windows Terminal the fragment path does not exist.
    // Microsoft recommends to create one
    fs::create_dir_all(&fragments_path)?;
    let fragment_json_path = format!("{}/fragment.json", fragments_path);
    println!("Updating Windows Terminal Fragment: {}", fragment_json_path);

    let command_line = format!("{} -ExecutionPolicy Bypass -NoExit -File {}/Initialize-Idf.ps1 -IdfId {}", get_powershell_path(), tools_path, idf_id);

    let profile_json = json::object! {
        "name": title,
        "startingDirectory": idf_path,
        "commandline": command_line
    };

    let json_value = json::object!{
        "profiles": [ profile_json ]
    };

    let json_string = json_value.to_string();
    println!("{}", json_string);
    fs::write(fragment_json_path, json_string).unwrap();

    Ok(())
}


pub fn get_add_cmd<'a>() -> Command<'a, str> {
    Command::new("add")
        .description("Add ESP-IDF launcher")
        .options(|app| {
            app.arg(
                Arg::with_name("shell")
                    .short("s")
                    .long("shell")
                    .takes_value(true)
                    .help("Shell which should be launched: powershell, cmd"),
            )
                .arg(
                    Arg::with_name("to")
                        .short("t")
                        .long("to")
                        .takes_value(true)
                        .help("Where to add the launcher: desktop, start-menu, windows-terminal"),
                )
                .arg(
                    Arg::with_name("title")
                        .short("i")
                        .long("title")
                        .takes_value(true)
                        .help("Title displayed on launcher"),
                )
                .arg(
                    Arg::with_name("idf-path")
                        .short("x")
                        .long("idf-path")
                        .takes_value(true)
                        .help("Path to ESP-IDF"),
                )
        })
        .runner(|_args, matches| get_add_runner(_args, matches)
        )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_add_cmd())
        .into_cmd("launcher")

        // Optionally specify a description
        .description("Manage ESP-IDF launchers.");

    return multi_cmd;
}
