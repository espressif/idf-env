
use idf_env_core::rust::{RustToolchain, build_rust_toolchain, install_rust_toolchain, uninstall_rust_toolchain};

pub fn install_rust() {
    let triple = guess_host_triple::guess_host_triple().unwrap();

    let toolchain_version = "1.57.0.2";
    let llvm_version = "esp-13.0.0-20211203";

    let toolchain = build_rust_toolchain(
        toolchain_version,
        llvm_version,
        triple);
    install_rust_toolchain(&toolchain);
}