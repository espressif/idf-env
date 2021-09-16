use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use dirs::home_dir;
use std::path::Path;
use std::fs::create_dir_all;
use crate::config::get_tool_path;
use crate::package::{prepare_package, prepare_package_strip_prefix};

fn get_install_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    // let arch = "aarch64-apple-darwin";
    let arch = "x86_64-pc-windows-msvc";
    let llvm_release = "esp-12.0.1-20210823";
    let mut llvm_arch = llvm_release;
    let mut archive_extension = "tar.xz";

    if arch == "x86_64-apple-darwin" {
        llvm_arch = "macos";
    } else if arch == "x86_64-unknown-linux-gnu" {
        llvm_arch = "linux-amd64";
    } else if arch == "x86_64-pc-windows-msvc" {
        llvm_arch = "win64";
        archive_extension = "zip";
    }

    let version="1.55.0-dev";
    let rust_dist = format!("rust-{}-{}", version, arch);
    let rust_src_dist = format!("rust-src-{}", version);
    let toolchain_destination_dir = format!("{}/.rustup/toolchains/esp", home_dir().unwrap().display().to_string());
    let llvm_file = format!("xtensa-esp32-elf-llvm12_0_1-{}-{}.{}", llvm_release, llvm_arch, archive_extension);
    // IDF_TOOLS_PATH="$HOME/.espressif"
    let idf_tool_xtensa_elf_clang = format!("{}/{}-${}", get_tool_path("xtensa-esp32-elf-clang".to_string()), llvm_release, arch);
    let rust_dist_file = format!("{}.{}", rust_dist, archive_extension);
    let rust_dist_url = format!("https://github.com/esp-rs/rust-build/releases/download/v{}/{}", version, rust_dist_file);
    let llvm_file= format!("xtensa-esp32-elf-llvm12_0_1-{}-{}.{}", llvm_release, llvm_arch, archive_extension);
    let llvm_url= format!("https://github.com/espressif/llvm-project/releases/download/{}/{}", llvm_release, llvm_file);
    let idf_tool_xtensa_elf_clang= format!("{}/{}-{}", get_tool_path("xtensa-esp32-elf-clang".to_string()), llvm_release, arch);

    if Path::new(toolchain_destination_dir.as_str()).exists() {
        println!("Previous installation of toolchain exist in: {}", toolchain_destination_dir);
        println!("Please, remove the directory before new installation.");
        return Ok(())
    }

    create_dir_all(toolchain_destination_dir.clone());
    prepare_package(rust_dist_url,
                    rust_dist_file,
                    toolchain_destination_dir.to_string());

    prepare_package_strip_prefix(llvm_url,
                    llvm_file,
                    idf_tool_xtensa_elf_clang.clone(),
                    "xtensa-esp32-elf-clang"
    );

    println!("Add following command to PowerShell profile");
    println!("$env:PATH+=\";{}/bin\"", idf_tool_xtensa_elf_clang);
    println!("$env:LIBCLANG_PATH=\"{}/bin/libclang.dll\"",idf_tool_xtensa_elf_clang);

    Ok(())
}

pub fn get_install_cmd<'a>() -> Command<'a, str> {
    Command::new("install")
        .description("Install Rust environment for Xtensa")
        .runner(|_args, matches|
            get_install_runner(_args, matches)
        )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_install_cmd())
        .into_cmd("rust")

        // Optionally specify a description
        .description("Maintain Rust environment for Xtensa.");

    return multi_cmd;
}
