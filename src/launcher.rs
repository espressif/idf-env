use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use std::{env, fs};
use std::path::Path;
//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]
extern crate serde_hjson;
use serde_hjson::{Map,Value};


fn get_json_path() -> String {
    let local_app_data = env::var("LocalAppData").unwrap();
    format!("{}/Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json", local_app_data)
}

fn load_json() -> Result<String, String> {
    let json_path = get_json_path();
    println!("Loading configuration of Windows Terminal: {}", json_path);
    if !Path::new(&json_path).exists() {
        let result = Err(format!("Windows Terminal configuration not found: {}", json_path));
        return result
    }


    let content = fs::read_to_string(json_path)
        .expect("Failure");

    let mut out: Value = serde_hjson::from_str(&content.to_string()).unwrap();
    let sample2 = serde_hjson::to_string(&out).unwrap();
    // let data: Value = serde_hjson::from_str("{foo: 13, bar: \"baz\"}").unwrap();
    println!("{}", sample2);
    Ok("Done".to_string())
}

fn get_add_runner(_args: &str, _matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    match load_json() {
        Err(e) => { println!("{}", e); }
        Ok(v) => { println!("ok") }
    }
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
