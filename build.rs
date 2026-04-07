use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        fs::write(out.join("memory.x"), include_bytes!("memory.x")).unwrap();
        fs::write(out.join("device.x"), include_bytes!("device.x")).unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=memory.x");
        println!("cargo:rerun-if-changed=device.x");
        println!("cargo:rustc-env=RISCV_RT_BASE_ISA=rv64imac");
        println!("cargo:rerun-if-env-changed=RISCV_RT_BASE_ISA");
        println!("cargo:rustc-env=RISCV_MTVEC_ALIGN=64");
        println!("cargo:rerun-if-env-changed=RISCV_MTVEC_ALIGN");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
