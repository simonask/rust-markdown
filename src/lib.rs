#![feature(libc, std_misc, rustc_private, collections)]
#[macro_use] extern crate rustc_bitflags;

extern crate libc;
use libc::{c_int};

mod buffer;
mod bridge;
mod document;
mod html;
pub use buffer::{Buffer};
pub use document::{Document, Renderer};
pub use html::HtmlRenderer;

pub fn library_version() -> (i32, i32, i32) {
  let mut major: c_int = -1;
  let mut minor: c_int = -1;
  let mut revision: c_int = -1;
  unsafe {
    bridge::hoedown_version(&mut major, &mut minor, &mut revision);
  }
  (major, minor, revision)
}

#[test]
fn test_library_version() {
  let (major, minor, revision) = library_version();
  assert!(major >= 0);
  assert!(minor >= 0);
  assert!(revision >= 0);
}

#[test]
fn test_render_to_html() {
  let renderer = HtmlRenderer::new();
  let mut document = Document::new(&renderer);
  let rendered = document.render("Hello, World!");
  let result = rendered.to_string();
  assert_eq!("<p>Hello, World!</p>\n", result);
}
