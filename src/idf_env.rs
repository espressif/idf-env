extern crate json;
extern crate clap;

use clap_nested::{Commander};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

mod antivirus;
mod config;
mod driver;
mod idf;

async fn app() -> Result<()> {
    Commander::new()
        .options(|app| {
            app.version("1.0.0")
                .name("idf-env")
                .author("Espressif Systems - https://www.espressif.com")
                .about("Tool for maintaining ESP-IDF environment on computer.")
        })
        .args(|_args, matches| matches.value_of("environment").unwrap_or("dev"))
        .add_cmd(antivirus::get_multi_cmd())
        .add_cmd(config::get_multi_cmd())
        .add_cmd(driver::get_multi_cmd())
        .add_cmd(idf::get_multi_cmd())
        .no_cmd(|_args, _matches| {
            println!("No subcommand matched");
            Ok(())
        })
        .run();
    return Ok(());
}

#[tokio::main]
async fn main() {
    app().await.unwrap();
}
