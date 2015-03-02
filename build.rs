#![feature(io, env)]

use std::old_io::Command;

fn main() {
  Command::new("make").status().unwrap();
  println!("cargo:rustc-flags=-L hoedown -l hoedown");
}
