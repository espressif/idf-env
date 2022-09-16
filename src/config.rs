extern crate json;

use crate::emoji;
use crate::shell::run_command;
use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
use dirs::home_dir;
use espflash::Chip;
use json::JsonValue;
use std::env;
use std::fs;
use std::path::Path;

pub fn get_home_dir() -> String {
    home_dir().unwrap().display().to_string()
}

pub fn get_tools_path() -> String {
    env::var("IDF_TOOLS_PATH").unwrap_or_else(|_e| get_home_dir() + "/.espressif")
}

pub fn get_cargo_home() -> String {
    format!("{}/.cargo", get_home_dir())
}

pub fn get_tool_path(tool_name: &str) -> String {
    let tools_path = get_tools_path();
    format!("{}/tools/{}", tools_path, tool_name)
}

pub fn get_dist_path(tool_name: &str) -> String {
    let tools_path = get_tools_path();
    format!("{}/dist/{}", tools_path, tool_name)
}

pub fn get_python_env_path(idf_version: &str, python_version: &str) -> String {
    let tools_path = get_tools_path();
    format!(
        "{}/python_env/idf{}_py{}_env",
        tools_path, idf_version, python_version
    )
}

pub fn get_esp_idf_directory(idf_version: &str) -> String {
    let parsed_version: String = idf_version
        .chars()
        .map(|x| match x {
            '/' => '-',
            _ => x,
        })
        .collect();
    format!("{}/frameworks/esp-idf-{}", get_tools_path(), parsed_version)
}

pub fn get_selected_idf_path() -> String {
    let selected_idf_id = get_property("idfSelectedId");
    get_property_with_idf_id("path", &selected_idf_id)
}

fn get_json_path() -> String {
    let idf_json_path = format!("{}/esp_idf.json", get_tools_path());
    idf_json_path
}

pub fn get_idf_id(idf_path: &str) -> String {
    let idf_path_with_slash = idf_path.replace('\\', "/");
    let digest = md5::compute(idf_path_with_slash);
    format!("esp-idf-{:x}", digest)
}

fn bootstrap_json(json_path: &str, tools_path: &str) -> std::result::Result<(), clap::Error> {
    if !Path::new(&get_json_path()).exists() {
        println!("{} Creating tools.json file: {}", emoji::WRENCH, json_path);
        if let Err(_e) = fs::create_dir_all(&tools_path) {
            return Err(clap::Error::with_description(
                format!("{} File tools.json creation failed", emoji::ERROR).as_str(),
                clap::ErrorKind::InvalidValue,
            ));
        }
    }
    let template = json::object! {
        "$schema": "http://json-schema.org/schema#",
        "$id": "http://dl.espressif.com/dl/schemas/esp_idf",
        "_comment": "Configuration file for ESP-IDF Eclipse plugin.",
        "_warning": "Use / or \\ when specifying path. Single backslash is not allowed by JSON format.",
        "gitPath": "",
        "idfToolsPath": tools_path,
        "idfSelectedId": "",
        "idfInstalled": json::JsonValue::new_object()
    };
    fs::write(get_json_path(), template.to_string()).unwrap();
    Ok(())
}

fn load_json() -> json::JsonValue {
    let json_path = get_json_path();
    if !Path::new(&json_path).exists() {
        println!(
            "{} Configuration file not found, creating new one: {}",
            emoji::WARN,
            json_path
        );
        bootstrap_json(&json_path, &get_tools_path())
            .unwrap_or_else(|e| panic!("{} {}", emoji::ERROR, e));
    }

    let content = fs::read_to_string(json_path).expect("Failure");
    json::parse(&content).unwrap()
}

pub fn get_property(property_name: &str) -> String {
    let parsed_json = load_json();
    parsed_json[property_name].to_string()
}

fn print_property(property_name: &str) {
    print!("{}", &get_property(property_name));
}

pub fn get_git_path() -> String {
    get_property("gitPath")
}

pub fn get_property_with_idf_id(property_name: &str, idf_id: &str) -> String {
    let parsed_json = load_json();
    parsed_json["idfInstalled"][idf_id][property_name].to_string()
}

pub fn get_property_with_path(property_name: &str, idf_path: &str) -> String {
    let parsed_json = load_json();
    let idf_id = get_idf_id(idf_path);
    parsed_json["idfInstalled"][idf_id][property_name].to_string()
}

fn print_property_with_path(property_name: &str, idf_path: &str) {
    print!("{}", get_property_with_path(property_name, idf_path));
}

fn print_property_with_id(property_name: &str, idf_id: &str) {
    print!("{}", get_property_with_idf_id(property_name, idf_id));
}

pub fn update_property(property_name: &str, property_value: &str) {
    let mut parsed_json = load_json();
    parsed_json[property_name] = JsonValue::String(property_value.to_string());
    fs::write(get_json_path(), format!("{:#}", parsed_json)).unwrap();
}

pub fn add_idf_config(idf_path: &str, version: &str, python_path: &str) {
    let idf_id = get_idf_id(idf_path);
    let _data = json::object! {
        version: version,
        python: python_path,
        path: idf_path
    };

    let mut parsed_json = load_json();
    parsed_json["idfInstalled"].insert(&idf_id, _data).unwrap();
    parsed_json["idfSelectedId"] = JsonValue::String(idf_id);

    fs::write(get_json_path(), format!("{:#}", parsed_json)).unwrap();
}

