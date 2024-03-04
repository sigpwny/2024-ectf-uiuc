fn main() {
    // Link the POST_BOOT C code
    let current_dir = std::env::current_dir().unwrap();
    let parent_dir = current_dir.parent().unwrap();
    let build_dir = parent_dir.join("c").join("build");
    let lib_path = build_dir.join("libpostboot.a");
    if !lib_path.exists() {
        panic!("libpostboot.a does not exist! Please build the C POST_BOOT code first.");
    }
    println!("cargo:rustc-link-search={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=postboot");
    // Force a rebuild every time (lol)
    println!("cargo:rerun-if-changed=NULL");
}