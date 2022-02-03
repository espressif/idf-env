use std::env;
use clap::{Arg, App};

use dirs::home_dir;
use std::path::Path;
use std::fs::{create_dir_all, remove_dir_all};
use std::io::Read;
use std::process::Stdio;
use crate::config::get_tool_path;
use crate::package::{prepare_package, prepare_package_strip_prefix, prepare_single_binary};
use crate::shell::run_command;

struct ToitTools {
    jaguar_dist_file: String,
    jaguar_dist_url: String,
    jaguar_destination_dir: String
}


fn build_toit_tools() -> ToitTools {
    let jaguar_dist_file = format!("jag_windows.zip");
    let jaguar_dist_url = format!("https://github.com/toitlang/jaguar/releases/latest/download/{}", jaguar_dist_file);

    ToitTools {
        jaguar_dist_file,
        jaguar_dist_url,
        jaguar_destination_dir: format!("{}/AppData/Local/Programs/jaguar", home_dir().unwrap().display().to_string()),
    }
}

#[cfg(windows)]
fn set_env_variable(key:&str, value:String) {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions
    env.set_value(key, &value).unwrap();
}

#[cfg(windows)]
fn update_env_path(value: &str) {
    let path_key = "PATH";
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey("Environment").unwrap();
    let env_path:String = env.get_value(path_key).unwrap();
    if !env_path.contains(&value) {
        let updated_env_path = format!("{};{}", env_path, value);
        set_env_variable(path_key, updated_env_path);
    }
}

#[cfg(unix)]
fn update_env_path(value: &str) {
}

#[cfg(unix)]
fn set_env_variable(key:&str, value:&str) {

}

pub fn is_toit_installed() -> bool {
    let toit_tools = build_toit_tools();
    Path::new(&toit_tools.jaguar_destination_dir.as_str()).exists()
}

fn install_toit_tools(toit_tools:&ToitTools) {

    if Path::new(&toit_tools.jaguar_destination_dir.as_str()).exists() {
        println!("Previous installation of Toit - Jaguar exist in: {}", toit_tools.jaguar_destination_dir);
        println!("Please, remove the directory before new installation.");
    } else {
        prepare_package(toit_tools.jaguar_dist_url.to_string(),
                                     &toit_tools.jaguar_dist_file,
                                     toit_tools.jaguar_destination_dir.to_string());
    }

    #[cfg(windows)]
    println!("PATH+=\";{}\"", &toit_tools.jaguar_destination_dir);
    #[cfg(unix)]
    println!("export PATH=\"{}:$PATH\"", &toit_tools.jaguar_destination_dir);

    update_env_path(&toit_tools.jaguar_destination_dir);

}

fn uninstall_toit_tools(toit_tools:&ToitTools) {
    if Path::new(toit_tools.jaguar_destination_dir.as_str()).exists() {
        println!("Removing: {}", toit_tools.jaguar_destination_dir);
        remove_dir_all(&toit_tools.jaguar_destination_dir);
    }
}

fn get_default_toit_tools(matches: &clap::ArgMatches) -> ToitTools {
    build_toit_tools()
}

fn get_install_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    let toit_tools = get_default_toit_tools(&matches);
    if matches.is_present("jaguar") {
        install_toit_tools(&toit_tools);
    }
    Ok(())
}

fn get_reinstall_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    let toit_tools = get_default_toit_tools(matches);
    if matches.is_present("jaguar") {
        uninstall_toit_tools(&toit_tools);
        install_toit_tools(&toit_tools);
    }
    Ok(())
}

fn get_uninstall_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    let toit_tools = get_default_toit_tools(matches);
    if matches.is_present("jaguar") {
        uninstall_toit_tools(&toit_tools);
    }
    Ok(())
}

pub fn get_install_cmd<'a>() -> App<'a> {
    App::new("install")
        .about("Install Toit environment")
        .arg(
        Arg::new("jaguar")
            .short('j')
            .long("jaguar")
        )
        // .runner(|_args, matches|
        //     get_install_runner(_args, matches)
        // )
}

pub fn get_reinstall_cmd<'a>() -> App<'a> {
    App::new("reinstall")
        .about("Re-install Toit environment")
        .arg(
            Arg::new("jaguar")
                .short('j')
                .long("jaguar")
        )
        // .runner(|_args, matches|
        //     get_reinstall_runner(_args, matches)
        // )
}

pub fn get_uninstall_cmd<'a>() -> App<'a> {
    App::new("uninstall")
        .about("Uninstall Toit environment")
        .arg(
            Arg::new("jaguar")
                .short('j')
                .long("jaguar")
        )
        // .runner(|_args, matches|
        //     get_uninstall_runner(_args, matches)
        // )
}

pub fn get_multi_cmd<'a>() -> App<'a> {
    App::new("toit")
        .about("Toit environment.")
        .subcommand(get_install_cmd())
        .subcommand(get_reinstall_cmd())
        .subcommand(get_uninstall_cmd())
}
