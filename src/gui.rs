use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
use iced::Sandbox;


impl Sandbox for Defender {
    fn view(&mut self) -> Element<Message> {

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
            let app = App::default();
            let mut wind = DoubleWindow::new(100, 100, 400, 300, "Hello from rust");
            let mut but = Button::new(160, 210, 80, 40, "Click me!");
            wind.shape()
            wind.end();
            wind.show();
            app.run().unwrap();
            // nwg::init().expect("Failed to init Native Windows GUI");
            //
            // let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
            //
            // nwg::dispatch_thread_events();
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
