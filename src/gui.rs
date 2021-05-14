use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text, Checkbox};
use std::fs;

use crate::config::get_tool_path;
use std::ptr::null;
use std::collections::HashMap;

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    toggle_value: bool,
    tools_model: HashMap<String, Tool>,
}

use iced::checkbox;

struct Tool {
    name: String,
    observed_state: bool,
    desired_state: bool,
}

impl Tool {
    pub fn view(&mut self) -> Element<Message> {
        let checkbox = Checkbox::new(self.desired_state, self.name.clone(),
                                     Message::CheckboxToggled);
        checkbox.into()
    }
}



#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    CheckboxToggled(bool),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        let mut app = Self::default();
        app.tools_model = HashMap::new();
        let paths = fs::read_dir(get_tool_path("".to_string())).unwrap();

        for path in paths {
            let file_name = path.unwrap().file_name().to_string_lossy().into_owned();
            // let string_path = path.unwrap().path().display().to_string();
            let model = Tool {
                name: file_name.clone(),
                observed_state: false,
                desired_state: false
            };
            app.tools_model.insert(file_name, model);
        }

        return app
    }

    fn title(&self) -> String {
        String::from("idv-env gui")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CheckboxToggled(value) => self.toggle_value = value,
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        let paths = fs::read_dir(get_tool_path("".to_string())).unwrap();
        let tasks: Element<_> = {
            self.tools_model.iter_mut()
                .fold(Column::new().spacing(20), |column, (key, task)| {
                    column.push(
                        task.view()
                    )
                })
                .into()
        };

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Tools"))
            .push(tasks)
            .into()

    }
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
            Counter::run(Settings::default());
            Ok(())
        })
}


pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_cmd())
        .into_cmd("gui")

        // Optionally specify a description
        .description("GUI for handling ESP-IDF configuration.");

    return multi_cmd;
}
