use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
use git2::Repository;
use std::path::Path;
use std::io::Cursor;
use std::process;
use tokio::runtime::Handle;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use std::env;

use std::process::Stdio;
use std::io::{Write};
use std::io::Read;

use std::time::{Duration, Instant};
use dirs::home_dir;

use crate::config::{add_idf_config, get_git_path, get_tool_path, get_dist_path, get_python_env_path, update_property};
use crate::config::get_tools_path;
use crate::config::get_selected_idf_path;
use crate::package::prepare_package;
use crate::shell::run_command;

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

#[cfg(windows)]
fn get_idf_base_directory() -> String {
    "C:/esp".to_string()
}

#[cfg(unix)]
fn get_idf_base_directory() -> String {
    home_dir().unwrap().display().to_string() + "/esp"
}

#[cfg(windows)]
fn get_esp_idf_directory(idf_name:String) -> String {
    format!("{}/{}", get_idf_base_directory(), idf_name).replace("/", "\\")
}

#[cfg(unix)]
fn get_esp_idf_directory(idf_name:String) -> String {
    format!("{}/{}", get_idf_base_directory(), idf_name)
}

fn get_install_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let esp_idf = get_esp_idf_directory("esp-idf-master/".to_string());
    println!("ESP-IDF Path: {}", esp_idf);

    #[cfg(windows)]
    prepare_package("https://dl.espressif.com/dl/idf-git/idf-git-2.30.1-win64.zip".to_string(),
        get_dist_path("idf-git-2.30.1-win64.zip").as_str(),
        get_tool_path("idf-git/2.30.1".to_string())
    );
    #[cfg(windows)]
    prepare_package("https://dl.espressif.com/dl/idf-python/idf-python-3.8.7-embed-win64.zip".to_string(),
        get_dist_path("idf-python-3.8.7-embed-win64.zip").as_str(),
        get_tool_path("idf-python/3.8.7".to_string())
    );

    #[cfg(windows)]
    let git_path = get_tool_path("idf-git/2.30.1/cmd/git.exe".to_string());
    #[cfg(unix)]
    let git_path = "/usr/bin/git".to_string();

    update_property("gitPath".to_string(), git_path.clone());

    #[cfg(windows)]
    let python_path = get_tool_path("idf-python/3.8.7/python.exe".to_string());
    #[cfg(unix)]
    let python_path = "/usr/bin/python".to_string();

    let virtual_env_path = get_python_env_path("4.4".to_string(), "3.8".to_string());

    if !Path::new(&esp_idf).exists() {
        // let clone_command = format!("git clone --shallow-since=2020-01-01 --jobs 8 --recursive git@github.com:espressif/esp-idf.git ");
        let mut arguments: Vec<String> = [].to_vec();
        arguments.push("clone".to_string());
        arguments.push("--shallow-since=2020-01-01".to_string());
        arguments.push("--jobs".to_string());
        arguments.push("8".to_string());
        arguments.push("--recursive".to_string());
        arguments.push("https://github.com/espressif/esp-idf.git".to_string());
        // arguments.push("git@github.com:espressif/esp-idf.git".to_string());
        arguments.push(esp_idf.clone());
        println!("Cloning: {} {:?}", git_path, arguments);
        run_command(git_path, arguments, "".to_string());
    }

    if !Path::new(&virtual_env_path).exists() {
        println!("Creating virtual environment: {}", virtual_env_path);
        let mut arguments: Vec<String> = [].to_vec();
        arguments.push("-m".to_string());
        arguments.push("virtualenv".to_string());
        arguments.push(virtual_env_path.clone());
        run_command(python_path, arguments, "".to_string());
    }
    #[cfg(windows)]
    let python_path = format!("{}/Scripts/python.exe", virtual_env_path);
    #[cfg(unix)]
    let python_path = format!("{}/bin/python", virtual_env_path);

    let idf_tools = format!("{}/tools/idf_tools.py", esp_idf);

    let mut arguments: Vec<String> = [].to_vec();
    arguments.push(idf_tools.clone());
    arguments.push("install".to_string());
    run_command(python_path.clone(), arguments, "".to_string());

    let mut arguments: Vec<String> = [].to_vec();
    arguments.push(idf_tools);
    arguments.push("install-python-env".to_string());
    run_command(python_path.clone(), arguments, "".to_string());

    add_idf_config(esp_idf, "4.4".to_string(), python_path);
    Ok(())
}

