// Generated by gir (https://github.com/gtk-rs/gir @ bb52094f1b0d)
// from /home/rene/git/gir-files (@ 0831511aed59)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}