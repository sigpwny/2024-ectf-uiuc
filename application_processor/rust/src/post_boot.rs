// Use post_boot() function from C code
extern "C" {
    pub fn post_boot() -> !;
}

// TODO: Provide functions that the post_boot code can call