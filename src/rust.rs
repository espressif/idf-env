use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

use crate::config::get_tool_path;
use crate::package::{prepare_package, prepare_package_strip_prefix, prepare_single_binary};
use crate::shell::{run_command, update_env_path};
use dirs::home_dir;
use std::fs::{copy, remove_dir_all};
use std::path::Path;
use std::process::Stdio;

const DEFAULT_RUST_TOOLCHAIN_VERSION: &str = "1.63.0.0";
const DEFAULT_LLVM_VERSION: &str = "esp-14.0.0-20220415";

struct RustCrate {
    name: String,
    url: String,
    dist_file: String,
    dist_bin: String,
    bin: String,
}

struct RustToolchain {
    arch: String,
    //llvm_release: String,
    //llvm_arch: String,
    //artifact_file_extension: String,
    // version: String,
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
    idf_tool_xtensa_elf_clang: String,
    extra_tools: String,
    extra_crates: Vec<RustCrate>,
    mingw_url: String,
    mingw_dist_file: String,
    mingw_destination_directory: String,
}

fn get_home_dir() -> String {
    home_dir().unwrap().display().to_string()
}

fn get_llvm_arch(arch: &str) -> &str {
    match arch {
        "x86_64-apple-darwin" => "macos",
        "x86_64-unknown-linux-gnu" => "linux-amd64",
        "x86_64-pc-windows-msvc" => "win64",
        "x86_64-pc-windows-gnu" => "win64",
        _ => arch,
    }
}

fn get_os_bin_extension(arch: &str) -> &str {
    match arch {
        "x86_64-pc-windows-msvc" => ".exe",
        "x86_64-pc-windows-gnu" => ".exe",
        _ => "",
    }
}

fn get_artifact_file_extension(arch: &str) -> &str {
    match arch {
        "x86_64-pc-windows-msvc" => "zip",
        "x86_64-pc-windows-gnu" => "zip",
        _ => "tar.xz",
    }
}

fn get_rust_installer(arch: &str) -> &str {
    match arch {
        "x86_64-pc-windows-msvc" => "",
        "x86_64-pc-windows-gnu" => "",
        _ => "./install.sh",
    }
}

/* Transforms esp-13.0.0-20211203 to 13_0_0 */
fn get_llvm_version_with_underscores(llvm_version: &str) -> String {
    let version: Vec<&str> = llvm_version.split("-").collect();
    let llvm_dot_version = version[1];
    llvm_dot_version.replace(".", "_")
}

fn get_cargo_home() -> String {
    format!("{}/.cargo", get_home_dir())
}

