use crate::package::prepare_package;
use crate::shell::update_env_path;
use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
use dirs::home_dir;
use std::fs::remove_dir_all;
use std::path::Path;

struct ToitTools {
    jaguar_dist_file: String,
    jaguar_dist_url: String,
    jaguar_destination_dir: String,
}

fn build_toit_tools() -> ToitTools {
    let jaguar_dist_file = format!("jag_windows.zip");
    let jaguar_dist_url = format!(
        "https://github.com/toitlang/jaguar/releases/latest/download/{}",
        jaguar_dist_file
    );

    ToitTools {
        jaguar_dist_file,
        jaguar_dist_url,
        jaguar_destination_dir: format!(
            "{}/AppData/Local/Programs/jaguar",
            home_dir().unwrap().display().to_string()
        ),
    }
}

fn install_toit_tools(toit_tools: &ToitTools) {
    if Path::new(&toit_tools.jaguar_destination_dir.as_str()).exists() {
        println!(
            "Previous installation of Toit - Jaguar exist in: {}",
            toit_tools.jaguar_destination_dir
        );
        println!("Please, remove the directory before new installation.");
    } else {
        match prepare_package(
            toit_tools.jaguar_dist_url.to_string(),
            &toit_tools.jaguar_dist_file,
            toit_tools.jaguar_destination_dir.to_string(),
        ) {
            Ok(_) => {
                println!("Toit package ready");
            }
            Err(_e) => {
                println!("Unable to prepare the package.");
                return;
            }
        }
    }

    #[cfg(windows)]
    println!("PATH+=\";{}\"", &toit_tools.jaguar_destination_dir);
    #[cfg(unix)]
    println!(
        "export PATH=\"{}:$PATH\"",
        &toit_tools.jaguar_destination_dir
    );

    update_env_path(&toit_tools.jaguar_destination_dir);
}

fn uninstall_toit_tools(toit_tools: &ToitTools) {
    if Path::new(toit_tools.jaguar_destination_dir.as_str()).exists() {
        println!("Removing: {}", toit_tools.jaguar_destination_dir);
        match remove_dir_all(&toit_tools.jaguar_destination_dir) {
            Ok(_) => {
                println!("Ok");
            }
            Err(_e) => {
                println!("Unable to remove directory");
            }
        }
    }
}

fn get_default_toit_tools(_matches: &clap::ArgMatches<'_>) -> ToitTools {
    build_toit_tools()
}

fn get_install_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
    let toit_tools = get_default_toit_tools(&matches);
    if matches.is_present("jaguar") {
        install_toit_tools(&toit_tools);
    }
    Ok(())
}

fn get_reinstall_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
    let toit_tools = get_default_toit_tools(matches);
    if matches.is_present("jaguar") {
        uninstall_toit_tools(&toit_tools);
        install_toit_tools(&toit_tools);
    }
    Ok(())
}

fn get_uninstall_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
    let toit_tools = get_default_toit_tools(matches);
    if matches.is_present("jaguar") {
        uninstall_toit_tools(&toit_tools);
    }
    Ok(())
}

pub fn get_install_cmd<'a>() -> Command<'a, str> {
    Command::new("install")
        .description("Install Toit environment")
        .options(|app| app.arg(Arg::with_name("jaguar").short("j").long("jaguar")))
        .runner(|_args, matches| get_install_runner(_args, matches))
}

pub fn get_reinstall_cmd<'a>() -> Command<'a, str> {
    Command::new("reinstall")
        .description("Re-install Toit environment")
        .options(|app| app.arg(Arg::with_name("jaguar").short("j").long("jaguar")))
        .runner(|_args, matches| get_reinstall_runner(_args, matches))
}

pub fn get_uninstall_cmd<'a>() -> Command<'a, str> {
    Command::new("uninstall")
        .description("Uninstall Toit environment")
        .options(|app| app.arg(Arg::with_name("jaguar").short("j").long("jaguar")))
        .runner(|_args, matches| get_uninstall_runner(_args, matches))
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_install_cmd())
        .add_cmd(get_reinstall_cmd())
        .add_cmd(get_uninstall_cmd())
        .into_cmd("toit")
        // Optionally specify a description
        .description("Toit environment.");

    return multi_cmd;
}
