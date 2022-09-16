use crate::config::{
    get_cargo_home, get_dist_path, get_esp_idf_directory, get_home_dir, get_log_level,
    get_tool_path, get_tools_path, parse_idf_targets, parse_targets,
};
use crate::emoji;
use crate::idf::{install_espidf, EspIdf, GIT_REPOSITORY_URL};
use crate::package::{
    download_file, prepare_package, prepare_package_strip_prefix, prepare_single_binary,
};
use crate::shell::{run_command, update_env_path};
use anyhow::{bail, Result};
use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
use espflash::Chip;
use log::{debug, error, info, warn, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use std::any::Any;
use std::fs::{copy, remove_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::process::Stdio;

const DEFAULT_RUST_TOOLCHAIN_VERSION: &str = "1.63.0.2";
const DEFAULT_LLVM_VERSION: &str = "esp-14.0.0-20220415";
const DEFAULT_BUILD_TARGET: &str = "all";
const DEFAULT_ESP_IDF: &str = "";
const DEFAULT_EXTRA_CRATES: &str = "cargo-espflash";
const DEFAULT_EXTRA_TOOLS: &str = "";
const DEFAULT_NIGHTLY_VERSION: &str = "nightly";
#[cfg(windows)]
const DEFAULT_EXPORT_FILE: &str = "export-esp.bat";
#[cfg(unix)]
const DEFAULT_EXPORT_FILE: &str = "export-esp.sh";

#[derive(Debug)]
struct RustCrate {
    name: String,
    url: String,
    dist_file: String,
    dist_bin: String,
    bin: String,
}

#[derive(Debug)]
struct RustToolchain {
    arch: String,
    build_target: Vec<Chip>,
    destination_dir: String,
    esp_idf: String,
    export_file: String,
    // extra_crates: Vec<RustCrate>,
    extra_tools: String,
    llvm_url: String,
    mingw_destination_directory: String,
    mingw_dist_file: String,
    mingw_url: String,
    nightly_version: String,
    rust_dist_url: String,
    rust_installer: String,
    rust_src_dist_url: String,
    xtensa_elf_clang_file: String,
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
fn get_gcc_arch_extension(arch: &str) -> &str {
    match arch {
        "aarch64-apple-darwin" => "macos.tar.gz",
        "aarch64-unknown-linux-gnu" => "linux-arm64.tar.gz",
        "x86_64-apple-darwin" => "macos.tar.gz",
        "x86_64-unknown-linux-gnu" => "linux-amd64.tar.gz",
        "x86_64-pc-windows-msvc" => "win64.zip",
        "x86_64-pc-windows-gnu" => "win64.zip",
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
    let version: Vec<&str> = llvm_version.split('-').collect();
    let llvm_dot_version = version[1];
    llvm_dot_version.replace('.', "_")
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
        .split(',')
        .into_iter()
        .filter_map(|s| get_rust_crate(s, arch))
        .collect()
}

fn install_gcc_targets(targets: Vec<Chip>) -> Result<Vec<String>, String> {
    let mut exports: Vec<String> = Vec::new();
    for target in targets {
        match target {
            Chip::Esp32 => {
                install_gcc("xtensa-esp32-elf").unwrap();
                exports.push(format!(
                    "export PATH={}:$PATH",
                    get_tool_path("xtensa-esp32-elf/bin")
                ));
            }
            Chip::Esp32s2 => {
                install_gcc("xtensa-esp32s2-elf").unwrap();
                exports.push(format!(
                    "export PATH={}:$PATH",
                    get_tool_path("xtensa-esp32s2-elf/bin")
                ));
            }
            Chip::Esp32s3 => {
                install_gcc("xtensa-esp32s3-elf").unwrap();
                exports.push(format!(
                    "export PATH={}:$PATH",
                    get_tool_path("xtensa-esp32s3-elf/bin")
                ));
            }
            Chip::Esp32c3 => {
                install_gcc("riscv32-esp-elf").unwrap();
                exports.push(format!(
                    "export PATH={}:$PATH",
                    get_tool_path("riscv32-esp-elf/bin")
                ));
            }
            _ => {
                error!("{} Unknown target", emoji::ERROR);
            }
        }
    }
    Ok(exports)
}

fn install_gcc(gcc_target: &str) -> Result<()> {
    let gcc_path = get_tool_path(gcc_target);
    debug!("{} gcc path: {}", emoji::INFO, gcc_path);
    let gcc_file = format!(
        "{}-gcc8_4_0-esp-2021r2-patch3-{}",
        gcc_target,
        get_gcc_arch_extension(guess_host_triple::guess_host_triple().unwrap())
    );
    let gcc_dist_url = format!(
        "https://github.com/espressif/crosstool-NG/releases/download/esp-2021r2-patch3/{}",
        gcc_file
    );
    download_file(gcc_dist_url, &gcc_file, &gcc_path, true).unwrap();
    Ok(())
}

fn install_rust_nightly() -> Result<()> {
    println!("{} Installing Rust nightly toolchain", emoji::WRENCH);
    #[cfg(windows)]
    let rustup_path = format!("{}/bin/rustup.exe", get_cargo_home());
    #[cfg(unix)]
    let rustup_path = format!("{}/bin/rustup", get_cargo_home());
    let mut arguments: Vec<String> = [].to_vec();
    arguments.push("install".to_string());
    arguments.push("nightly".to_string());
    arguments.push("--profile".to_string());
    arguments.push("minimal".to_string());
    run_command(&rustup_path, arguments, "".to_string())?;
    Ok(())
}

pub fn install_riscv_target(version: &str) {
    match std::process::Command::new("rustup")
        .arg("component")
        .arg("add")
        .arg("rust-src")
        .arg("--toolchain")
        .arg(version)
        .stdout(Stdio::piped())
        .output()
    {
        Ok(child_output) => {
            let result = String::from_utf8_lossy(&child_output.stdout);
            info!(
                "{} Rust-src for RiscV target installed suscesfully: {}",
                emoji::CHECK,
                result
            );
        }
        Err(e) => {
            error!(
                "{}  Rust-src for RiscV target installation failed: {}",
                emoji::ERROR,
                e
            );
        }
    }

    match std::process::Command::new("rustup")
        .arg("target")
        .arg("add")
        .arg("--toolchain")
        .arg(version)
        .arg("riscv32imc-unknown-none-elf")
        .stdout(Stdio::piped())
        .output()
    {
        Ok(child_output) => {
            let result = String::from_utf8_lossy(&child_output.stdout);
            info!(
                "{} RiscV target installed suscesfully: {}",
                emoji::CHECK,
                result
            );
        }
        Err(e) => {
            error!("{} RiscV target installation failed: {}", emoji::ERROR, e);
        }
    }
}

pub fn install_rustup() -> Result<()> {
    #[cfg(windows)]
    let rustup_init_path = download_file(
        "https://win.rustup.rs/x86_64".to_string(),
        "rustup-init.exe",
        &get_dist_path("rustup"),
        false,
    )
    .unwrap();
    #[cfg(unix)]
    let rustup_init_path = download_file(
        "https://sh.rustup.rs".to_string(),
        "rustup-init.sh",
        &get_dist_path("rustup"),
        false,
    )
    .unwrap();
    println!("{} Installing rustup with nightly toolchain", emoji::WRENCH);
    let mut arguments: Vec<String> = [].to_vec();
    arguments.push(rustup_init_path);
    arguments.push("--default-toolchain".to_string());
    arguments.push("nightly".to_string());
    arguments.push("--profile".to_string());
    arguments.push("minimal".to_string());
    arguments.push("-y".to_string());
    run_command("/bin/bash", arguments, "".to_string())?;

    Ok(())
}

fn install_mingw(toolchain: &RustToolchain) {
    if Path::new(toolchain.mingw_destination_directory.as_str()).exists() {
        warn!(
            "{} Previous installation of MinGW exist in: {}. Please, remove the directory before new installation.",
            emoji::INFO,
            toolchain.mingw_destination_directory
        );
        return;
    }

    match prepare_package_strip_prefix(
        &toolchain.mingw_url,
        &toolchain.mingw_dist_file,
        &toolchain.mingw_destination_directory,
        "mingw64",
    ) {
        Ok(_) => {
            info!("Package ready");
        }
        Err(_e) => {
            error!("Unable to prepare package");
        }
    }
}

fn install_crate(extra_crate: &RustCrate) {
    println!("Installing crate {}", extra_crate.name);
    if extra_crate.url.is_empty() {
        // Binary crate is not available, install from source code
        let cargo_path = format!("{}/bin/cargo.exe", get_cargo_home());

        info!("{} install {}", cargo_path, extra_crate.name);
        match std::process::Command::new(cargo_path)
            .arg("install")
            .arg(&extra_crate.name)
            .stdout(Stdio::piped())
            .output()
        {
            Ok(child_output) => {
                let result = String::from_utf8_lossy(&child_output.stdout);
                info!("Crate installed: {}", result);
            }
            Err(e) => {
                error!("Crate installation failed: {}", e);
            }
        }
    } else {
        // Binary crate is available donwload it
        let tmp_path = get_tool_path(&extra_crate.name);
        match prepare_package(&extra_crate.url, &extra_crate.dist_file, &tmp_path) {
            Ok(_) => {
                let source = format!(
                    "{}/{}",
                    get_tool_path(&extra_crate.name),
                    extra_crate.dist_bin
                );
                match copy(source.clone(), &extra_crate.bin) {
                    Ok(_) => {
                        info!("Create {} installed.", extra_crate.name);
                    }
                    Err(_e) => {
                        error!(
                            "Unable to copy crate binary from {} to {}",
                            source, extra_crate.bin
                        )
                    }
                }
            }
            Err(_e) => {
                error!("Unable to unpack bianry crate {}.", extra_crate.name);
            }
        };
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
    )
    .unwrap();
    info!("Running VS BuildTools: vs_BuildTools.exe --passive --wait --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows10SDK.20348");

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
            info!("{}", result);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}

fn install_rust_toolchain(toolchain: &RustToolchain) -> Result<()> {
    match std::process::Command::new("rustup")
        .arg("toolchain")
        .arg("list")
        .stdout(Stdio::piped())
        .output()
    {
        Ok(child_output) => {
            let result = String::from_utf8_lossy(&child_output.stdout);
            if !result.contains("nightly") {
                warn!("{} Rust nightly toolchain not found", emoji::WARN);
                install_rust_nightly()?;
            }
        }
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                warn!("{} rustup was not found.", emoji::WARN);
                install_rustup()?;
            } else {
                bail!("{} Error: {}", emoji::ERROR, e);
            }
        }
    }

    if Path::new(toolchain.destination_dir.as_str()).exists() {
        println!(
            "{} Previous installation of Rust Toolchain exist in: {}.\n Please, remove the directory before new installation.",
            emoji::INFO,
            toolchain.destination_dir
        );
        return Ok(());
    } else {
        // Some platfroms like Windows are available in single bundle rust + src, because install
        // script in dist is not available for the plaform. It's sufficient to extract the toolchain
        if toolchain.rust_installer.is_empty() {
            download_file(
                toolchain.rust_dist_url.clone(),
                "rust.zip",
                &toolchain.destination_dir,
                true,
            )
            .unwrap();
        } else {
            download_file(
                toolchain.rust_dist_url.clone(),
                "rust.tar.xz",
                &get_dist_path(""),
                true,
            )
            .unwrap();

            println!("{} Installing rust", emoji::WRENCH);
            let mut arguments: Vec<String> = [].to_vec();
            arguments.push("-c".to_string());
            arguments.push(format!(
                "{}/rust-nightly-{}/install.sh --destdir={} --prefix='' --without=rust-docs",
                get_dist_path(""),
                toolchain.arch,
                toolchain.destination_dir
            ));
            run_command("/bin/bash", arguments, "".to_string())?;

            download_file(
                toolchain.rust_src_dist_url.clone(),
                "rust-src.tar.xz",
                &get_dist_path(""),
                true,
            )
            .unwrap();

            println!("{} Installing rust-src", emoji::WRENCH);
            let mut arguments: Vec<String> = [].to_vec();
            arguments.push("-c".to_string());
            arguments.push(format!(
                "{}/rust-src-nightly/install.sh --destdir={} --prefix='' --without=rust-docs",
                get_dist_path(""),
                toolchain.destination_dir
            ));
            run_command("/bin/bash", arguments, "".to_string())?;
        }
    }

    if Path::new(toolchain.xtensa_elf_clang_file.as_str()).exists() {
        println!(
            "{} Previous installation of LLVM exist in: {}.\n Please, remove the directory before new installation.",
            emoji::WARN,
            &toolchain.xtensa_elf_clang_file
        );
    } else {
        #[cfg(windows)]
        let file_name = "xtensa-esp32-elf-llvm.zip";
        #[cfg(unix)]
        let file_name = "xtensa-esp32-elf-llvm.tar.xz";
        download_file(
            toolchain.llvm_url.clone(),
            file_name,
            &toolchain.xtensa_elf_clang_file,
            true,
        )
        .unwrap();
    }

    let mut exports: Vec<String> = Vec::new();
    let libclang_path = format!("{}/lib", get_tool_path("xtensa-esp32-elf-clang"));
    #[cfg(unix)]
    exports.push(format!("export LIBCLANG_PATH=\"{}\"", &libclang_path));
    #[cfg(windows)]
    exports.push(format!("set \"LIBCLANG_PATH={}\"", &libclang_path));

    if toolchain.build_target.contains(&Chip::Esp32c3) {
        println!("{} Installing riscv target", emoji::WRENCH);
        install_riscv_target(&toolchain.nightly_version);
    }

    if !toolchain.esp_idf.is_empty() {
        println!("{} Installing ESP-IDF", emoji::WRENCH);
        let build_target = parse_idf_targets(toolchain.build_target.clone()).unwrap();
        let espidf = EspIdf {
            build_target,
            minified: true,
            path: get_tools_path(),
            repository_url: GIT_REPOSITORY_URL.to_string(),
            version: toolchain.esp_idf.clone(),
        };
        install_espidf(&espidf)?;
        exports.push(format!("export IDF_TOOLS_PATH=\"{}\"", get_tools_path()));
        exports.push(format!(
            ". {}/export.sh",
            get_esp_idf_directory(&espidf.version)
        ));
        // let ldproxy = get_rust_crate("ldproxy", &toolchain.arch).unwrap();
        // install_crate(&ldproxy);
    } else {
        warn!("{} No esp-idf version provided.", emoji::WARN);
        println!("{} Installing gcc for targets", emoji::WRENCH);
        exports.extend(
            install_gcc_targets(toolchain.build_target.clone())
                .unwrap()
                .iter()
                .cloned(),
        );
    }

    // Install additional dependencies specific for the host
    // for extra_tool in toolchain.extra_tools
    match toolchain.extra_tools.as_str() {
        "mingw" => match toolchain.arch.as_str() {
            "x86_64-pc-windows-gnu" => {
                install_mingw(toolchain);
                update_env_path(format!("{}/bin", toolchain.mingw_destination_directory).as_str());
            }
            _ => {
                info!("Ok");
            }
        },
        "vctools" => {
            install_vctools();
            update_env_path("C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Tools\\MSVC\\14.33.31629\\bin\\Hostx64\\x64");
        }
        _ => {
            info!("{} No extra tools selected", emoji::INFO);
        }
    }

    // for extra_crate in toolchain.extra_crates.iter() {
    //     install_crate(extra_crate);
    // }

    println!("{} Updating environment variables:", emoji::DIAMOND);
    for e in exports.iter() {
        println!("{}", e);
    }
    println!(
        "{} Creating export file at {}",
        emoji::INFO,
        &toolchain.export_file
    );
    let mut file = File::create(&toolchain.export_file)?;
    for e in exports.iter() {
        file.write_all(e.as_bytes())?;
        file.write_all(b"\n")?;
    }
    println!("{} Installation succeeded", emoji::CHECK);

    Ok(())
}

fn uninstall_rust_toolchain(toolchain: &RustToolchain) {
    if Path::new(toolchain.destination_dir.as_str()).exists() {
        println!("Removing: {}", toolchain.destination_dir);
        match remove_dir_all(&toolchain.destination_dir) {
            Ok(_) => {
                info!("Removed.");
            }
            Err(_e) => {
                error!("Failed to remove.");
            }
        }
    }

    if Path::new(toolchain.xtensa_elf_clang_file.as_str()).exists() {
        println!("Removing: {}", toolchain.xtensa_elf_clang_file);
        match remove_dir_all(&toolchain.xtensa_elf_clang_file) {
            Ok(_) => {
                info!("Removed.");
            }
            Err(_e) => {
                error!("Failed to remove.");
            }
        }
    }
}

fn get_rust_toolchain(matches: &clap::ArgMatches<'_>) -> RustToolchain {
    let build_target = matches.value_of("build-target").unwrap();
    let targets: Vec<Chip> = parse_targets(build_target).unwrap();
    let arch = matches.value_of("default-host").unwrap();
    let esp_idf = matches.value_of("esp-idf").unwrap();
    let export_file = matches.value_of("export-file").unwrap();
    let extra_crates = matches.value_of("extra-crates").unwrap();
    let extra_tools = matches.value_of("extra-tools").unwrap();
    let llvm_version = matches.value_of("llvm-version").unwrap();
    let nightly_version = matches.value_of("nightly-version").unwrap();
    let toolchain_version = matches.value_of("toolchain-version").unwrap();
    let artifact_file_extension = get_artifact_file_extension(arch).to_string();
    let llvm_arch = get_llvm_arch(arch).to_string();
    let llvm_file = format!(
        "xtensa-esp32-elf-llvm{}-{}-{}.{}",
        get_llvm_version_with_underscores(llvm_version),
        llvm_version,
        llvm_arch,
        artifact_file_extension
    );
    let llvm_url = format!(
        "https://github.com/espressif/llvm-project/releases/download/{}/{}",
        llvm_version, llvm_file
    );
    let rust_dist = format!("rust-{}-{}", toolchain_version, arch);
    let rust_src_dist = format!("rust-src-{}", toolchain_version);
    let rust_dist_file = format!("{}.{}", rust_dist, artifact_file_extension);
    let rust_src_dist_file = format!("{}.{}", rust_src_dist, artifact_file_extension);
    let rust_dist_url = format!(
        "https://github.com/esp-rs/rust-build/releases/download/v{}/{}",
        toolchain_version, rust_dist_file
    );
    let rust_src_dist_url = format!(
        "https://github.com/esp-rs/rust-build/releases/download/v{}/{}",
        toolchain_version, rust_src_dist_file
    );
    let xtensa_elf_clang_file = format!(
        "{}/{}-{}",
        get_tool_path("xtensa-esp32-elf-clang"),
        llvm_version,
        arch
    );
    let mingw_release = "x86_64-12.1.0-release-posix-seh-rt_v10-rev3".to_string();
    // Temporal solution - repackaging 7z to zip, because Rust based decompression crate does not have BCJ support: https://github.com/dyz1990/sevenz-rust/issues/1
    // let mingw_dist_file = format!("{}.zip", mingw_release);
    // let mingw_url = format!(
    //     "https://github.com/esp-rs/rust-build/releases/download/mingw-12/{}",
    //     mingw_dist_file
    // );
    // Final solution - TO BE TESTED
    let mingw_dist_file = format!("{}.7z", mingw_release);
    let mingw_url = format!(
        "https://github.com/niXman/mingw-builds-binaries/releases/download/12.1.0-rt_v10-rev3/{}",
        mingw_dist_file
    );
    let mingw_destination_directory = format!("{}/{}", get_tool_path("mingw"), mingw_release);

    RustToolchain {
        arch: arch.to_string(),
        build_target: targets,
        destination_dir: format!("{}/.rustup/toolchains/esp", get_home_dir()),
        esp_idf: esp_idf.to_string(),
        export_file: export_file.to_string(),
        // extra_crates: get_extra_crates(extra_crates, arch),
        extra_tools: extra_tools.to_string(),
        llvm_url,
        mingw_destination_directory,
        mingw_dist_file,
        mingw_url,
        nightly_version: nightly_version.to_string(),
        rust_dist_url,
        rust_installer: get_rust_installer(arch).to_string(),
        rust_src_dist_url,
        xtensa_elf_clang_file,
    }
}

fn get_install_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
    // Setup logging
    let log_level = matches.value_of("log-level").unwrap();
    let mut log_config = ConfigBuilder::new();
    log_config.set_location_level(LevelFilter::Off);
    TermLogger::init(
        get_log_level(log_level),
        log_config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    let toolchain = get_rust_toolchain(matches);
    println!( "{} Installing esp Rust toolchan",emoji::DISC,);
    info!("{} Esp Rust Toolchain arguments: {:#?}",
        emoji::DIAMOND,
        toolchain);
    install_rust_toolchain(&toolchain).unwrap();
    Ok(())
}

fn get_reinstall_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
    let toolchain = get_rust_toolchain(matches);

    uninstall_rust_toolchain(&toolchain);
    install_rust_toolchain(&toolchain).unwrap();
    Ok(())
}

fn get_uninstall_runner(
    _args: &str,
    matches: &clap::ArgMatches<'_>,
) -> std::result::Result<(), clap::Error> {
    let toolchain = get_rust_toolchain(matches);

    uninstall_rust_toolchain(&toolchain);
    Ok(())
}

pub fn get_install_cmd<'a>() -> Command<'a, str> {
    Command::new("install")
        .description("Install Rust environment for Xtensa")
        .options(|app| {
            app.arg(
                Arg::with_name("build-target")
                    .short("b")
                    .long("build-target")
                    .help("Comma or space separated list of targets [esp32,esp32s2,esp32s3,esp32c3,all].")
                    .takes_value(true)
                    .default_value(DEFAULT_BUILD_TARGET),
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
                Arg::with_name("esp-idf")
                    .short("e")
                    .long("esp-idf")
                    .help("ESP-IDF branch to install. If empty, no esp-idf is installed.")
                    .takes_value(true)
                    .default_value(DEFAULT_ESP_IDF),
            )
            .arg(
                Arg::with_name("export-file")
                    .short("f")
                    .long("export-file")
                    .help("Destination of the export file generated")
                    .takes_value(true)
                    .default_value(DEFAULT_EXPORT_FILE),
            )
            .arg(
                Arg::with_name("extra-crates")
                    .short("c")
                    .long("extra-crates")
                    .help("Extra crates which should be deployed. E.g. cargo-espflash")
                    .takes_value(true)
                    .default_value(DEFAULT_EXTRA_CRATES),
            )
            .arg(
                Arg::with_name("extra-tools")
                    .short("t")
                    .long("extra-tools")
                    .help("Extra tools which should be deployed. E.g. MinGW")
                    .possible_values(&["vctools", "mingw", ""])
                    .takes_value(true)
                    .default_value(DEFAULT_EXTRA_TOOLS),
            )
            .arg(
                Arg::with_name("llvm-version")
                    .short("l")
                    .long("llvm-version")
                    .help("Version of LLVM with Xtensa support")
                    .possible_values(&["esp-13.0.0-20211203", "esp-14.0.0-20220415"])
                    .takes_value(true)
                    .default_value(DEFAULT_LLVM_VERSION),
            )
            .arg(
                Arg::with_name("log-level")
                    .short("g")
                    .help("Log level for the installation process")
                    .long("log-level")
                    .possible_values(&["debug", "info", "warn", "error", "off"])
                    .takes_value(true)
                    .default_value("info"),
            )
            .arg(
                Arg::with_name("nightly-version")
                    .short("n")
                    .long("nightly-version")
                    .help("Nightly Rust toolchain version.")
                    .takes_value(true)
                    .default_value(DEFAULT_NIGHTLY_VERSION),
            )
            .arg(
                Arg::with_name("toolchain-version")
                    .short("v")
                    .long("toolchain-version")
                    .help("Version of Rust toolchain")
                    .takes_value(true)
                    .default_value(DEFAULT_RUST_TOOLCHAIN_VERSION),
            )
        })
        .runner(get_install_runner)
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
        .runner(get_reinstall_runner)
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
        .runner(get_uninstall_runner)
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_install_cmd())
        .add_cmd(get_reinstall_cmd())
        .add_cmd(get_uninstall_cmd())
        .into_cmd("rust")
        // Optionally specify a description
        .description("Maintain Rust environment for Xtensa.");

    multi_cmd
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