fn get_rust_crate(name: &str, arch: &str) -> Option<RustCrate> {
    let os_bin_extension = get_os_bin_extension(arch);
    match name {
        "cargo-espflash" => {
            Some(RustCrate {
                name: name.to_string(),
                url: format!("https://github.com/esp-rs/espflash/releases/latest/download/cargo-espflash-{}.zip", arch),
                dist_file: format!("cargo-espflash-{}.zip", arch),
                dist_bin: format!("cargo-espflash{}", os_bin_extension),
                bin: format!("{}/bin/cargo-espflash{}", get_cargo_home(), os_bin_extension)
            })
        },
        "cargo-generate" => {
            let url = match arch {
                "x86_64-pc-windows-msvc" => { format!("https://github.com/cargo-generate/cargo-generate/releases/download/v{}/cargo-generate-v{}-{}.tar.gz", "0.16.0", "0.16.0", "x86_64-pc-windows-msvc") },
                _ => { "".to_string() }
            };
            Some(RustCrate {
                name: name.to_string(),
                url,
                dist_file: format!("cargo-generate-{}.tar.gz", arch),
                dist_bin: format!("cargo-generate{}", os_bin_extension),
                bin: format!("{}/bin/cargo-generate{}", get_cargo_home(), os_bin_extension)
            })
        },
        "espflash" => {
            Some(RustCrate {
                name: name.to_string(),
                url: format!("https://github.com/esp-rs/espflash/releases/latest/download/espflash-{}.zip", arch),
                dist_file: format!("espflash-{}.zip", arch),
                dist_bin: format!("espflash{}", os_bin_extension),
                bin: format!("{}/bin/espflash{}", get_cargo_home(), os_bin_extension)
            })
        },
        "ldproxy" => {
            Some(RustCrate {
                name: name.to_string(),
                url: format!("https://github.com/esp-rs/embuild/releases/latest/download/ldproxy-{}.zip", arch),
                dist_file: format!("ldproxy-{}.zip", arch),
                dist_bin: format!("ldproxy{}", os_bin_extension),
                bin: format!("{}/bin/ldproxy{}", get_cargo_home(), os_bin_extension)
            })
        },
        "wokwi-server" => {
            Some(RustCrate {
                name: name.to_string(),
                url: format!("https://github.com/MabezDev/wokwi-server/releases/latest/download/wokwi-server-{}.zip", arch),
                dist_file: format!("wokwi-server-{}.zip", arch),
                dist_bin: format!("wokwi-server{}", os_bin_extension),
                bin: format!("{}/bin/wokwi-server{}", get_cargo_home(), os_bin_extension)
            })
        },
        "web-flash" => {
            Some(RustCrate {
                name: name.to_string(),
                url: format!("https://github.com/bjoernQ/esp-web-flash-server/releases/latest/download/web-flash-{}.zip", arch),
                dist_file: format!("web-flash-{}.zip", arch),
                dist_bin: format!("web-flash{}", os_bin_extension),
                bin: format!("{}/bin/web-flash{}", get_cargo_home(), os_bin_extension)
            })
        },
        _ => None
    }
}

fn get_extra_crates(crates_list: &str, arch: &str) -> Vec<RustCrate> {
    crates_list
        .split(",")
        .into_iter()
        .filter_map(|s| get_rust_crate(s, arch))
        .collect()
}

fn build_rust_toolchain(
    version: &str,
    llvm_version: &str,
    arch: &str,
    extra_tools: &str,
    extra_crates_list: &str,
) -> RustToolchain {
    let llvm_release = llvm_version.to_string();
    let artifact_file_extension = get_artifact_file_extension(arch).to_string();
    let llvm_arch = get_llvm_arch(arch).to_string();
    let llvm_file = format!(
        "xtensa-esp32-elf-llvm{}-{}-{}.{}",
        get_llvm_version_with_underscores(&llvm_release),
        llvm_release,
        llvm_arch,
        artifact_file_extension
    );
    let rust_dist = format!("rust-{}-{}", version, arch);
    let rust_src_dist = format!("rust-src-{}", version);
    let rust_dist_file = format!("{}.{}", rust_dist, artifact_file_extension);
    let rust_src_dist_file = format!("{}.{}", rust_src_dist, artifact_file_extension);
    let rust_dist_url = format!(
        "https://github.com/esp-rs/rust-build/releases/download/v{}/{}",
        version, rust_dist_file
    );
    let rust_src_dist_url = format!(
        "https://github.com/esp-rs/rust-build/releases/download/v{}/{}",
        version, rust_src_dist_file
    );
    let llvm_url = format!(
        "https://github.com/espressif/llvm-project/releases/download/{}/{}",
        llvm_release, llvm_file
    );
    let idf_tool_xtensa_elf_clang = format!(
        "{}/{}-{}",
        get_tool_path("xtensa-esp32-elf-clang".to_string()),
        llvm_release,
        arch
    );
    let mingw_release = "x86_64-12.1.0-release-posix-seh-rt_v10-rev3".to_string();
    let mingw_dist_file = format!("{}.zip", mingw_release);
    // Temporal solution - repackaging 7z to zip, because Rust based decompression crate does not have BCJ support: https://github.com/dyz1990/sevenz-rust/issues/1
    //let mingw_dist_file = format!("{}.7z", mingw_release);
    //let mingw_url = format!("https://github.com/niXman/mingw-builds-binaries/releases/download/12.1.0-rt_v10-rev3/{}", mingw_dist_file);
    let mingw_url = format!(
        "https://github.com/esp-rs/rust-build/releases/download/mingw-12/{}",
        mingw_dist_file
    );
    let mingw_destination_directory =
        format!("{}/{}", get_tool_path("mingw".to_string()), mingw_release);

    RustToolchain {
        arch: arch.to_string(),
        //llvm_release,
        //llvm_arch,
        //artifact_file_extension,
        //version: version.to_string(),
        rust_dist,
        rust_dist_temp: get_tool_path("rust".to_string()),
        rust_src_dist,
        rust_src_dist_temp: get_tool_path("rust-src".to_string()),
        rust_src_dist_file,
        rust_dist_file,
        rust_dist_url,
        rust_src_dist_url,
        rust_installer: get_rust_installer(arch).to_string(),
        destination_dir: format!("{}/.rustup/toolchains/esp", get_home_dir()),
        llvm_file,
        llvm_url,
        idf_tool_xtensa_elf_clang,
        extra_tools: extra_tools.to_string(),
        extra_crates: get_extra_crates(extra_crates_list, arch),
        mingw_url,
        mingw_dist_file,
        mingw_destination_directory,
    }
}

