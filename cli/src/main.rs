extern crate json;
extern crate clap;

use clap_nested::{Commander};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use idf_env_core::antivirus;
use idf_env_core::config;
use idf_env_core::companion;
use idf_env_core::driver;
use idf_env_core::idf;
use idf_env_core::launcher;
use idf_env_core::package;
use idf_env_core::shell;
use idf_env_core::certificate;
use idf_env_core::toit;

mod rust;

async fn app() -> Result<()> {
    Commander::new()
        .options(|app| {
            app.version("1.2.20")
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
        .add_cmd(idf::get_multi_cmd())
        .add_cmd(launcher::get_multi_cmd())
        .add_cmd(rust::get_multi_cmd())
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