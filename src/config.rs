// extern crate json;
//
// use clap::{Arg, ArgMatches};
// use clap_nested::{Command, Commander, MultiCommand};
// use std::fs;
// use std::path::Path;
// use md5;
use std::env;
use dirs::home_dir;
// use json::JsonValue;
use crate::shell::run_command;

pub fn get_tools_path() -> String {
    env::var("IDF_TOOLS_PATH").unwrap_or_else(|_e|
        home_dir().unwrap().display().to_string() + "/.espressif"
    )
}

pub fn get_tool_path(tool_name:String) -> String {
    let tools_path = get_tools_path();
    format!("{}/tools/{}", tools_path, tool_name)
}
//
// pub fn get_dist_path(tool_name:&str) -> String {
//     let tools_path = get_tools_path();
//     format!("{}/dist/{}", tools_path, tool_name)
// }
//
// pub fn get_python_env_path(idf_version: String, python_version: String) -> String {
//     let tools_path = get_tools_path();
//     format!("{}/python_env/idf{}_py{}_env", tools_path, idf_version, python_version)
// }
//
// pub fn get_selected_idf_path() -> String {
//     let selected_idf_id = get_property("idfSelectedId".to_string());
//     get_property_with_idf_id("path".to_string(), selected_idf_id)
// }
//
fn get_json_path() -> String {
    let idf_json_path = format!("{}/esp_idf.json", get_tools_path());
    return idf_json_path;
}
//
// pub fn get_idf_id(idf_path: &str) -> String {
//     let idf_path_with_slash = format!("{}", idf_path.replace("\\", "/"));
//     let digest = md5::compute(idf_path_with_slash);
//     return format!("esp-idf-{:x}", digest);
// }
//
// fn bootstrap_json(_json_path: String, tools_path: String) {
//     let template = json::object!{
//         "$schema": "http://json-schema.org/schema#",
//         "$id": "http://dl.espressif.com/dl/schemas/esp_idf",
//         "_comment": "Configuration file for ESP-IDF Eclipse plugin.",
//         "_warning": "Use / or \\ when specifying path. Single backslash is not allowed by JSON format.",
//         "gitPath": "",
//         "idfToolsPath": tools_path,
//         "idfSelectedId": "",
//         "idfInstalled": json::JsonValue::new_object()
//     };
//     fs::write(get_json_path(), template.to_string()).unwrap();
// }
//
// fn load_json() -> json::JsonValue {
//     let json_path = get_json_path();
//     if !Path::new(&json_path).exists() {
//         println!("Configuration file not found, creating new one: {}", json_path);
//         bootstrap_json(json_path.clone(), get_tools_path());
//     }
//
//     let content = fs::read_to_string(json_path)
//         .expect("Failure");
//     return json::parse(&content.to_string()).unwrap();
// }
//
// pub fn get_property(property_name: String) -> String {
//     let parsed_json = load_json();
//     return parsed_json[property_name].to_string();
// }
//
// fn print_property(property_name: String) {
//     print!("{}", &get_property(property_name));
// }
//
// pub fn get_git_path() -> String {
//     get_property("gitPath".to_string())
// }
//
// pub fn get_property_with_idf_id(property_name: String, idf_id: String) -> String {
//     let parsed_json = load_json();
//     return parsed_json["idfInstalled"][idf_id][property_name].to_string();
// }
//
//
// pub fn get_property_with_path(property_name: String, idf_path: String) -> String {
//     let parsed_json = load_json();
//     let idf_id = get_idf_id(&idf_path);
//     return parsed_json["idfInstalled"][idf_id][property_name].to_string();
// }
//
// fn print_property_with_path(property_name: String, idf_path: String) {
//     print!("{}", get_property_with_path(property_name, idf_path));
// }
//
// fn print_property_with_id(property_name: String, idf_id: String) {
//     print!("{}", get_property_with_idf_id(property_name, idf_id));
// }
//
// pub fn update_property(property_name: String, property_value: String) {
//     let mut parsed_json = load_json();
//     parsed_json[property_name] = JsonValue::String(property_value);
//     fs::write(get_json_path(), format!("{:#}", parsed_json)).unwrap();
// }
//
// pub fn add_idf_config(idf_path: String, version: String, python_path: String) {
//     let idf_id = get_idf_id(&idf_path);
//     let _data = json::object! {
//         version: version,
//         python: python_path,
//         path: idf_path
//     };
//
//     let mut parsed_json = load_json();
//     parsed_json["idfInstalled"].insert(&idf_id, _data).unwrap();
//     parsed_json["idfSelectedId"] = JsonValue::String(idf_id);
//
//     fs::write(get_json_path(), format!("{:#}", parsed_json)).unwrap();
// }
//
// pub fn get_cmd<'a>() -> Command<'a, str> {
//     Command::new("get")
//         .description("Retrieve configuration")
//         .options(|app| {
//             app.arg(
//                 Arg::with_name("property")
//                     .short("p")
//                     .long("property")
//                     .help("Filter result for property name")
//                     .takes_value(true)
//             )
//                 .arg(
//                     Arg::with_name("idf-path")
//                         .short("i")
//                         .long("idf-path")
//                         .help("Path to ESP-IDF")
//                         .takes_value(true),
//                 )
//                 .arg(
//                     Arg::with_name("idf-id")
//                         .short("j")
//                         .long("idf-id")
//                         .help("ESP-IDF installation ID")
//                         .takes_value(true),
//                 )
//         })
//         .runner(|_args, matches| {
//             if matches.is_present("property") {
//                 let property_name = matches.value_of("property").unwrap().to_string();
//
//                 if matches.is_present("idf-id") {
//                     let idf_id = matches.value_of("idf-id").unwrap().to_string();
//                     print_property_with_id(property_name, idf_id);
//                 } else if matches.is_present("idf-path") {
//                     let idf_path = matches.value_of("idf-path").unwrap().to_string();
//                     print_property_with_path(property_name, idf_path);
//                 } else {
//                     print_property(property_name);
//                 }
//             } else {
//                 let content = load_json();
//                 println!("{:#}", &content);
//             }
//             Ok(())
//         })
// }