fn install_rust_stable(default_host: &str) {
    let rustup_init_path =
        prepare_single_binary("https://win.rustup.rs/x86_64", "rustup-init.exe", "rustup");
    println!("rustup stable");
    match std::process::Command::new(rustup_init_path)
        .arg("--default-toolchain")
        .arg("stable")
        .arg("-y")
        .arg("--default-host")
        .arg(default_host)
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
    let rustup_path = format!("{}/bin/rustup.exe", get_cargo_home());

    println!("{} install nightly", rustup_path);
    match std::process::Command::new(rustup_path)
        .arg("install")
        .arg("nightly")
        // .arg("--default-host")
        // .arg(default_host)
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

fn install_rust(default_host: &str) {
    install_rust_stable(default_host);
    install_rust_nightly();
}

fn install_mingw(toolchain: &RustToolchain) {
    if Path::new(toolchain.mingw_destination_directory.as_str()).exists() {
        println!(
            "Previous installation of MinGW exist in: {}",
            toolchain.mingw_destination_directory
        );
        println!("Please, remove the directory before new installation.");
        return;
    }

    match prepare_package_strip_prefix(
        &toolchain.mingw_url,
        &toolchain.mingw_dist_file,
        toolchain.mingw_destination_directory.clone(),
        "mingw64",
    ) {
        Ok(_) => {
            println!("Package ready");
        }
        Err(_e) => {
            println!("Unable to prepare package");
        }
    }
}

fn install_extra_crates(extra_crates: &Vec<RustCrate>) {
    for extra_crate in extra_crates.into_iter() {
        println!("Installing crate {}", extra_crate.name);

        if extra_crate.url.is_empty() {
            // Binary crate is not available, install from source code
            let cargo_path = format!("{}/bin/cargo.exe", get_cargo_home());

            println!("{} install {}", cargo_path, extra_crate.name);
            match std::process::Command::new(cargo_path)
                .arg("install")
                .arg(extra_crate.name.to_string())
                .stdout(Stdio::piped())
                .output()
            {
                Ok(child_output) => {
                    let result = String::from_utf8_lossy(&child_output.stdout);
                    println!("Crate installed: {}", result);
                }
                Err(e) => {
                    println!("Crate installation failed: {}", e);
                }
            }
        } else {
            // Binary crate is available donwload it
            let tmp_path = get_tool_path(extra_crate.name.to_string());
            match prepare_package(
                extra_crate.url.to_string(),
                &extra_crate.dist_file,
                tmp_path,
            ) {
                Ok(_) => {
                    let source = format!(
                        "{}/{}",
                        get_tool_path(extra_crate.name.to_string()),
                        extra_crate.dist_bin
                    );
                    match copy(source.clone(), extra_crate.bin.to_string()) {
                        Ok(_) => {
                            println!("Create {} installed.", extra_crate.name);
                        }
                        Err(_e) => {
                            println!(
                                "Unable to copy crate binary from {} to {}",
                                source, extra_crate.bin
                            )
                        }
                    }
                }
                Err(_e) => {
                    println!("Unable to unpack bianry crate {}.", extra_crate.name);
                }
            };
        }
    }
}

fn install_vctools() {
    // installer: https://docs.microsoft.com/en-us/visualstudio/install/use-command-line-parameters-to-install-visual-studio?view=vs-2022
    // Windows 10 SDK - https://docs.microsoft.com/en-us/visualstudio/install/workload-component-id-vs-build-tools?view=vs-2022&preserve-view=true
    // .\vs_BuildTools.exe --passive --wait --add Microsoft.VisualStudio.Component.Windows10SDK.20348
    // .\vs_BuildTools.exe --passive --wait --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows10SDK.20348
    // path C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.33.31629\bin\Hostx64\x64

    let vs_build_tools = prepare_single_binary(
        "https://aka.ms/vs/17/release/vs_buildtools.exe",
        "vs_buildtools.exe",
        "vs_buildtools",
    );
    println!("Running VS BuildTools: vs_BuildTools.exe --passive --wait --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows10SDK.20348");

    match std::process::Command::new(vs_build_tools)
        .arg("--passive")
        .arg("--wait")
        .arg("--add")
        .arg("Microsoft.VisualStudio.Component.VC.Tools.x86.x64")
        .arg("--add")
        .arg("Microsoft.VisualStudio.Component.Windows10SDK.20348")
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

fn install_rust_toolchain(toolchain: &RustToolchain) {
    match std::process::Command::new("rustup")
        .arg("toolchain")
        .arg("list")
        .stdout(Stdio::piped())
        .output()
    {
        Ok(child_output) => {
            println!("rustup - found");
            let result = String::from_utf8_lossy(&child_output.stdout);
            if !result.contains("stable") {
                println!("stable toolchain not found");
                install_rust_stable(&toolchain.arch);
            }
            if !result.contains("nightly") {
                println!("nightly toolchain not found");
                install_rust_nightly();
            }
            println!(
                "rustup - found - {}",
                String::from_utf8_lossy(&child_output.stdout)
            );
        }
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                println!("rustup was not found.");
                install_rust(&toolchain.arch);
            }
        }
    }

    if Path::new(toolchain.destination_dir.as_str()).exists() {
        println!(
            "Previous installation of Rust Toolchain exist in: {}",
            toolchain.destination_dir
        );
        println!("Please, remove the directory before new installation.");
    } else {
        // Some platfroms like Windows are available in single bundle rust + src, because install
        // script in dist is not available for the plaform. It's sufficient to extract the toolchain
        if toolchain.rust_installer.is_empty() {
            match prepare_package_strip_prefix(
                &toolchain.rust_dist_url,
                &toolchain.rust_dist_file,
                toolchain.destination_dir.to_string(),
                "esp",
            ) {
                Ok(_) => {
                    println!("Package ready");
                }
                Err(_e) => {
                    println!("Unable to prepare package");
                }
            }
        } else {
            match prepare_package_strip_prefix(
                &toolchain.rust_dist_url,
                &toolchain.rust_dist_file,
                toolchain.rust_dist_temp.to_string(),
                toolchain.rust_dist.as_str(),
            ) {
                Ok(_) => {
                    println!("Package ready");
                }
                Err(_e) => {
                    println!("Unable to prepare package");
                }
            }

            let mut arguments: Vec<String> = [].to_vec();

            arguments.push("-c".to_string());
            arguments.push(format!(
                "/tmp/rust/install.sh --destdir={} --prefix='' --without=rust-docs",
                toolchain.destination_dir
            ));

            match run_command("/bin/bash".to_string(), arguments.clone(), "".to_string()) {
                Ok(_) => {
                    println!("Command succeeded");
                }
                Err(_e) => {
                    println!("Command failed");
                }
            }

            match prepare_package_strip_prefix(
                &toolchain.rust_src_dist_url,
                &toolchain.rust_src_dist_file,
                toolchain.rust_src_dist_temp.to_string(),
                toolchain.rust_src_dist.as_str(),
            ) {
                Ok(_) => {
                    println!("Package ready");
                }
                Err(_e) => {
                    println!("Unable to prepare package");
                }
            }

            let mut arguments: Vec<String> = [].to_vec();

            arguments.push("-c".to_string());
            arguments.push(format!(
                "/tmp/rust-src/install.sh --destdir={} --prefix='' --without=rust-docs",
                toolchain.destination_dir
            ));

            match run_command("/bin/bash".to_string(), arguments, "".to_string()) {
                Ok(_) => {
                    println!("Command succeeded");
                }
                Err(_e) => {
                    println!("Command failed");
                }
            }
        }
    }

    if Path::new(toolchain.idf_tool_xtensa_elf_clang.as_str()).exists() {
        println!(
            "Previous installation of LLVM exist in: {}",
            toolchain.idf_tool_xtensa_elf_clang
        );
        println!("Please, remove the directory before new installation.");
    } else {
        match prepare_package_strip_prefix(
            &toolchain.llvm_url,
            &toolchain.llvm_file,
            toolchain.idf_tool_xtensa_elf_clang.clone(),
            "xtensa-esp32-elf-clang",
        ) {
            Ok(_) => {
                println!("Package ready");
            }
            Err(_e) => {
                println!("Unable to prepare package");
            }
        }
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

    // Install additional dependencies specific for the host
    // for extra_tool in toolchain.extra_tools
    match toolchain.extra_tools.as_str() {
        "mingw" => match toolchain.arch.as_str() {
            "x86_64-pc-windows-gnu" => {
                install_mingw(toolchain);
                update_env_path(format!("{}/bin", toolchain.mingw_destination_directory).as_str());
            }
            _ => {
                println!("Ok");
            }
        },
        "vctools" => {
            install_vctools();
            update_env_path("C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Tools\\MSVC\\14.33.31629\\bin\\Hostx64\\x64");
        }
        _ => {
            println!("No extra tools selected");
        }
    }

    install_extra_crates(&toolchain.extra_crates);
}

fn uninstall_rust_toolchain(toolchain: &RustToolchain) {
    if Path::new(toolchain.destination_dir.as_str()).exists() {
        println!("Removing: {}", toolchain.destination_dir);
        match remove_dir_all(&toolchain.destination_dir) {
            Ok(_) => {
                println!("Removed.");
            }
            Err(_e) => {
                println!("Failed to remove.");
            }
        }
    }

    if Path::new(toolchain.idf_tool_xtensa_elf_clang.as_str()).exists() {
        println!("Removing: {}", toolchain.idf_tool_xtensa_elf_clang);
        match remove_dir_all(&toolchain.idf_tool_xtensa_elf_clang) {
            Ok(_) => {
                println!("Removed.");
            }
            Err(_e) => {
                println!("Failed to remove.");
            }
        }
    }
}

fn get_default_rust_toolchain(matches: &clap::ArgMatches<'_>) -> RustToolchain {
    let default_host_triple = matches.value_of("default-host").unwrap();

    let toolchain_version = matches.value_of("toolchain-version").unwrap();
    let llvm_version = matches.value_of("llvm-version").unwrap();
    let extra_tools = matches.value_of("extra-tools").unwrap();
    let extra_crates_list = matches.value_of("extra-crates").unwrap();

    build_rust_toolchain(
        toolchain_version,
        llvm_version,
        default_host_triple,
        extra_tools,
        extra_crates_list,
    )
}

fn get_install_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
    let toolchain = get_default_rust_toolchain(matches);

    install_rust_toolchain(&toolchain);
    Ok(())
}

