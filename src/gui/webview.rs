use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use web_view::*;

fn get_gui_runner(_args: &str, matches: &clap::ArgMatches<'_>)  -> std::result::Result<(), clap::Error> {
    // let app = include_str!("../../gui/index.html");
    let url = matches.value_of("url").unwrap();
    web_view::builder()
        .title("Espressif Environment Installer")
        .content(Content::Url(url))
        .size(800, 600)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
    Ok(())
}

pub fn get_gui_start_cmd<'a>() -> Command<'a, str> {
    Command::new("start")
        .description("Start GUI")

        .options(|app| {
            app.arg(
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .help("URL of interface")
                    .takes_value(true)
                    .default_value("https://espressif.com")
            )
        })
        .runner(|_args, matches|
            get_gui_runner(_args, matches)
        )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_gui_start_cmd())
        .into_cmd("gui")
        // Optionally specify a description
        .description("GUI");

    return multi_cmd;
}
