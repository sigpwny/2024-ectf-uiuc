use std::env;
use std::path::Path;

fn main() {
  // Get current directory
  let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  // Get the path to Ascon
  let dir_src_c = Path::new(&dir).join("src_c");
  // Make the Ascon C library
  std::process::Command::new("make")
      .current_dir(dir_src_c)
      .status()
      .expect("Failed to run make");
  // Get the build directory
  let dir_build = Path::new(&dir).join("src_c").join("build");
  // Check that libascon.a exists
  let lib_path = dir_build.join("libascon.a");
  if !lib_path.exists() {
      panic!("libascon.a does not exist");
  }
  // Provide compiled Ascon object files to Rust linker
  println!("cargo:rustc-link-search={}", dir_build.display());
  println!("cargo:rustc-link-lib=ascon");
}