//
// pub fn get_add_cmd<'a>() -> Command<'a, str> {
//     Command::new("add")
//         .description("Add configuration")
//         .options(|app| {
//             app.arg(
//                 Arg::with_name("python")
//                     .short("p")
//                     .long("python")
//                     .help("Full path to Python binary")
//                     .takes_value(true)
//             )
//                 .arg(
//                     Arg::with_name("idf-path")
//                         .short("i")
//                         .long("idf-path")
//                         .help("Path to ESP-IDF")
//                         .takes_value(true),
//                 )
//                 .arg(
//                     Arg::with_name("idf-version")
//                         .short("x")
//                         .long("idf-version")
//                         .help("ESP-IDF version")
//                         .takes_value(true)
//                 )
//                 .arg(
//                     Arg::with_name("git")
//                         .short("g")
//                         .long("git")
//                         .help("Full path to Git binary")
//                         .takes_value(true)
//                 )
//                 .arg(
//                     Arg::with_name("name")
//                         .short("n")
//                         .long("name")
//                         .help("Custom name of ESP-IDF installation")
//                         .takes_value(true)
//                 )
//         })
//         .runner(|_args, matches| {
//             let python_path = matches.value_of("python").unwrap().to_string();
//             let version = matches.value_of("idf-version").unwrap().to_string();
//             let idf_path = matches.value_of("idf-path").unwrap().to_string();
//             add_idf_config(idf_path, version, python_path);
//             Ok(())
//         })
// }
//
// fn get_set_runner(_args: &str, matches: &ArgMatches) -> std::result::Result<(), clap::Error> {
//     let git_path = matches.value_of("git").unwrap().to_string();
//     update_property("gitPath".to_string(), git_path);
//     Ok(())
// }
//
// pub fn get_set_cmd<'a>() -> Command<'a, str> {
//     Command::new("set")
//         .description("set configuration")
//         .options(|app| {
//                 app.arg(
//                     Arg::with_name("git")
//                         .short("g")
//                         .long("git")
//                         .help("Full path to Git binary")
//                         .takes_value(true)
//                 )
//         })
//         .runner(|_args, matches|
//             get_set_runner(_args, matches)
//         )
// }
//
//


