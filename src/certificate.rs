use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use tokio::runtime::Handle;

type ResultTokio<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn request_url(uri: String) -> ResultTokio<()> {
    let response = reqwest::get(uri).await;
    match response {
        Ok(_r) => {
            return Ok(())
        },
        _ => {
            std::process::exit(1);
        }
    };
}

fn open_url(url: String) -> ResultTokio<()> {
    let handle = Handle::current().clone();
    let th = std::thread::spawn(move || {
        handle.block_on(request_url(url)).unwrap();
    });
    Ok(th.join().unwrap())
}

fn get_verify_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let url = matches.value_of("url").unwrap().to_string();
    match open_url(url) {
        Ok(_) => { println!("URL verified"); },
        Err(_e) => { println!("Unable to open URL"); }
    }
    Ok(())

}

pub fn get_verify_cmd<'a>() -> Command<'a, str> {
    Command::new("verify")
        .description("Verify whether server with HTTPS certificate is reachable")
        .options(|app| {
            app.arg(
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .help("URL to perform certificate check")
                    .takes_value(true)
            )
        })
        .runner(|_args, matches| get_verify_runner(_args, matches) )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_verify_cmd())
        .into_cmd("certificate")
        .description("Manage HTTPS certificates");

    return multi_cmd;
}
