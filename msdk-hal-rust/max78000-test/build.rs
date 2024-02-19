fn main() {
    // Get current directory
    let current_dir = std::env::current_dir().unwrap();
    // Get the path to the MSDK
    let msdk_path = current_dir.join("msdk").join("build");
    // Check that libmsdk.a exists
    let mut libmsdk_path = msdk_path.join("libmsdk.a");
    if !libmsdk_path.exists() {
        panic!("libmsdk.a does not exist");
    }
    // Copy to target/debug/deps
    let target_dir = current_dir.join("target").join("debug").join("deps");
    std::fs::copy(libmsdk_path, target_dir.join("libmsdk.a")).unwrap();
    // Copy to target/thumbv7em-none-eabihf/debug/deps
    libmsdk_path = msdk_path.join("libmsdk.a");
    let target_dir = current_dir.join("target").join("thumbv7em-none-eabihf").join("debug").join("deps");
    std::fs::copy(libmsdk_path, target_dir.join("libmsdk.a")).unwrap();
    // Provide compiled MSDK object files to Rust linker
    println!("cargo:rustc-link-search={}", msdk_path.display());
    println!("cargo:rustc-link-lib=msdk");
    println!("cargo:rustc-link-lib=static=msdk");
}