fn get_install_inno_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
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
        .runner(|_args, matches|
            get_install_runner(_args, matches)
        )
}

#[cfg(unix)]
fn get_shell() -> String {
    "/bin/bash".to_string()
}

#[cfg(unix)]
fn get_initializer() -> String {
    format!("{}/export.sh", get_selected_idf_path())
}

#[cfg(unix)]
fn get_initializer_arguments() -> Vec<String> {
    let mut arguments: Vec<String> = [].to_vec();
    arguments.push("-c".to_string());
    arguments.push(". ./export.sh;cd examples/get-started/blink;idf.py fullclean; idf.py build".to_string());
    arguments
}

#[cfg(windows)]
fn get_shell() -> String {
    "powershell".to_string()
}

#[cfg(windows)]
fn get_initializer() -> String {
    format!("{}/Initialize-Idf.ps1", get_tools_path())
}

#[cfg(windows)]
fn get_initializer_arguments() -> Vec<String> {
    let mut arguments: Vec<String> = [].to_vec();
    arguments.push("-ExecutionPolicy".to_string());
    arguments.push("Bypass".to_string());
    arguments.push("-NoExit".to_string());
    arguments.push("-File".to_string());
    arguments.push(get_initializer());
    arguments
}

fn get_shell_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    println!("Starting process");
    // let root = Path::new("C:\\esp");
    // assert!(env::set_current_dir(&root).is_ok());
    // println!("Successfully changed working directory to {}!", root.display());


    let process = std::process::Command::new(get_shell())
        .args(get_initializer_arguments())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn().unwrap();

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read stdout: {}", why),
        Ok(_) => print!("{}", s),
    }

    Ok(())
}

pub fn get_shell_cmd<'a>() -> Command<'a, str> {
    Command::new("shell")
        .description("Start the companion")
        .options(|app| {
            app.arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .help("Name of communication port")
                    .takes_value(true)
            )
        })
        .runner(|_args, matches| get_shell_runner(_args, matches) )
}

#[cfg(unix)]
fn run_build(idf_path: &String, shell_initializer: &String) -> std::result::Result<(), clap::Error> {
    // println!("Starting process");
    let root = Path::new(&idf_path);
    assert!(env::set_current_dir(&root).is_ok());

    run_idf_command("cd examples/get-started/blink; idf.py fullclean; idf.py build".to_string());

    //println!("output = {:?}", output);
    Ok(())
}

fn run_idf_command(command: String) {
    run_command(get_shell(), get_initializer_arguments(), command);
}

#[cfg(windows)]
fn run_build(idf_path: &String, shell_initializer: &String) -> std::result::Result<(), clap::Error> {
    // println!("Starting process");
    let root = Path::new(&idf_path);
    assert!(env::set_current_dir(&root).is_ok());

    run_idf_command("cd examples/get-started/blink; idf.py fullclean; idf.py build\n".to_string());

    Ok(())
}

fn get_build_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let build_repetitions:i32 = matches.value_of("repeat").unwrap().to_string().parse().unwrap();
    let idf_path = matches.value_of("idf-path")
        .unwrap_or(&*get_selected_idf_path()).to_string();

    let initializer = get_initializer();
    println!("Number of CPU cores: {}", num_cpus::get());
    println!("ESP-IDF Shell Initializer: {}", initializer);
    println!("ESP-IDF Path: {}", idf_path);
    for build_number in 0..build_repetitions {
        let start = Instant::now();
        run_build(&idf_path, &initializer);
        let duration = start.elapsed();
        println!("Time elapsed in build: {:?}", duration);
    }
    Ok(())
}

