
mod gui;
mod rust;
use idf_env_core;
use clap::{Arg, App, ArgMatches};


use crate::gui::webview::open_url;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


fn get_gui_runner(matches: &ArgMatches) -> std::result::Result<(), clap::Error> {
    let url = matches.value_of("url").unwrap();
    open_url(url);
    Ok(())
}


pub fn get_start_cmd<'a>() -> App<'a> {
    App::new("start")
        .about("Start GUI")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .takes_value(true)
                .default_value("https://espressif.github.io/idf-env/gui/assets/#/modify")
                .help("URL with web interface of the installer"),
        )
        //
        // .runner(|_args, matches| get_gui_runner(_args, matches)
        // )
}



pub fn get_gui_multi_cmd<'a>() -> App<'a> {
    App::new("gui")
        .about("Start GUI application to maintain the environemnt.")
        .subcommand(get_start_cmd())

}

async fn app() -> Result<()> {
    // open_url("http://localhost:8000");

    let matches = App::new("idf-env-gui")
        .version("1.2.20")
        .name("idf-env-gui")
        .author("Espressif Systems - https://www.espressif.com")
        .about("GUI Tool for maintaining ESP-IDF environment on computer.")
        .subcommand(get_gui_multi_cmd())
        //
        // .no_cmd(|_args, _matches| {
        //     open_url("https://espressif.github.io/idf-env/gui/assets/#/modify");
        //     Ok(())
        // })
        .get_matches();

    match matches.subcommand() {
        Some(("gui", gui_matches)) => {
            match gui_matches.subcommand() {
                Some(("start", start_matches)) => {
                    get_gui_runner(start_matches);
                }
                _ => {}
            }
        }
        _ => {
            open_url("https://espressif.github.io/idf-env/gui/assets/#/modify");
        }
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    app().await.unwrap();
}