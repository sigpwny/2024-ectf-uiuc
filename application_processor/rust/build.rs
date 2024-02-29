use std::env;
use std::path::PathBuf;

fn main() {
    // Link the POST_BOOT C code
    let current_dir = std::env::current_dir().unwrap();
    let parent_dir = current_dir.parent().unwrap();
    let build_dir = parent_dir.join("c").join("build");
    // let lib_path = build_dir.join("libpostboot.a");
    // if !lib_path.exists() {
    //     panic!("libpostboot.a does not exist! Please build the C code first.");
    // }
    println!("cargo:rustc-link-search={}", build_dir.display());
    // println!("cargo:rustc-link-lib=dylib=postbootandlibc");
    println!("cargo:rustc-link-lib=static=postboot");
    // println!("cargo:rustc-link-lib=static=msdk-periph");
    // println!("cargo:rustc-link-lib=static=msdk-board");
    // Re-run build if the C code changes
    // println!("cargo:rerun-if-changed={}", lib_path.display());
    // println!("cargo:rerun-if-changed=build.rs");
    // Force a rebuild everytime, LOL
    println!("cargo:rerun-if-changed=NULL");
    // Link ARM Embedded GCC standard C library
    // the path is stored in environment variable GCC_ARM_PATH
    // let gcc_arm_path = env::var("GCC_ARM_PATH").unwrap();
    // let libc_path = PathBuf::from(gcc_arm_path).join("arm-none-eabi").join("lib").join("thumb").join("v7e-m+fp").join("softfp");
    // let libc_lib_path = libc_path.join("libc.a");
    // println!("cargo:rerun-if-changed={}", libc_lib_path.display());
    // if !libc_lib_path.exists() {
    //     panic!("libc.a does not exist! Please set the GCC_ARM_PATH environment variable.");
    // }
    // println!("cargo:rustc-link-search={}", libc_path.display());
    // println!("cargo:rustc-link-lib=static=c");
}