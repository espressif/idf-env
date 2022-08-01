extern crate json;
extern crate clap;

use clap_nested::{Commander};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

mod antivirus;
mod config;
mod companion;
mod driver;
mod ide;
mod idf;
mod launcher;
mod package;
mod rust;
mod shell;
mod certificate;
mod toit;

async fn app() -> Result<()> {
    Commander::new()
        .options(|app| {
            app.version("1.2.28")
                .name("idf-env")
                .author("Espressif Systems - https://www.espressif.com")
                .about("Tool for maintaining ESP-IDF environment on computer.")

        })
        .args(|_args, matches| matches.value_of("environment").unwrap_or("dev"))
        .add_cmd(antivirus::get_multi_cmd())
        .add_cmd(certificate::get_multi_cmd())
        .add_cmd(companion::get_multi_cmd())
        .add_cmd(config::get_multi_cmd())
        .add_cmd(driver::get_multi_cmd())
        .add_cmd(ide::get_multi_cmd())
        .add_cmd(idf::get_multi_cmd())
        .add_cmd(launcher::get_multi_cmd())
        .add_cmd(rust::get_multi_cmd())
        .add_cmd(shell::get_multi_cmd())
        .add_cmd(toit::get_multi_cmd())
        .no_cmd(|_args, _matches| {
            println!("No command matched. Use parameter --help");
            Ok(())
        })
        .run();
    return Ok(());
}

#[tokio::main]
async fn main() {
    app().await.unwrap();
}
