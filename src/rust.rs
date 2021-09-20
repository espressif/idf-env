use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use dirs::home_dir;
use std::path::Path;
use std::fs::{create_dir_all, remove_dir_all};
use crate::config::get_tool_path;
use crate::package::{prepare_package, prepare_package_strip_prefix};

struct RustToolchain {
    arch: String,
    llvm_release: String,
    llvm_arch: String,
    artifact_file_extension: String,
    version: String,
    rust_dist: String,
    rust_src_dist: String,
    rust_dist_file: String,
    rust_dist_url: String,
    destination_dir: String,
    llvm_file: String,
    llvm_url: String,
    idf_tool_xtensa_elf_clang: String
}

fn get_llvm_arch(arch:&str) -> &str {
    match arch {
        "x86_64-apple-darwin" => "macos",
        "x86_64-unknown-linux-gnu" => "linux-amd64",
        "x86_64-pc-windows-msvc" => "win64",
        _ => arch
    }
}

fn get_artifact_file_extension(arch:&str) -> &str {
    match arch {
        "x86_64-pc-windows-msvc" => "zip",
        _ => "tar.xz"
    }
}

fn build_rust_toolchain(version:&str, arch:&str) -> RustToolchain {
    let llvm_release = "esp-12.0.1-20210823".to_string();
    let artifact_file_extension = get_artifact_file_extension(arch).to_string();
    let llvm_arch = get_llvm_arch(arch).to_string();
    let llvm_file = format!("xtensa-esp32-elf-llvm12_0_1-{}-{}.{}", llvm_release, llvm_arch, artifact_file_extension);
    let rust_dist = format!("rust-{}-{}", version, arch);
    let rust_dist_file = format!("{}.{}", rust_dist, artifact_file_extension);
    let rust_dist_url = format!("https://github.com/esp-rs/rust-build/releases/download/v{}/{}", version, rust_dist_file);
    let llvm_url = format!("https://github.com/espressif/llvm-project/releases/download/{}/{}", llvm_release, llvm_file);
    let idf_tool_xtensa_elf_clang = format!("{}/{}-{}", get_tool_path("xtensa-esp32-elf-clang".to_string()), llvm_release, arch);

    RustToolchain {
        arch: arch.to_string(),
        llvm_release,
        llvm_arch,
        artifact_file_extension,
        version: "1.55.0-dev".to_string(),
        rust_dist,
        rust_src_dist: format!("rust-src-{}", version),
        rust_dist_file,
        rust_dist_url,
        destination_dir: format!("{}/.rustup/toolchains/esp", home_dir().unwrap().display().to_string()),
        llvm_file,
        llvm_url,
        idf_tool_xtensa_elf_clang
    }
}

#[cfg(windows)]
fn set_env_variable(key:String, value:String) {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions
    env.set_value(key, &value).unwrap();
}

#[cfg(unix)]
fn set_env_variable(key:&str, value:&str) {

}


fn install_rust_toolchain(toolchain:&RustToolchain) {
    if Path::new(toolchain.destination_dir.as_str()).exists() {
        println!("Previous installation of Rust Toolchain exist in: {}", toolchain.destination_dir);
        println!("Please, remove the directory before new installation.");
    } else {
        // create_dir_all(toolchain_destination_dir.clone());
        prepare_package_strip_prefix(&toolchain.rust_dist_url,
                                     &toolchain.rust_dist_file,
                                     toolchain.destination_dir.to_string(),
                                     "esp");
    }

    if Path::new(toolchain.idf_tool_xtensa_elf_clang.as_str()).exists() {
        println!("Previous installation of LLVM exist in: {}", toolchain.idf_tool_xtensa_elf_clang);
        println!("Please, remove the directory before new installation.");
    } else {
        prepare_package_strip_prefix(&toolchain.llvm_url,
                                     &toolchain.llvm_file,
                                     toolchain.idf_tool_xtensa_elf_clang.clone(),
                                     "xtensa-esp32-elf-clang"
        );
    }

    println!("Add following command to your shell profile");
    println!("$env:PATH+=\";{}/bin\"", toolchain.idf_tool_xtensa_elf_clang);

    let libclang_path = format!("{}/bin/libclang.dll", toolchain.idf_tool_xtensa_elf_clang);
    println!("$env:LIBCLANG_PATH=\"{}\"", libclang_path);
    set_env_variable("LIBCLANG_PATH".to_string(), libclang_path);

}

fn uninstall_rust_toolchain(toolchain:&RustToolchain) {
    if Path::new(toolchain.destination_dir.as_str()).exists() {
        println!("Removing: {}", toolchain.destination_dir);
        remove_dir_all(&toolchain.destination_dir);
    }

    if Path::new(toolchain.idf_tool_xtensa_elf_clang.as_str()).exists() {
        println!("Removing: {}", toolchain.idf_tool_xtensa_elf_clang);
        remove_dir_all(&toolchain.idf_tool_xtensa_elf_clang);
    }
}

fn get_install_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let toolchain = build_rust_toolchain("1.55.0-dev", "x86_64-pc-windows-msvc");

    install_rust_toolchain(&toolchain);
    Ok(())
}

fn get_reinstall_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let toolchain = build_rust_toolchain("1.55.0-dev", "x86_64-pc-windows-msvc");

    uninstall_rust_toolchain(&toolchain);
    install_rust_toolchain(&toolchain);
    Ok(())
}

fn get_uninstall_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let toolchain = build_rust_toolchain("1.55.0-dev", "x86_64-pc-windows-msvc");

    uninstall_rust_toolchain(&toolchain);
    Ok(())
}

pub fn get_install_cmd<'a>() -> Command<'a, str> {
    Command::new("install")
        .description("Install Rust environment for Xtensa")
        .runner(|_args, matches|
            get_install_runner(_args, matches)
        )
}

pub fn get_reinstall_cmd<'a>() -> Command<'a, str> {
    Command::new("reinstall")
        .description("Re-install Rust environment for Xtensa")
        .runner(|_args, matches|
            get_reinstall_runner(_args, matches)
        )
}

pub fn get_uninstall_cmd<'a>() -> Command<'a, str> {
    Command::new("uninstall")
        .description("Uninstall Rust environment for Xtensa")
        .runner(|_args, matches|
            get_uninstall_runner(_args, matches)
        )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_install_cmd())
        .add_cmd(get_reinstall_cmd())
        .add_cmd(get_uninstall_cmd())
        .into_cmd("rust")

        // Optionally specify a description
        .description("Maintain Rust environment for Xtensa.");

    return multi_cmd;
}
