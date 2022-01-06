
mod gui;
mod rust;
use idf_env_core;
use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use crate::gui::webview::open_url;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


fn get_gui_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let url = matches.value_of("url").unwrap();
    open_url(url);
    Ok(())
}


pub fn get_start_cmd<'a>() -> Command<'a, str> {
    Command::new("start")
        .description("Start GUI")
        .options(|app| {
            app.arg(
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .takes_value(true)
                    .default_value("https://espressif.github.io/idf-env/gui/assets/#/modify")
                    .help("URL with web interface of the installer"),
            )
        })
        .runner(|_args, matches| get_gui_runner(_args, matches)
        )
}



pub fn get_gui_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_start_cmd())
        .into_cmd("gui")

        // Optionally specify a description
        .description("Maintain ESP-IDF environment");

    return multi_cmd;
}

async fn app() -> Result<()> {
    // open_url("http://localhost:8000");

    Commander::new()
        .options(|app| {
            app.version("1.2.20")
                .name("idf-env-gui")
                .author("Espressif Systems - https://www.espressif.com")
                .about("GUI Tool for maintaining ESP-IDF environment on computer.")

        })
        .args(|_args, matches| matches.value_of("environment").unwrap_or("dev"))
        .add_cmd(get_gui_multi_cmd())
        .no_cmd(|_args, _matches| {
            open_url("https://espressif.github.io/idf-env/gui/assets/#/modify");
            Ok(())
        })
        .run();
    Ok(())
}


#[tokio::main]
async fn main() {
    app().await.unwrap();
}