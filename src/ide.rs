use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use std::fs::File;
use std::io::{self, BufRead, Write};

use std::path::Path;
use crate::package::{prepare_package, prepare_package_strip_prefix, prepare_single_binary};

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn set_vm_to_ini_file(ini_file: String, vm_path: String) {
    match read_lines(ini_file.clone()) {
        Ok(lines) => {
            let memory_buffer:Vec<String> = lines.map(|r|r.unwrap()).collect();
            let index = memory_buffer.iter().position(|r| r == "-vm");
            match index {
                Some(index) => {
                    let mut position = 0;
                    let mut out_file = File::create(ini_file).unwrap();
                    for line in memory_buffer {
                        if index+1 == position {
                            out_file.write_all(vm_path.as_bytes());
                            out_file.write_all("\r\n".as_bytes());
                        } else {
                            out_file.write_all( line.as_bytes());
                            out_file.write_all("\r\n".as_bytes());
                        }
                        position = position + 1;
                    }
                }
                _ => {
                    let mut out_file =File::create(ini_file).unwrap();
                    out_file.write_all("-vm\r\n".as_bytes());
                    out_file.write_all(vm_path.as_bytes());
                    out_file.write_all("\r\n".as_bytes());
                    for line in memory_buffer {
                        out_file.write_all(line.as_bytes());
                        out_file.write_all("\r\n".as_bytes());
                    }
                }
            }
        }
        _ => {
            println!("Unable to open file {}", ini_file);
        }
    }
}

fn get_configure_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let ini_file = matches.value_of("ini").unwrap().to_string();
    let vm_path = matches.value_of("vm").unwrap().to_string();

    set_vm_to_ini_file(ini_file, vm_path);
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

pub fn get_configure_cmd<'a>() -> Command<'a, str> {
    Command::new("configure")
        .description("Configure Espressif-IDE")
        .options(|app| {
            app.arg(
                Arg::with_name("ini")
                    .short("i")
                    .long("ini")
                    .help("Path to Espressif-IDE ini file")
                    .takes_value(true)
            )
                .arg(
                    Arg::with_name("vm")
                        .short("m")
                        .long("vm")
                        .help("Path to VM")
                        .takes_value(true)
                )
        })
        .runner(|_args, matches|
            get_configure_runner(_args, matches)
        )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_install_cmd())
        .add_cmd(get_configure_cmd())
        .into_cmd("ide")

        // Optionally specify a description
        .description("Maintain Espressif-IDE.");

    return multi_cmd;
}