fn get_reinstall_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
    let toolchain = get_default_rust_toolchain(matches);

    uninstall_rust_toolchain(&toolchain);
    install_rust_toolchain(&toolchain);
    Ok(())
}

fn get_uninstall_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
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
                    .long("toolchain-version")
                    .help("Version of Rust toolchain")
                    .takes_value(true)
                    .default_value(DEFAULT_RUST_TOOLCHAIN_VERSION),
            )
            .arg(
                Arg::with_name("llvm-version")
                    .short("l")
                    .long("llvm-version")
                    .help("Version of LLVM with Xtensa support")
                    .takes_value(true)
                    .default_value(DEFAULT_LLVM_VERSION),
            )
            .arg(
                Arg::with_name("default-host")
                    .short("d")
                    .long("default-host")
                    .help("Default host triple for Rust installation")
                    .takes_value(true)
                    .default_value(guess_host_triple::guess_host_triple().unwrap()),
            )
            .arg(
                Arg::with_name("extra-tools")
                    .short("t")
                    .long("extra-tools")
                    .help("Extra tools which should be deployed. E.g. MinGW")
                    .takes_value(true)
                    .default_value(""),
            )
            .arg(
                Arg::with_name("extra-crates")
                    .short("e")
                    .long("extra-crates")
                    .help("Extra crates which should be deployed. E.g. cargo-espflash")
                    .takes_value(true)
                    .default_value(""),
            )
        })
        .runner(|_args, matches| get_install_runner(_args, matches))
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
                    .default_value(DEFAULT_RUST_TOOLCHAIN_VERSION),
            )
            .arg(
                Arg::with_name("llvm-version")
                    .short("l")
                    .long("llvm-version")
                    .help("Version of LLVM with Xtensa support")
                    .takes_value(true)
                    .default_value(DEFAULT_LLVM_VERSION),
            )
            .arg(
                Arg::with_name("default-host")
                    .short("d")
                    .long("default-host")
                    .help("Default host triple for Rust installation")
                    .takes_value(true)
                    .default_value(guess_host_triple::guess_host_triple().unwrap()),
            )
        })
        .runner(|_args, matches| get_reinstall_runner(_args, matches))
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
                    .default_value(DEFAULT_RUST_TOOLCHAIN_VERSION),
            )
            .arg(
                Arg::with_name("llvm-version")
                    .short("l")
                    .long("llvm-version")
                    .help("Version of LLVM with Xtensa support")
                    .takes_value(true)
                    .default_value(DEFAULT_LLVM_VERSION),
            )
            .arg(
                Arg::with_name("default-host")
                    .short("d")
                    .long("default-host")
                    .help("Default host triple for Rust installation")
                    .takes_value(true)
                    .default_value(guess_host_triple::guess_host_triple().unwrap()),
            )
        })
        .runner(|_args, matches| get_uninstall_runner(_args, matches))
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_extra_crates() {
        let extra_crates =
            get_extra_crates("cargo-espflash,unknown,Unknonwn", "x86_64-pc-windows-gnu");
        assert_eq!(extra_crates.len(), 1);
        let extra_crates = get_extra_crates(
            "cargo-espflash,cargo-generate,ldproxy",
            "x86_64-pc-windows-gnu",
        );
        assert_eq!(extra_crates.len(), 3);
        let extra_crates = get_extra_crates(
            "cargo-espflash,cargo-generate,ldproxy,espflash,wokwi-server",
            "x86_64-pc-windows-gnu",
        );
        assert_eq!(extra_crates.len(), 5);
    }
}
