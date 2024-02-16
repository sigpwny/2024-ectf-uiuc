use std::env;
use std::path::PathBuf;

fn main() {
    let maxim_path = env::var("MAXIM_PATH").unwrap();
    let gcc_arm_path = env::var("GCC_ARM_PATH").unwrap();
    let out_path = "src";
    // Compile MSDK via makefile located in ./msdk and print output as it runs
    std::process::Command::new("make")
        .current_dir("./msdk")
        .output()
        .expect("Failed to compile MSDK");
    // Provide compiled MSDK object files to Rust linker
    println!("cargo:rustc-link-search=./msdk/build");
    // Create Rust bindings
    let bindings = bindgen::Builder::default()
        .header("./msdk/wrapper.h")
        .clang_arg(format!("-I{}/Libraries/Boards/MAX78000/FTHR_RevA/Include/", maxim_path))
        .clang_arg(format!("-I{}/Libraries/CMSIS/5.9.0/Core/Include/", maxim_path))
        .clang_arg(format!("-I{}/Libraries/CMSIS/Device/Maxim/MAX78000/Include/", maxim_path))
        .clang_arg(format!("-I{}/Libraries/MiscDrivers/Display/", maxim_path))
        .clang_arg(format!("-I{}/Libraries/MiscDrivers/LED/", maxim_path))
        .clang_arg(format!("-I{}/Libraries/MiscDrivers/PushButton/", maxim_path))
        .clang_arg(format!("-I{}/Libraries/MiscDrivers/Touchscreen/", maxim_path))
        .clang_arg(format!("-I{}/Libraries/PeriphDrivers/Include/MAX78000/", maxim_path))
        .clang_arg(format!("-I{}/arm-none-eabi/include/", gcc_arm_path))
        .clang_arg("--target=arm-none-eabi")
        .clang_arg("-DTARGET=MAX78000")
        .clang_arg("-DTARGET_UC=MAX78000")
        .clang_arg("-DTARGET_LC=max78000")
        .clang_arg("-DBOARD=FTHR_RevA")
        .clang_arg("-DMFLOAT_ABI=soft")
        .clang_arg("-DTARGET_REV=0x4131")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(PathBuf::from(out_path).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}