
use std::env;
use clap::{Arg, App};
use clap_nested::{Command, Commander, MultiCommand};
use idf_env_core::rust::{RustToolchain, build_rust_toolchain, install_rust_toolchain, uninstall_rust_toolchain};

pub fn get_default_rust_toolchain(matches: &clap::ArgMatches) -> RustToolchain {
    let triple = guess_host_triple::guess_host_triple().unwrap();

    let toolchain_version = matches.value_of("toolchain-version")
        .unwrap();
    let llvm_version = matches.value_of("llvm-version")
        .unwrap();

    build_rust_toolchain(
        toolchain_version,
        llvm_version,
        triple)
}

fn get_install_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    let toolchain = get_default_rust_toolchain(matches);

    install_rust_toolchain(&toolchain);
    Ok(())
}

fn get_reinstall_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    let toolchain = get_default_rust_toolchain(matches);

    uninstall_rust_toolchain(&toolchain);
    install_rust_toolchain(&toolchain);
    Ok(())
}

fn get_uninstall_runner(_args: &str, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
    let toolchain = get_default_rust_toolchain(matches);

    uninstall_rust_toolchain(&toolchain);
    Ok(())
}


pub fn get_install_cmd<'a>() -> App<> {
    Command::new("install")
        .about("Install Rust environment for Xtensa")
        .options(|app| {
            app.arg(
                Arg::new("toolchain-version")
                    .short('t')
                    .long("toolchain-version")
                    .help("Version of Rust toolchain")
                    .takes_value(true)
                    .default_value("1.57.0.2")
            )
                .arg(
                    Arg::new("llvm-version")
                        .short('l')
                        .long("llvm-version")
                        .help("Version of LLVM with Xtensa support")
                        .takes_value(true)
                        .default_value("esp-13.0.0-20211203")
                )
        })
        .runner(|_args, matches|
            get_install_runner(_args, matches)
        )
}

pub fn get_reinstall_cmd<'a>() -> App<> {
    Command::new("reinstall")
        .about("Re-install Rust environment for Xtensa")
        .options(|app| {
            app.arg(
                Arg::new("toolchain-version")
                    .short('t')
                    .long("toolchain-version")
                    .help("Version of Rust toolchain")
                    .takes_value(true)
                    .default_value("1.57.0.2")
            )
                .arg(
                    Arg::new("llvm-version")
                        .short('l')
                        .long("llvm-version")
                        .help("Version of LLVM with Xtensa support")
                        .takes_value(true)
                        .default_value("esp-13.0.0-20211203")
                )
        })
        .runner(|_args, matches|
            get_reinstall_runner(_args, matches)
        )
}

pub fn get_uninstall_cmd<'a>() -> App<> {
    Command::new("uninstall")
        .about("Uninstall Rust environment for Xtensa")
        .options(|app| {
            app.arg(
                Arg::new("toolchain-version")
                    .short('t')
                    .long("toolchain-version")
                    .help("Version of Rust toolchain")
                    .takes_value(true)
                    .default_value("1.57.0.2")
            )
                .arg(
                    Arg::new("llvm-version")
                        .short('l')
                        .long("llvm-version")
                        .help("Version of LLVM with Xtensa support")
                        .takes_value(true)
                        .default_value("esp-13.0.0-20211203")
                )
        })
        .runner(|_args, matches|
            get_uninstall_runner(_args, matches)
        )
}

pub fn get_multi_cmd<'a>() -> App<> {
    App::new("rust")
        .about("Maintain Rust environment for Xtensa.")
        .subcommand(get_install_cmd())
        .subcommand(get_reinstall_cmd())
        .subcommand(get_uninstall_cmd())
}
