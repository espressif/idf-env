use clap::{Arg, App};

use web_view::*;
use serde::{Deserialize};
use serde_json;
use crate::rust::install_rust;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Log { text: String },
    AddTask { name: String },
    MarkTask { index: usize, done: bool },
    ClearDoneTasks,
}

pub fn open_url(url: &str) {
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
                },
            };
            Ok(())
        })
        .run()
        .unwrap();

        // .invoke_handler(|webview, arg| {
        //     use Cmd::*;
        //     match serde_json::from_str(arg).unwrap() {
        //         LoadComponentStatus { component_name } => {
        //             let eval_str = format!("UpdateComponent({},{:?});", component_name, "installed");
        //             webview.eval(&eval_str)?;
        //
        //         }
        //     }
            // match arg {
            //     "install" => {
            //         println!("Start installation...");
            //         install_rust();
            //     }
            //     "test_two" => {
            //         // Invoke a JavaScript function!
            //         // webview.eval(&format!("myFunction({}, {})", 123, 456))
            //     }
            //     _ => {
            //         println!("Operation not implemented: {}", arg)
            //     },
            // };
            // Ok(())
        // })
        // .run()
        // .unwrap();
}

fn get_gui_runner(_args: &str, matches: &clap::ArgMatches)  -> std::result::Result<(), clap::Error> {
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
                },
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

