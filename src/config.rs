extern crate json;

use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
use std::fs;
use std::path::Path;
use md5;
use std::env;


fn print_path(property_path: &std::string::String) {
    let path = Path::new(&property_path);
    let parent = path.parent().unwrap().to_str();
    print!("{}", parent.unwrap());
}

fn get_json_path() -> String {
    let idf_tools_path_env = "IDF_TOOLS_PATH";

    let idf_tools_path = env::var(idf_tools_path_env).unwrap_or_else(|e| {
        panic!("could not find {}: {}", idf_tools_path_env, e)
    });
    let idf_json_path = idf_tools_path + "/esp_idf.json";
    return idf_json_path;
}

fn get_idf_id(idf_path: String) -> String {
    let idf_path_with_slash = format!("{}", idf_path.replace("\\", "/"));
    let digest = md5::compute(idf_path_with_slash);
    return format!("esp-idf-{:x}", digest);
}

fn load_json() -> json::JsonValue {
    let content = fs::read_to_string(get_json_path())
        .expect("Failure");
    return json::parse(&content.to_string()).unwrap();
}

pub fn get_property(property_name: String) -> String {
    let parsed_json = load_json();
    return parsed_json[property_name].to_string();
}

fn print_property(property_name: String) {
    print_path(&get_property(property_name));
}

pub fn get_git_path() -> String {
    get_property("gitPath".to_string())
}

pub fn get_property_with_path(property_name: String, idf_path: String) -> String {
    let parsed_json = load_json();
    let idf_id = get_idf_id(idf_path);
    return parsed_json["idfInstalled"][idf_id][property_name].to_string();
}

fn print_property_with_path(property_name: String, idf_path: String) {
    print_path(&get_property_with_path(property_name, idf_path));
}

fn add_idf_config(idf_path: String, version: String, python_path: String) {
    let idf_id = get_idf_id(idf_path.clone());
    let _data = json::object! {
        version: version,
        python: python_path,
        path: idf_path
    };

    let mut parsed_json = load_json();
    parsed_json["idfInstalled"].insert(&idf_id, _data).unwrap();

    fs::write(get_json_path(), format!("{:#}", parsed_json)).unwrap();
}

pub fn get_cmd<'a>() -> Command<'a, str> {
    Command::new("get")
        .description("Retrieve configuration")
        .options(|app| {
            app.arg(
                Arg::with_name("property")
                    .short("p")
                    .long("property")
                    .help("Filter result for property name")
                    .takes_value(true)
            )
                .arg(
                    Arg::with_name("idf-path")
                        .short("i")
                        .long("idf-path")
                        .help("Path to ESP-IDF")
                        .takes_value(true),
                )
        })
        .runner(|_args, matches| {
            if matches.is_present("property") {
                let property_name = matches.value_of("property").unwrap().to_string();

                if matches.is_present("idf-path") {
                    let idf_path = matches.value_of("idf-path").unwrap().to_string();
                    print_property_with_path(property_name, idf_path);
                } else {
                    print_property(property_name);
                }
            } else {
                let content = load_json();
                println!("{:#}", &content);
            }
            Ok(())
        })
}


pub fn get_add_cmd<'a>() -> Command<'a, str> {
    Command::new("add")
        .description("Add configuration")
        .options(|app| {
            app.arg(
                Arg::with_name("python")
                    .short("p")
                    .long("python")
                    .help("Full path to Python binary")
                    .takes_value(true)
            )
                .arg(
                    Arg::with_name("idf-path")
                        .short("i")
                        .long("idf-path")
                        .help("Path to ESP-IDF")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("idf-version")
                        .short("x")
                        .long("idf-version")
                        .help("ESP-IDF version")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("git")
                        .short("g")
                        .long("git")
                        .help("Full path to Git binary")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .help("Custom name of ESP-IDF installation")
                        .takes_value(true)
                )
        })
        .runner(|_args, matches| {
            let python_path = matches.value_of("python").unwrap().to_string();
            let version = matches.value_of("idf-version").unwrap().to_string();
            let idf_path = matches.value_of("idf-path").unwrap().to_string();
            add_idf_config(idf_path, version, python_path);
            Ok(())
        })
}


pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_cmd())
        .add_cmd(get_add_cmd())
        .into_cmd("config")

        // Optionally specify a description
        .description("Maintain configuration of ESP-IDF installations.");

    return multi_cmd;
}
