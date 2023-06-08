// extern crate json;

use miette::Result;

// mod antivirus;
mod config;
use config::{ ConfigOpts, command_config };
// mod companion;
// mod driver;
// mod ide;
// mod idf;
// mod launcher;
// mod package;
// mod rust;
mod shell;
// mod certificate;
// mod toit;

use clap::{CommandFactory, Parser};

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommand,
}

// async fn app() -> Result<()> {
//     Commander::new()
//         .options(|app| {
//             app.version("2.0.0")
//                 .name("idf-env")
//                 .author("Espressif Systems - https://www.espressif.com")
//                 .about("Tool for maintaining ESP-IDF environment on computer.")
//
//         })
//         .args(|_args, matches| matches.value_of("environment").unwrap_or("dev"))
//         .add_cmd(antivirus::get_multi_cmd())
//         .add_cmd(certificate::get_multi_cmd())
//         .add_cmd(companion::get_multi_cmd())
//         .add_cmd(config::get_multi_cmd())
//         .add_cmd(driver::get_multi_cmd())
//         .add_cmd(ide::get_multi_cmd())
//         .add_cmd(idf::get_multi_cmd())
//         .add_cmd(launcher::get_multi_cmd())
//         .add_cmd(rust::get_multi_cmd())
//         .add_cmd(shell::get_multi_cmd())
//         .add_cmd(toit::get_multi_cmd())
//         .no_cmd(|_args, _matches| {
//             println!("No command matched. Use parameter --help");
//             Ok(())
//         })
//         .run();
//     return Ok(());
// }




#[derive(Parser)]
pub enum SubCommand {
    /// Maintain configuration of ESP-IDF installations.
    Config(ConfigOpts),
    // /// Installs Espressif Rust ecosystem.
    // // We use a Box here to make clippy happy (see https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant)
    // Install(Box<InstallOpts>),
    // /// Uninstalls Espressif Rust ecosystem.
    // Uninstall(UninstallOpts),
    // /// Updates Xtensa Rust toolchain.
    // Update(UpdateOpts),
}

#[tokio::main]
async fn main() -> Result<()> {
    match Cli::parse().subcommand {
        SubCommand::Config(args) => command_config(args).await,
        // SubCommand::Install(args) => install(*args).await,
        // SubCommand::Update(args) => update(args).await,
        // SubCommand::Uninstall(args) => uninstall(args).await,
    }
    // app().await.unwrap();
}