fn change_submodules_mirror(mut repo: Repository, submodule_url: String) {
    let mut change_set: Vec<(String, String)> = Vec::new();
    for submodule in repo.submodules().unwrap() {
        let repo_name = submodule.name().unwrap().to_string();
        let original_url = submodule.url().unwrap();

        if !( original_url.starts_with("../../") ||
            original_url.starts_with("https://github.com")
        ) {
            println!("Submodule: {}, URL: {} - skip", repo_name, original_url);
            continue;
        }

        let mut old_repo = original_url.split('/').last().unwrap();

        // Correction of some names
        if old_repo.starts_with("unity") {
            old_repo = "Unity"
        } else if old_repo.starts_with("cexception") {
            old_repo = "CException"
        }

        let new_url = format!("{}{}", submodule_url, old_repo);

        change_set.push((repo_name, new_url));

    }

    for submodule in change_set {
        println!("Submodule: {}, new URL: {}", submodule.0, submodule.1);
        repo.submodule_set_url(&*submodule.0, &*submodule.1);
    }

}

fn get_mirror_switch_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let idf_path = matches.value_of("idf-path")
        .unwrap_or(&*get_selected_idf_path()).to_string();
    let url = matches.value_of("url")
        .unwrap().to_string();
    let submodule_url = matches.value_of("submodule-url")
        .unwrap().to_string();

    println!("Processing main repository...");
    match Repository::open(idf_path.clone()) {
        Ok(mut repo) => {
            //repo.find_remote("origin")?.url()
            if matches.is_present("url") {
                repo.remote_set_url("origin", url.as_str());
            }

            change_submodules_mirror(repo, submodule_url.clone());

        },
        Err(e) => {
            println!("failed to open: {}", e.to_string());
            std::process::exit(1);
        },
    };

    println!("Processing submodules...");
    match Repository::open(idf_path) {
        Ok(mut repo) => {
            //repo.find_remote("origin")?.url()
            if matches.is_present("url") {
                repo.remote_set_url("origin", url.as_str());
            }

            for mut submodule_repo_reference in repo.submodules().unwrap() {
                submodule_repo_reference.init(false);
                submodule_repo_reference.update(true, None);
                match submodule_repo_reference.open() {
                    Ok(mut sub_repo) => {
                        println!("Processing submodule: {:?}", sub_repo.workdir().unwrap());
                        change_submodules_mirror(sub_repo, submodule_url.clone());
                    },
                    Err(e) => {
                        println!("Unable to update submodule");
                    }
                }
            }

        },
        Err(e) => {
            println!("failed to open: {}", e.to_string());
            std::process::exit(1);
        },
    };

    Ok(())
}

pub fn get_build_cmd<'a>() -> Command<'a, str> {
    Command::new("build")
        .description("Start build process")
        .options(|app| {
            app.arg(
                Arg::with_name("repeat")
                    .short("r")
                    .long("repeat")
                    .help("Number of repetitions of the same command")
                    .takes_value(true)
                    .default_value("1")
            )
                .arg(
                    Arg::with_name("idf-path")
                        .short("p")
                        .long("idf-path")
                        .help("Path to ESP IDF source code repository")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("tools-path")
                        .short("t")
                        .long("tools-path")
                        .help("Path to Tools directory")
                        .takes_value(true)
                )
        })
        .runner(|_args, matches|
            get_build_runner(_args, matches)
        )
}

pub fn get_mirror_cmd<'a>() -> Command<'a, str> {
    Command::new("mirror")
        .description("Switch the URL of repository mirror")
        .options(|app| {
            app.arg(
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .help("Base URL of the main repo")
                    .takes_value(true)
            )
                .arg(
                    Arg::with_name("idf-path")
                        .short("p")
                        .long("idf-path")
                        .help("Path to ESP IDF source code repository")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("submodule-url")
                        .short("s")
                        .long("submodule-url")
                        .help("Base URL for submodule mirror")
                        .required(true)
                        .takes_value(true)
                )
        })
        .runner(|_args, matches|
            get_mirror_switch_runner(_args, matches)
        )
}


pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_build_cmd())
        .add_cmd(get_install_cmd())
        .add_cmd(get_mirror_cmd())
        .add_cmd(get_reset_cmd())
        .add_cmd(get_shell_cmd())
        .into_cmd("idf")

        // Optionally specify a description
        .description("Maintain configuration of ESP-IDF installations.");

    return multi_cmd;
}
