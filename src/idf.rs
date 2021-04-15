use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
use std::path::Path;
use std::io::Cursor;
use std::process;
use tokio::runtime::Handle;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use std::env;

use crate::config::get_git_path;

fn get_installer(matches: &clap::ArgMatches) -> String {
    if matches.is_present("installer") {
        return matches.value_of("installer").unwrap().to_string();
    }
    return "installer.exe".to_string();
}

async fn fetch_url(url: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create("installer.exe")?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

fn download_installer() -> Result<()> {
    if Path::new("installer.exe").exists() {
        println!("Using cached installer.");
        return Ok(());
    }
    let url_string = "https://github.com/espressif/idf-installer/releases/download/online-2.7-beta-06/esp-idf-tools-setup-online-2.7-beta-06.exe".to_string();

    let handle = Handle::current().clone();
    let th = std::thread::spawn(move || {
        handle.block_on(fetch_url(url_string))
    });
    th.join().unwrap()
}

fn execute_command(command: String, arguments: Vec<String>) -> Result<()> {
    let argument_string = arguments.clone().into_iter().map(|i| format!("{} ", i.to_string())).collect::<String>();
    println!("Executing: {} {}", command, argument_string);
    std::process::Command::new(command)
        .args(arguments)
        .output()
        .expect("failed to execute process");
    Ok(())
}

fn reset_repository(repository_path: String) -> Result<()> {
    let idf_path = Path::new(&repository_path);
    assert!(env::set_current_dir(&idf_path).is_ok());
    println!("Working directory: {}", idf_path.display());

    let git_path = get_git_path();
    let mut arguments: Vec<String> = [].to_vec();
    arguments.push("reset".to_string());
    arguments.push("--hard".to_string());
    assert!(execute_command(git_path, arguments).is_ok());

    let mut arguments_submodule: Vec<String> = [].to_vec();
    arguments_submodule.push("submodule".to_string());
    arguments_submodule.push("foreach".to_string());
    arguments_submodule.push("git".to_string());
    arguments_submodule.push("reset".to_string());
    arguments_submodule.push("--hard".to_string());
    assert!(execute_command(get_git_path(), arguments_submodule).is_ok());

    let mut arguments_clean: Vec<String> = [].to_vec();
    arguments_clean.push("clean".to_string());
    arguments_clean.push("force".to_string());
    arguments_clean.push("-d".to_string());
    assert!(execute_command(get_git_path(), arguments_clean).is_ok());

    let mut arguments_status: Vec<String> = [].to_vec();
    arguments_status.push("status".to_string());
    assert!(execute_command(get_git_path(), arguments_status).is_ok());

    Ok(())
}

fn get_reset_cmd<'a>() -> Command<'a, str> {
    Command::new("reset")
        .description("Reset ESP-IDF git repository to initial state and wipe out modified data")
        .options(|app| {
            app.arg(
                Arg::with_name("idf-path")
                    .short("d")
                    .long("idf-path")
                    .help("Path to existing ESP-IDF")
                    .takes_value(true)
            )
        })
        .runner(|_args, matches| {
            if matches.value_of("idf-path").is_some() {
                let dir = matches.value_of("idf-path").unwrap();
                assert!(reset_repository(dir.to_string()).is_ok());
            }
            Ok(())
        })
}

pub fn get_install_cmd<'a>() -> Command<'a, str> {
    Command::new("install")
        .description("Install new instance of IDF")
        .options(|app| {
            app.arg(
                Arg::with_name("installer")
                    .short("e")
                    .long("installer")
                    .help("Path to installer binary"),
            )
                .arg(
                    Arg::with_name("interactive")
                        .short("i")
                        .long("interactive")
                        .help("Run installation in interactive mode"),
                )
                .arg(
                    Arg::with_name("upgrade")
                        .short("u")
                        .long("upgrade")
                        .takes_value(false)
                        .help("Upgrade existing installation"))
                .arg(
                    Arg::with_name("idf-version")
                        .short("x")
                        .long("idf-version")
                        .takes_value(true)
                        .help("ESP-IDF version"))
                .arg(
                    Arg::with_name("idf-path")
                        .short("d")
                        .long("idf-path")
                        .takes_value(true)
                        .help("ESP-IDF installation directory"))
                .arg(
                    Arg::with_name("verbose")
                        .short("w")
                        .long("verbose")
                        .takes_value(false)
                        .help("display diagnostic log after installation"))
        })
        .runner(|_args, matches| {
            let mut arguments: Vec<String> = [].to_vec();

            if !matches.is_present("installer") {
                download_installer().unwrap();
            }

            if !matches.is_present("interactive") {
                arguments.push("/VERYSILENT".to_string());
                arguments.push("/SUPPRESSMSGBOXES".to_string());
                arguments.push("/SP-".to_string());
                arguments.push("/NOCANCEL".to_string());
            }

            if matches.is_present("idf-version") {
                let version = matches.value_of("idf-version").unwrap();
                let parameter = String::from("/IDFVERSION=") + version;
                arguments.push(parameter);
            }

            if matches.is_present("verbose") {
                arguments.push("/LOG=log.txt".to_string());
            }

            if matches.value_of("idf-path").is_some() {
                let dir = matches.value_of("idf-path").unwrap();
                let parameter = String::from("/IDFDIR=") + dir;
                arguments.push(parameter);
                let path_exists = Path::new(dir).exists();

                if matches.is_present("upgrade") {
                    if !path_exists {
                        println!("Unable to upgrade, path does not exist: {}", dir);
                        println!("Specify path to existing idf, or install new one without --upgrade parameter.");
                        process::exit(1);
                    }
                    arguments.push("/IDFUSEEXISTING=yes".to_string());
                } else {
                    if path_exists {
                        println!("Unable to install fresh version of IDF to existing directory: {}", dir);
                        println!("Options:");
                        println!("* specify --upgrade parameter to update existing installation");
                        println!("* specify --idf-path to directory which does not exit");
                        process::exit(1);
                    }
                }
            }

            let output = if cfg!(target_os = "windows") {
                println!("{} {:?}", get_installer(matches), arguments);
                std::process::Command::new(get_installer(matches))
                    .args(arguments)
                    .output()
                    .expect("failed to execute process")
            } else {
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg("echo hello")
                    .output()
                    .expect("failed to execute process")
            };
            let _data = output.stdout;
            if matches.is_present("verbose") {
                if cfg!(target_os = "windows") {
                    std::process::Command::new("notepad.exe")
                        .args(&["log.txt"])
                        .output()
                        .expect("failed to execute process")
                } else {
                    std::process::Command::new("sh")
                        .arg("-c")
                        .arg("echo hello")
                        .output()
                        .expect("failed to execute process")
                };
            }

            Ok(())
        })
}


pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_install_cmd())
        .add_cmd(get_reset_cmd())
        .into_cmd("idf")

        // Optionally specify a description
        .description("Maintain configuration of ESP-IDF installations.");

    return multi_cmd;
}
