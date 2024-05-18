//! The build script sets the linker flags to tell it which link script to use.

fn main() {
    // Specify linker arguments.
    println!("cargo:rustc-link-arg-bins=-Tlinkall.x");
}
