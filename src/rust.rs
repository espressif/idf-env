use std::env;
use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use dirs::home_dir;
use std::path::Path;
use std::fs::{remove_dir_all};
use std::process::Stdio;
use crate::config::get_tool_path;
use crate::package::{prepare_package_strip_prefix, prepare_single_binary};
use crate::shell::{run_command, update_env_path};

const DEFAULT_RUST_TOOLCHAIN_VERSION:&str = "1.60.0.1";
const DEFAULT_LLVM_VERSION:&str = "esp-14.0.0-20220415";

struct RustToolchain {
    arch: String,
    llvm_release: String,
    llvm_arch: String,
    artifact_file_extension: String,
    version: String,
    rust_dist: String,
    rust_dist_temp: String,
    rust_src_dist: String,
    rust_src_dist_temp: String,
    rust_src_dist_file: String,
    rust_dist_file: String,
    rust_dist_url: String,
    rust_src_dist_url: String,
    rust_installer: String,
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

fn get_rust_installer(arch:&str) -> &str {
    match arch {
        "x86_64-pc-windows-msvc" => "",
        _ => "./install.sh"
    }
}

/* Transforms esp-13.0.0-20211203 to 13_0_0 */
fn get_llvm_version_with_underscores(llvm_version: &str) -> String {
    let version:Vec<&str> = llvm_version.split("-").collect();
    let llvm_dot_version = version[1];
    llvm_dot_version.replace(".","_")
}

fn build_rust_toolchain(version:&str, llvm_version: &str, arch:&str) -> RustToolchain {
    let llvm_release = llvm_version.to_string();
    let artifact_file_extension = get_artifact_file_extension(arch).to_string();
    let llvm_arch = get_llvm_arch(arch).to_string();
    let llvm_file = format!("xtensa-esp32-elf-llvm{}-{}-{}.{}", get_llvm_version_with_underscores(&llvm_release), llvm_release, llvm_arch, artifact_file_extension);
    let rust_dist = format!("rust-{}-{}", version, arch);
    let rust_src_dist = format!("rust-src-{}", version);
    let rust_dist_file = format!("{}.{}", rust_dist, artifact_file_extension);
    let rust_src_dist_file =  format!("{}.{}", rust_src_dist, artifact_file_extension);
    let rust_dist_url = format!("https://github.com/esp-rs/rust-build/releases/download/v{}/{}", version, rust_dist_file);
    let rust_src_dist_url = format!("https://github.com/esp-rs/rust-build/releases/download/v{}/{}", version, rust_src_dist_file);
    let llvm_url = format!("https://github.com/espressif/llvm-project/releases/download/{}/{}", llvm_release, llvm_file);
    let idf_tool_xtensa_elf_clang = format!("{}/{}-{}", get_tool_path("xtensa-esp32-elf-clang".to_string()), llvm_release, arch);

    RustToolchain {
        arch: arch.to_string(),
        llvm_release,
        llvm_arch,
        artifact_file_extension,
        version: version.to_string(),
        rust_dist,
        rust_dist_temp: get_tool_path("rust".to_string()),
        rust_src_dist,
        rust_src_dist_temp: get_tool_path("rust-src".to_string()),
        rust_src_dist_file,
        rust_dist_file,
        rust_dist_url,
        rust_src_dist_url,
        rust_installer: get_rust_installer(arch).to_string(),
        destination_dir: format!("{}/.rustup/toolchains/esp", home_dir().unwrap().display().to_string()),
        llvm_file,
        llvm_url,
        idf_tool_xtensa_elf_clang
    }
}

fn install_rust_stable() {
    let rustup_init_path = prepare_single_binary("https://win.rustup.rs/x86_64",
                         "rustup-init.exe",
                          "rustup");
    println!("rustup stable");
    match std::process::Command::new(rustup_init_path)
        .arg("--default-toolchain")
        .arg("stable")
        .arg("-y")
        .stdout(Stdio::piped())
        .output()
    {
        Ok(child_output) => {
            let result = String::from_utf8_lossy(&child_output.stdout);
            println!("{}", result);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn install_rust_nightly() {

    let rustup_path = format!("{}/.cargo/bin/rustup.exe", env::var("USERPROFILE").unwrap());

    println!("{} install nightly", rustup_path);
    match std::process::Command::new(rustup_path)
        .arg("install")
        .arg("nightly")
        .stdout(Stdio::piped())
        .output()
    {
        Ok(child_output) => {
            let result = String::from_utf8_lossy(&child_output.stdout);
            println!("Result: {}", result);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

}

fn install_rust() {
    install_rust_stable();
    install_rust_nightly();
}

fn install_rust_toolchain(toolchain:&RustToolchain) {
    match std::process::Command::new("rustup")
        .arg("toolchain")
        .arg("list")
        .stdout(Stdio::piped())
        .output() {
        Ok(child_output) => {
            println!("rustup - found");
            let result = String::from_utf8_lossy(&child_output.stdout);
            if !result.contains("stable") {
                println!("stable toolchain not found");
                install_rust_stable();
            }
            if !result.contains("nightly") {
                println!("nightly toolchain not found");
                install_rust_nightly();
            }
            println!("rustup - found - {}", String::from_utf8_lossy(&child_output.stdout));
        },
        Err(e) => {
            if let NotFound = e.kind() {
                println!("rustup was not found.");
                install_rust();
            }
        },
    }

    if Path::new(toolchain.destination_dir.as_str()).exists() {
        println!("Previous installation of Rust Toolchain exist in: {}", toolchain.destination_dir);
        println!("Please, remove the directory before new installation.");
    } else {

        // Some platfroms like Windows are available in single bundle rust + src, because install
        // script in dist is not available for the plaform. It's sufficient to extract the toolchain
        if toolchain.rust_installer.is_empty() {
            prepare_package_strip_prefix(&toolchain.rust_dist_url,
                                         &toolchain.rust_dist_file,
                                         toolchain.destination_dir.to_string(),
                                         "esp");
        } else {
            prepare_package_strip_prefix(&toolchain.rust_dist_url,
                                         &toolchain.rust_dist_file,
                                         toolchain.rust_dist_temp.to_string(),
                                         toolchain.rust_dist.as_str());

            let mut arguments: Vec<String> = [].to_vec();

            arguments.push("-c".to_string());
            arguments.push(format!("/tmp/rust/install.sh --destdir={} --prefix='' --without=rust-docs", toolchain.destination_dir));

            run_command("/bin/bash".to_string(), arguments.clone(), "".to_string());

            prepare_package_strip_prefix(&toolchain.rust_src_dist_url,
                                         &toolchain.rust_src_dist_file,
                                         toolchain.rust_src_dist_temp.to_string(),
                                         toolchain.rust_src_dist.as_str());

            let mut arguments: Vec<String> = [].to_vec();

            arguments.push("-c".to_string());
            arguments.push(format!("/tmp/rust-src/install.sh --destdir={} --prefix='' --without=rust-docs", toolchain.destination_dir));

            run_command("/bin/bash".to_string(), arguments, "".to_string());

        }
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

    println!("Updating environment variables:");
    let libclang_bin = format!("{}/bin/", toolchain.idf_tool_xtensa_elf_clang);

    #[cfg(windows)]
    println!("PATH+=\";{}\"", libclang_bin);
    #[cfg(unix)]
    println!("export PATH=\"{}:$PATH\"", libclang_bin);

    update_env_path(&libclang_bin);

    // It seems that LIBCLANG_PATH is not necessary for Windows
    // let libclang_path = format!("{}/libclang.dll", libclang_bin);
    // println!("LIBCLANG_PATH=\"{}\"", libclang_path);
    // set_env_variable("LIBCLANG_PATH", libclang_path);

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

fn get_default_rust_toolchain(matches: &clap::ArgMatches<'_>) -> RustToolchain {
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

fn get_install_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let toolchain = get_default_rust_toolchain(matches);

    install_rust_toolchain(&toolchain);
    Ok(())
}

fn get_reinstall_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let toolchain = get_default_rust_toolchain(matches);

    uninstall_rust_toolchain(&toolchain);
    install_rust_toolchain(&toolchain);
    Ok(())
}

fn get_uninstall_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let toolchain = get_default_rust_toolchain(matches);

    uninstall_rust_toolchain(&toolchain);
    Ok(())
}

pub fn get_install_cmd<'a>() -> Command<'a, str> {
    Command::new("install")
        .description("Install Rust environment for Xtensa")
        .options(|app| {
            app.arg(
                Arg::with_name("toolchain-version")
                    .short("t")
                    .long("toolchain-version")
                    .help("Version of Rust toolchain")
                    .takes_value(true)
                    .default_value(DEFAULT_RUST_TOOLCHAIN_VERSION)
            )
                .arg(
                    Arg::with_name("llvm-version")
                        .short("l")
                        .long("llvm-version")
                        .help("Version of LLVM with Xtensa support")
                        .takes_value(true)
                        .default_value(DEFAULT_LLVM_VERSION)
                )
        })
        .runner(|_args, matches|
            get_install_runner(_args, matches)
        )
}

pub fn get_reinstall_cmd<'a>() -> Command<'a, str> {
    Command::new("reinstall")
        .description("Re-install Rust environment for Xtensa")
        .options(|app| {
            app.arg(
                Arg::with_name("toolchain-version")
                    .short("t")
                    .long("toolchain-version")
                    .help("Version of Rust toolchain")
                    .takes_value(true)
                    .default_value(DEFAULT_RUST_TOOLCHAIN_VERSION)
            )
                .arg(
                    Arg::with_name("llvm-version")
                        .short("l")
                        .long("llvm-version")
                        .help("Version of LLVM with Xtensa support")
                        .takes_value(true)
                        .default_value(DEFAULT_LLVM_VERSION)
                )
        })
        .runner(|_args, matches|
            get_reinstall_runner(_args, matches)
        )
}

pub fn get_uninstall_cmd<'a>() -> Command<'a, str> {
    Command::new("uninstall")
        .description("Uninstall Rust environment for Xtensa")
        .options(|app| {
            app.arg(
                Arg::with_name("toolchain-version")
                    .short("t")
                    .long("toolchain-version")
                    .help("Version of Rust toolchain")
                    .takes_value(true)
                    .default_value(DEFAULT_RUST_TOOLCHAIN_VERSION)
            )
                .arg(
                    Arg::with_name("llvm-version")
                        .short("l")
                        .long("llvm-version")
                        .help("Version of LLVM with Xtensa support")
                        .takes_value(true)
                        .default_value(DEFAULT_LLVM_VERSION)
                )
        })
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