use clap::{Parser, Subcommand};
use miette::Result;


#[derive(Debug, Parser)]
pub struct Add {
    // Full path to Python binary
    #[arg(short = 'p', long)]
    pub python: String,

    // Path to ESP-IDF
    #[arg(short = 'i', long)]
    pub idf_path: String,

    // ESP-IDF version
    #[arg(short = 'x', long)]
    pub idf_version: String,

    // Full path to Git binary
    #[arg(short = 'g', long)]
    pub git: String,

    // Custom name of ESP-IDF installation
    #[arg(short = 'n', long)]
    pub name: String,
}

#[derive(Debug, Parser)]
pub struct Edit {

    /// Name of the property to get.
    #[arg(short = 'x', long)]
    pub idf_path: String,
}

#[derive(Debug, Parser)]
pub struct Get {
    /// Verbosity level of the logs.
    #[arg(short = 'l', long, default_value = "info", value_parser = ["debug", "info", "warn", "error"])]
    pub log_level: String,

    /// Name of the property to get.
    #[arg(short = 'p', long)]
    pub property: String,

    /// Name of the property to get.
    #[arg(short = 'i', long)]
    pub idf_id: String,

    /// Name of the property to get.
    #[arg(short = 'x', long)]
    pub idf_path: String,
}

#[derive(Debug, Parser)]
pub struct Set {
    #[arg(short = 'g', long)]
    pub git: String,
}

#[derive(PartialEq, Debug, Subcommand)]
enum ConfigCommands {
    Add,
    Edit,
    Get,
    Set
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ConfigOpts {
    /// Verbosity level of the logs.
    #[arg(short = 'l', long, default_value = "info", value_parser = ["debug", "info", "warn", "error"])]
    pub log_level: String,
    #[command(subcommand)]
    subcommand: Option<ConfigCommands>,
    // Shell to generate completions for.
    // pub shell: Shell,
}


// impl ConfigOpts {
//     pub fn get_subcommand(&self) -> ConfigCommands {
//         // self.subcommand.clone().unwrap()
//     }
// }


// Updates Xtensa Rust toolchain.
pub async fn command_config(args: ConfigOpts) -> Result<()> {

    match args {
        ConfigOpts { subcommand: Some(ConfigCommands::Add), .. } => {
            println!("Add");
        }
        ConfigOpts { subcommand: Some(ConfigCommands::Edit), .. } => {
            println!("Edit");
            command_edit().await;
        }
        ConfigOpts { subcommand: Some(ConfigCommands::Get), .. } => {
            println!("Get");
            command_get(p).await;
        }
        ConfigOpts { subcommand: Some(ConfigCommands::Set), .. } => {
            println!("Set");
        }
        _ => {
            println!("None");
        }
    }

    Ok(())
}

// Get property from configuration file.
pub async fn command_get(args: Get) -> Result<()> {

    let content = get_json_content();
    println!("{:#}", content);

    Ok(())
}

// Get name of editor for specific platform.
fn get_editor() -> String {
    if cfg!(target_os = "windows") {
        "notepad".to_string()
    } else if cfg!(target_os = "macos") {
        "open".to_string()
    } else {
        "nano".to_string()
    }
}

// Edit configuration file.
pub async fn command_edit() -> Result<()>
{
    // .description("Open configuration file in editor")
    let mut arguments: Vec<String> = [].to_vec();

    #[cfg(macos)]
    arguments.push("-a".to_string());
    #[cfg(macos)]
    arguments.push("TextEdit".to_string());

    arguments.push(get_json_path());

    match run_command(get_editor(), arguments, "".to_string()) {
        Ok(_) => { println!("Ok"); },
        Err(_e) => { println!("Failed");}
    }

    Ok(())
}