pub fn parse_targets(build_target: &str) -> Result<Vec<Chip>, String> {
    // println!("Parsing targets: {}", build_target);
    let mut chips: Vec<Chip> = Vec::new();
    if build_target.contains("all") {
        chips.push(Chip::Esp32);
        chips.push(Chip::Esp32s2);
        chips.push(Chip::Esp32s3);
        chips.push(Chip::Esp32c3);
        return Ok(chips);
    }
    let targets: Vec<&str> = if build_target.contains(' ') || build_target.contains(',') {
        build_target.split([',', ' ']).collect()
    } else {
        vec![build_target]
    };
    for target in targets {
        match target {
            "esp32" => chips.push(Chip::Esp32),
            "esp32s2" => chips.push(Chip::Esp32s2),
            "esp32s3" => chips.push(Chip::Esp32s3),
            "esp32c3" => chips.push(Chip::Esp32c3),
            _ => {
                return Err(format!("Unknown target: {}", target));
            }
        };
    }

    Ok(chips)
}

pub fn parse_idf_targets(build_target: Vec<Chip>) -> Result<String, String> {
    let mut espidf_targets: String = String::new();
    for chip in build_target {
        if espidf_targets.is_empty() {
            espidf_targets = espidf_targets + &chip.to_string().to_lowercase().replace('-', "");
        } else {
            espidf_targets =
                espidf_targets + "," + &chip.to_string().to_lowercase().replace('-', "");
        }
    }
    Ok(espidf_targets)
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
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("idf-path")
                    .short("i")
                    .long("idf-path")
                    .help("Path to ESP-IDF")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("idf-id")
                    .short("j")
                    .long("idf-id")
                    .help("ESP-IDF installation ID")
                    .takes_value(true),
            )
        })
        .runner(|_args, matches| {
            if matches.is_present("property") {
                let property_name = matches.value_of("property").unwrap().to_string();

                if matches.is_present("idf-id") {
                    let idf_id = matches.value_of("idf-id").unwrap().to_string();
                    print_property_with_id(&property_name, &idf_id);
                } else if matches.is_present("idf-path") {
                    let idf_path = matches.value_of("idf-path").unwrap().to_string();
                    print_property_with_path(&property_name, &idf_path);
                } else {
                    print_property(&property_name);
                }
            } else {
                let content = load_json();
                println!("{:#}", &content);
            }
            Ok(())
        })
}

fn open_idf_config() {
    let mut arguments: Vec<String> = [].to_vec();
    arguments.push(get_json_path());
    match run_command("notepad", arguments, "".to_string()) {
        Ok(_) => {
            println!("Ok");
        }
        Err(_e) => {
            println!("Failed");
        }
    }
}

pub fn get_edit_cmd<'a>() -> Command<'a, str> {
    Command::new("edit")
        .description("Open configuration file in editor")
        .runner(|_args, _matches| {
            open_idf_config();
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
                    .takes_value(true),
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
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("git")
                    .short("g")
                    .long("git")
                    .help("Full path to Git binary")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .help("Custom name of ESP-IDF installation")
                    .takes_value(true),
            )
        })
        .runner(|_args, matches| {
            let python_path = matches.value_of("python").unwrap();
            let version = matches.value_of("idf-version").unwrap();
            let idf_path = matches.value_of("idf-path").unwrap();
            add_idf_config(idf_path, version, python_path);
            Ok(())
        })
}

fn get_set_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
    let git_path = matches.value_of("git").unwrap();
    update_property("gitPath", git_path);
    Ok(())
}

pub fn get_set_cmd<'a>() -> Command<'a, str> {
    Command::new("set")
        .description("set configuration")
        .options(|app| {
            app.arg(
                Arg::with_name("git")
                    .short("g")
                    .long("git")
                    .help("Full path to Git binary")
                    .takes_value(true),
            )
        })
        .runner(get_set_runner)
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_cmd())
        .add_cmd(get_edit_cmd())
        .add_cmd(get_add_cmd())
        .add_cmd(get_set_cmd())
        .into_cmd("config")
        // Optionally specify a description
        .description("Maintain configuration of ESP-IDF installations.");

    multi_cmd
}

#[cfg(test)]
mod tests {
    use crate::config::get_esp_idf_directory;
    use crate::config::get_tools_path;
    use crate::config::parse_idf_targets;
    // TODO: Refactor test into two subtests
    // #[test]
    // fn test_parse_idf_targets() {
    //     // assert_eq!(parse_idf_targets(""), "");
    //     assert_eq!(parse_idf_targets("esp32"), Ok("esp32".to_string()));
    //     assert_eq!(
    //         parse_idf_targets("esp32 esp32s2"),
    //         Ok("esp32,esp32s2".to_string())
    //     );
    //     assert_eq!(
    //         parse_idf_targets("esp32 esp32s2,esp32s3,esp32c3"),
    //         Ok("esp32,esp32s2,esp32s3,esp32c3".to_string())
    //     );
    //     assert_eq!(
    //         parse_idf_targets("all"),
    //         Ok("esp32,esp32s2,esp32s3,esp32c3".to_string())
    //     );
    // }
    #[test]
    fn test_get_esp_idf_directory() {
        assert_eq!(
            get_esp_idf_directory("release/v4.4"),
            format!("{}/frameworks/esp-idf-release-v4.4", get_tools_path())
        );
        assert_eq!(
            get_esp_idf_directory("v4.4.2"),
            format!("{}/frameworks/esp-idf-v4.4.2", get_tools_path())
        );
        assert_eq!(
            get_esp_idf_directory("master"),
            format!("{}/frameworks/esp-idf-master", get_tools_path())
        );
    }
}
