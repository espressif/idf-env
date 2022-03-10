use std::env;
use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use dirs::home_dir;
use std::path::Path;
use std::fs::{create_dir_all, remove_dir_all};
use std::io::Read;
use std::process::Stdio;
use crate::config::get_tool_path;
use crate::package::{prepare_package, prepare_package_strip_prefix, prepare_single_binary};
use crate::shell::run_command;

const DEFAULT_IDE_URL:&str = "https://dl.espressif.com/dl/idf-eclipse-plugin/ide/Espressif-IDE-2.4.2-win32.win32.x86_64.zip";
const DEFAULT_IDE_FILE:&str = "Espressif-IDE-2.4.2-win32.win32.x86_64.zip";

struct Ide {
    dist_url: String,
    dist_file: String,
    destination_dir: String,
    prefix: String
}

fn install_ide(ide:&Ide) {

    prepare_package_strip_prefix(&ide.dist_url,
                                 &ide.dist_file,
                                 ide.destination_dir.clone(),
                                 &ide.prefix);


}

fn get_install_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let ide = Ide {
        dist_url: matches.value_of("url").unwrap().to_string(),
        dist_file: matches.value_of("file").unwrap().to_string(),
        destination_dir: matches.value_of("destination").unwrap().to_string(),
        prefix: "Espressif-IDE".to_string()
    };

    install_ide(&ide);
    Ok(())
}

pub fn get_install_cmd<'a>() -> Command<'a, str> {
    Command::new("install")
        .description("Install Espressif-IDE")
        .options(|app| {
            app.arg(
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .help("URL to Espressif-IDE download")
                    .takes_value(true)
                    .default_value(DEFAULT_IDE_URL)
            )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .help("Archive with Espressif-IDE")
                        .takes_value(true)
                        .default_value(DEFAULT_IDE_FILE)
                )
                .arg(
                    Arg::with_name("destination")
                        .short("d")
                        .long("destination")
                        .help("Location where Espressif-IDE should be deployed")
                        .takes_value(true)
                )
        })
        .runner(|_args, matches|
            get_install_runner(_args, matches)
        )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_install_cmd())
        .into_cmd("ide")

        // Optionally specify a description
        .description("Maintain Espressif-IDE.");

    return multi_cmd;
}
