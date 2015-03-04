#![feature(process)]

use std::process::Command;

fn main() {
  Command::new("make")
    .args(&["libhoedown.a", "-C", "hoedown"])
    .status().unwrap();
  println!("cargo:rustc-flags=-L native=hoedown");
}
