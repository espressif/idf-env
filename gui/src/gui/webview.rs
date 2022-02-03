use clap::{Arg, App};

use web_view::*;
use serde::{Deserialize};
use serde_json;
use crate::rust::install_rust;
use crate::idf_env_core::rust::{is_rustup_installed, is_rust_toolchain_installed, is_llvm_installed, install_rust_stable};

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Log { text: String },
    AddTask { name: String },
    MarkTask { index: usize, done: bool },
    ClearDoneTasks,
    GetComponentStatus { name: String },
    SetComponentDesiredState { name: String, state: String }
}

pub fn open_url(url: &str) {
    let webview = web_view::builder()
        .title("Espressif Environment Installer")
        // t= to avoid caching problems
        .content(Content::Url(format!("{}?t={}", url, 1)))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            use Cmd::*;
            match serde_json::from_str(arg).unwrap() {
                GetComponentStatus { name } => {
                    match name.as_str() {
                        "rustup" => {
                            let rustup_state = if is_rustup_installed() { "installed" } else { "not installed" };
                            let eval_str = format!("UpdateComponent('{}',{:?});", name, rustup_state);
                            println!("Load component {}", name);
                            println!("Eval: {}", eval_str);
                            webview.eval(&eval_str)?;
                        }
                        "rust-toolchain-nightly" => {
                            let rust_toolchain_state = if is_rust_toolchain_installed("nightly") { "installed" } else { "not installed" };
                            let eval_str = format!("UpdateComponent('{}',{:?});", name, rust_toolchain_state);
                            println!("Load component {}", name);
                            println!("Eval: {}", eval_str);
                            webview.eval(&eval_str)?;
                        }
                        "rust-toolchain-stable" => {
                            let rust_toolchain_state = if is_rust_toolchain_installed("stable") { "installed" } else { "not installed" };
                            let eval_str = format!("UpdateComponent('{}',{:?});", name, rust_toolchain_state);
                            println!("Load component {}", name);
                            println!("Eval: {}", eval_str);
                            webview.eval(&eval_str)?;
                        }
                        "rust-toolchain-xtensa" => {
                            let rust_toolchain_state = if is_rust_toolchain_installed("esp") { "installed" } else { "not installed" };
                            let eval_str = format!("UpdateComponent('{}',{:?});", name, rust_toolchain_state);
                            println!("Load component {}", name);
                            println!("Eval: {}", eval_str);
                            webview.eval(&eval_str)?;
                        }
                        "llvm-xtensa" => {
                            let toolchain_state = if is_llvm_installed() { "installed" } else { "not installed" };
                            let eval_str = format!("UpdateComponent('{}',{:?});", name, toolchain_state);
                            println!("Load component {}", name);
                            println!("Eval: {}", eval_str);
                            webview.eval(&eval_str)?;
                        }

                        _ => {
                            println!("Unknown component {}", name);
                        }
                    }
                }

                SetComponentDesiredState { name, state } => {
                    match name.as_str() {
                        "rustup" => {
                            match state.as_str() {
                                "installed" => {
                                    if !is_rustup_installed() {
                                        install_rust_stable();
                                    }
                                }
                                "uninstalled" => {
                                    if !is_rustup_installed() {

                                    }
                                }
                                _ => {
                                    println!("Unknown state {} of component {}", state, name);
                                }
                            }
                        }

                        _ => {
                            println!("Unknown component {}", name);
                        }

                    }
                }

                _ => {
                    println!("Unknown command");
                }
            };
            Ok(())
        })
        .run()
        .unwrap();
}

fn get_gui_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    // let app = include_str!("../../gui/index.html");
    let url = matches.value_of("url").unwrap();
    web_view::builder()
        .title("Espressif Environment Installer")
        .content(Content::Url(url))
        .size(800, 600)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "install" => {
                    println!("Start installation...")
                }
                "test_two" => {
                    // Invoke a JavaScript function!
                    // webview.eval(&format!("myFunction({}, {})", 123, 456))
                }
                _ => {
                    println!("Operation not implemented: {}", arg)
                }
            };
            Ok(())
        })
        .run()
        .unwrap();
    Ok(())
}

pub fn get_gui_start_cmd<'a>() -> App<'a> {
    App::new("start")
        .about("Start GUI")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("URL of interface")
                .takes_value(true)
                .default_value("https://espressif.com")
        )
    // .runner(|_args, matches|
    //     get_gui_runner(_args, matches)
    // )
}

pub fn get_multi_cmd<'a>() -> App<'a> {
    App::new("gui")
        .about("GUI")
        .subcommand(get_gui_start_cmd())
}

