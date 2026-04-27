use std::io::Write;
use std::path::PathBuf;

fn create_el2_entry_bin() -> PathBuf {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let project_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let toolchain = project_dir.join("..").join("..").join("toolchain");

    let obj = out_dir.join("el2_entry.o");
    let src = project_dir.join("src").join("el2").join("entry.S");
    let cc = toolchain.join("clang.sh");
    let status = std::process::Command::new(cc)
        .args(["-target", "aarch64-none-elf", "-march=morello"])
        .arg("-c")
        .arg(src)
        .arg("-o")
        .arg(&obj)
        .status()
        .expect("failed to invoke clang");
    assert!(status.success(), "clang failed");

    let elf = out_dir.join("el2_entry.elf");
    let linker_script = project_dir.join("el2.ld");
    let ld = toolchain.join("ld.sh");
    let status = std::process::Command::new(ld)
        .arg("-T")
        .arg(linker_script)
        .arg("-o")
        .arg(&elf)
        .arg(obj)
        .status()
        .expect("failed to invoke ld");
    assert!(status.success(), "ld failed");

    let bin = out_dir.join("el2_entry.bin");
    let objcopy = toolchain.join("objcopy.sh");
    let status = std::process::Command::new(objcopy)
        .args(["-O", "binary"])
        .arg(elf)
        .arg(&bin)
        .status()
        .expect("failed to invoke objcopy");
    assert!(status.success(), "objcopy failed");

    bin
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/el2/entry.S");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let el2_bin = create_el2_entry_bin();

    println!(
        "cargo:rustc-env=EL2_ENTRY_BIN={}",
        el2_bin.to_str().unwrap()
    );

    std::fs::File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-arg=-Tlink.x");
}
