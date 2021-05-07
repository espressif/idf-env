use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text, Checkbox};
use std::fs;

use crate::config::get_tool_path;
use std::ptr::null;

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    toggle_value: bool,
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
        Self::default()
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
            paths
                .enumerate()
                .fold(Column::new().spacing(20), |column, (i, task)| {
                    column.push(
                        Checkbox::new(self.toggle_value, task.unwrap().path().display().to_string(),Message::CheckboxToggled)
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
