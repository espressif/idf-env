use clap::{Arg, App};

use crate::package::{prepare_package, remove_package};
use std::process::Stdio;
use std::io::Read;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn prepare_companion() {
    prepare_package("https://dl.espressif.com/dl/esp-iwidc/esp-iwidc.zip".to_string(),
                    "esp-iwidc.zip",
                    "tmp/esp-iwidc".to_string());
}

fn remove_companion() -> Result<()> {
    remove_package("esp-iwidc.zip",
                    "tmp/esp-iwidc")
}

fn get_update_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    match remove_companion() {
        Ok(content) => {
            prepare_companion();
            println!("Web Companion updated");
        }
        Err(error) => { println!("{}", error);  }
    }

    Ok(())
}


fn get_companion_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    prepare_companion();

    let mut arguments: Vec<String> = [].to_vec();

    if matches.is_present("port") {
        let port = matches.value_of("port").unwrap().to_string();
        arguments.push("--port".to_string());
        arguments.push(port);
    }

    println!("Starting process");
    let process = std::process::Command::new("tmp/esp-iwidc/main.exe")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().unwrap();

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read stdout: {}", why),
        Ok(_) => print!("{}", s),
    }

    Ok(())

}

pub fn get_start_cmd<'a>() -> App<'a> {
    App::new("start")
        .about("Start the companion")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Name of communication port")
                .takes_value(true)
        )
        // .runner(|_args, matches| get_companion_runner(_args, matches) )
}


pub fn get_update_cmd<'a>() -> App<'a> {
    App::new("update")
        .about("Update the companion from the server")
        // .runner(|_args, matches| get_update_runner(_args, matches) )
}


pub fn get_multi_cmd<'a>() -> App<'a> {
    App::new("companion")
        .about("ESP-IDF Desktop Web Companion for flashing and monitoring device from Web IDE.")
        .subcommand(get_start_cmd())
        .subcommand(get_update_cmd())
}
