// On Windows
#![windows_subsystem = "windows"]

use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use crate::config::get_git_path;

use druid::widget::{Flex, Label, TextBox, Button};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc, Widget, Env};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

#[derive(Clone, Data, Lens)]
struct HelloState {
    git: String,
}

pub fn get_cmd<'a>() -> Command<'a, str> {
    Command::new("start")
        .description("Start GUI")
        .options(|app| {
            app.arg(
                Arg::with_name("property")
                    .short("p")
                    .long("property")
                    .help("Filter result for property name")
                    .takes_value(true)
            )

        })
        .runner(|_args, matches| {


            let main_window = WindowDesc::new(build_root_widget())
                .title("idf-env")
                .window_size((800.0, 600.0));

            // create the initial app state
            let initial_state: HelloState = HelloState {
                git: get_git_path(),
            };

            // start the application. Here we pass in the application state.
            AppLauncher::with_window(main_window)
                .launch(initial_state)
                .expect("Failed to launch application");
            Ok(())
        })
}



fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| {
        if data.git.is_empty() {
            "Not found!".to_string()
        } else {
            format!("Git: {}", data.git)
        }
    })
        .with_text_size(32.0);

    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::git);

    let button_esp32 = Button::new("ESP32");
    let button_esp32c3 = Button::new("ESP32-C3");
    let button_esp32s2 = Button::new("ESP32-S2");
    let button_esp32s3 = Button::new("ESP32-S3");

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_child(button_esp32)
        .with_child(button_esp32c3)
        .with_child(button_esp32s2)
        .with_child(button_esp32s3)
        .align_vertical(UnitPoint::CENTER)
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_cmd())
        .into_cmd("gui")

        // Optionally specify a description
        .description("GUI for handling ESP-IDF configuration.");

    return multi_cmd;
}