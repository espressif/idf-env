// On Windows
#![windows_subsystem = "windows"]

use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
use std::path::Path;

use crate::config::{get_git_path, get_selected_idf_path, get_home_dir, get_tools_path};
use crate::shell::{ run_command, start_terminal };

use druid::widget::{Flex, Label, TextBox, Button, Checkbox};
use druid::{commands, AppLauncher, AppDelegate, Data, DelegateCtx, FileDialogOptions, Handled, Lens, Target, UnitPoint, WidgetExt, WindowDesc, Widget, Env, Selector, FileInfo};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 400.0;

struct Delegate;


fn get_idf_suggested_path() -> String {
    let idf_path = get_selected_idf_path();
    if idf_path == "null" {
        return format!("{}/esp/esp-idf", get_home_dir());
    }
    return idf_path;
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

            let data = AppData {
                target: "esp32".into(),
                is_target_esp32: true,
                is_target_esp32c3: true,
                is_target_esp32s2: false,
                is_target_esp32s3: true,
                idf_path: get_idf_suggested_path(),
                idf_tools_path: get_tools_path()
            };

            // start the application. Here we pass in the application state.
            AppLauncher::with_window(main_window)
                .delegate(Delegate)
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
    idf_path: String,
    idf_tools_path: String
}

pub const OPEN_IDF_PATH: Selector<FileInfo> = Selector::new("idf-env.open-idf-path");
pub const OPEN_IDF_TOOLS_PATH: Selector<FileInfo> = Selector::new("idf-env.open-idf-tools-path");

impl AppDelegate<AppData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &druid::Command,
        data: &mut AppData,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(OPEN_IDF_PATH) {
            // if let Err(e) = std::fs::write(file_info.path(), &data[..]) {
            //     println!("Error writing file: {}", e);
            // }
            data.idf_path = file_info.path.display().to_string();
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(OPEN_IDF_TOOLS_PATH) {
            // match std::fs::read_to_string(file_info.path()) {
            //     Ok(s) => {
            //         let first_line = s.lines().next().unwrap_or("");
            //         *data = first_line.to_owned();
            //     }
            //     Err(e) => {
            //         println!("Error opening file: {}", e);
            //     }
            // }
            data.idf_tools_path = file_info.path.display().to_string();
            return Handled::Yes;
        }
        Handled::No
    }
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
    let textbox_esp_idf_path = TextBox::new()
        .with_placeholder("~/esp/esp-idf")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AppData::idf_path);
    //     // .lens(HelloState::git);

    let textbox_esp_idf_tools_path = TextBox::new()
        .with_placeholder("~/.espressif")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AppData::idf_tools_path);

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

    let button_apply = Button::new("Modify installation").on_click(|_ctx, data: &mut AppData, _env| {
        let idf_path = Path::new(data.idf_path.as_str());
        let idf_path_str = idf_path.display().to_string();
        let idf_parent = idf_path.parent().unwrap().display().to_string();

        if !idf_path.exists() {
            let clone_command = format!("mkdir -p '{}'; cd '{}'; git clone https://github.com/espressif/esp-idf --depth 1 --recursive '{}' --jobs=8", idf_parent, idf_parent, idf_path_str);
            start_terminal(clone_command.as_str());
        }

        #[cfg(windows)]
        let install_command = format!("cd {}\\; ./install.bat\\; ./export.ps1", idf_path_str);
        #[cfg(unix)]
        let install_command = format!("cd {}; ./install.sh && . ./export.sh", idf_path_str);

        start_terminal(install_command.as_str());

    });

    let button_terminal = Button::new("ESP-IDF Terminal").on_click(|_ctx, data: &mut AppData, _env| {
        let idf_path = Path::new(data.idf_path.as_str());
        let idf_path_str = idf_path.display().to_string();
        #[cfg(windows)]
        let command = format!("cd {}\\; . ./export.ps1", idf_path_str);
        #[cfg(unix)]
        let command = format!("cd {}; . ./export.sh", idf_path_str);
        start_terminal(command.as_str());
        // let mut arguments: Vec<&str> = [].to_vec();
        // arguments.push("-a");
        // arguments.push("iTerm");
        // run_command("/usr/bin/open", arguments, ".");
        // arguments.push("-c");
        // run_command("/bin/bash", arguments, "open -a iTerm .");
    });


    let browse_idf_path_options = FileDialogOptions::new()
        .name_label("ESP-IDF - Source")
        .select_directories()
        .title("Where to store ESP-IDF source code")
        .accept_command(OPEN_IDF_PATH)
        .button_text("Select");

    let browse_idf_path_button = Button::new("Browse").on_click(move |ctx, data: &mut AppData, _env| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(browse_idf_path_options.clone()))
    });

    let browse_idf_tools_path_options = FileDialogOptions::new()
        .name_label("ESP-IDF - Tools")
        .select_directories()
        .title("Where to store ESP-IDF tools")
        .accept_command(OPEN_IDF_TOOLS_PATH)
        .button_text("Select");

    let browse_idf_tools_path_button = Button::new("Browse").on_click(move |ctx, data: &mut AppData, _env| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(browse_idf_tools_path_options.clone()))
    });

    let checkbox_esp32 = Checkbox::new("ESP32").lens(AppData::is_target_esp32);
    let checkbox_esp32c3 = Checkbox::new("ESP32-C3").lens(AppData::is_target_esp32c3);
    let checkbox_esp32s2 = Checkbox::new("ESP32-S2").lens(AppData::is_target_esp32s2);
    let checkbox_esp32s3 = Checkbox::new("ESP32-S3").lens(AppData::is_target_esp32s3);


    // arrange the two widgets vertically, with some padding
    // Flex::column()
    //     .with_child(label)
    //     .with_spacer(VERTICAL_WIDGET_SPACING)
    //     .with_child(textbox)
    //     // .with_child(button_esp32)
    //     // .with_child(button_esp32c3)
    //     // .with_child(button_esp32s2)
    //     // .with_child(button_esp32s3)
    //     .with_child(checkbox_esp32)
    //     // .with_child(checkbox_esp32c3)
    //     // .with_child(checkbox_esp32s2)
    //     // .with_child(checkbox_esp32s3)
    //     .with_child(button_apply)
    //     .with_child(button_terminal)
    //     .align_vertical(UnitPoint::CENTER);

    Flex::column()
        .with_flex_child(
            Flex::row()
                .with_flex_child(Label::new("ESP-IDF Path: "), 1.0)
                .with_flex_child(textbox_esp_idf_path, 1.0)
                .with_flex_child(browse_idf_path_button, 1.0),
            1.0)
        .with_flex_child(
            Flex::row()
                .with_flex_child(Label::new("ESP-IDF Tools Path: "), 1.0)
                .with_flex_child(textbox_esp_idf_tools_path, 1.0)
                .with_flex_child(browse_idf_tools_path_button, 1.0),
            1.0)
        .with_flex_child(
            Flex::row()
                .with_flex_child(button_apply, 1.0)
                .with_flex_child(button_terminal, 1.0),
            1.0)
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_cmd())
        .into_cmd("gui")

        // Optionally specify a description
        .description("GUI for handling ESP-IDF configuration.");

    return multi_cmd;
}
