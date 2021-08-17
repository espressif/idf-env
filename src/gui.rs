// On Windows
#![windows_subsystem = "windows"]

use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use crate::config::get_git_path;
use crate::shell::{ run_command, start_terminal };

use druid::widget::{Flex, Label, TextBox, Button, Checkbox};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc, Widget, Env};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

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

            let data = AppData {
                target: "esp32".into(),
                is_target_esp32: true,
                is_target_esp32c3: true,
                is_target_esp32s2: false,
                is_target_esp32s3: true
            };

            // start the application. Here we pass in the application state.
            AppLauncher::with_window(main_window)
                .launch(data)
                .expect("Failed to launch application");
            Ok(())
        })
}


#[derive(Clone, Data, Lens)]
struct AppData {
    target: String,
    is_target_esp32: bool,
    is_target_esp32c3: bool,
    is_target_esp32s2: bool,
    is_target_esp32s3: bool,
}

fn build_root_widget() -> impl Widget<AppData> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &AppData, _env: &Env| {
        if data.target.is_empty() {
            "Not set!".to_string()
        } else {
            format!("Target: {}", data.target)
        }
    })
        .with_text_size(32.0);

    // a textbox that modifies `name`.
    // let textbox = TextBox::new()
    //     .with_placeholder("Who are we greeting?")
    //     .with_text_size(18.0)
    //     .fix_width(TEXT_BOX_WIDTH);
    //     // .lens(HelloState::git);

    let button_esp32 = Button::new("ESP32").on_click(|_ctx, data: &mut AppData, _env| {
        data.target = "esp32".into();
    });
    let button_esp32c3 = Button::new("ESP32-C3").on_click(|_ctx, data: &mut AppData, _env| {
        data.target = "esp32c3".into();
    });
    let button_esp32s2 = Button::new("ESP32-S2").on_click(|_ctx, data: &mut AppData, _env| {
        data.target = "esp32s2".into();
    });
    let button_esp32s3 = Button::new("ESP32-S3").on_click(|_ctx, data: &mut AppData, _env| {
        data.target = "esp32s3".into();
    });

    let button_idf_rust= Button::new("idf-rust").on_click(|_ctx, data: &mut AppData, _env| {
        start_terminal();
        // let mut arguments: Vec<&str> = [].to_vec();
        // arguments.push("-a");
        // arguments.push("iTerm");
        // run_command("/usr/bin/open", arguments, ".");
        // arguments.push("-c");
        // run_command("/bin/bash", arguments, "open -a iTerm .");
    });


    let checkbox_esp32 = Checkbox::new("ESP32").lens(AppData::is_target_esp32);
    let checkbox_esp32c3 = Checkbox::new("ESP32-C3").lens(AppData::is_target_esp32c3);
    let checkbox_esp32s2 = Checkbox::new("ESP32-S2").lens(AppData::is_target_esp32s2);
    let checkbox_esp32s3 = Checkbox::new("ESP32-S3").lens(AppData::is_target_esp32s3);


    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        // .with_child(textbox)
        .with_child(button_esp32)
        .with_child(button_esp32c3)
        .with_child(button_esp32s2)
        .with_child(button_esp32s3)
        .with_child(checkbox_esp32)
        .with_child(checkbox_esp32c3)
        .with_child(checkbox_esp32s2)
        .with_child(checkbox_esp32s3)
        .with_child(button_idf_rust)
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