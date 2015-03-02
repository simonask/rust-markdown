#![allow(dead_code)]

use bridge::*;
use std::mem::transmute;
use std::str::from_utf8_unchecked;
use std::ffi::CStr;

pub struct Buffer {
  buf: *mut HoedownBuffer
}

impl Buffer {
  pub fn new() -> Buffer {
    unsafe {
      Buffer { buf: hoedown_buffer_new(1024) }
    }
  }

  pub fn as_ptr(&self) -> *const HoedownBuffer {
    self.buf
  }

  pub fn as_mut_ptr(&mut self) -> *mut HoedownBuffer {
    self.buf
  }

  pub fn reset(&mut self) {
    unsafe { hoedown_buffer_reset(self.as_mut_ptr()) }
  }

  pub fn grow(&mut self, neosz: usize) {
    unsafe { hoedown_buffer_grow(self.as_mut_ptr(), transmute(neosz)) }
  }

  pub fn put(&mut self, data: &[i8]) {
    unsafe {
      hoedown_buffer_put(self.as_mut_ptr(), data.as_ptr(), transmute(data.len()))
    }
  }

  pub fn puts(&mut self, data: &str) {
    unsafe {
      hoedown_buffer_put(self.as_mut_ptr(), transmute(data.as_ptr()), transmute(data.len()))
    }
  }

  pub fn putc(&mut self, c: i8) {
    unsafe {
      hoedown_buffer_putc(self.as_mut_ptr(), c)
    }
  }

  pub fn set(&mut self, data: &[i8]) {
    unsafe {
      hoedown_buffer_set(self.as_mut_ptr(), data.as_ptr(), transmute(data.len()))
    }
  }

  pub fn sets(&mut self, data: &str) {
    unsafe {
      hoedown_buffer_set(self.as_mut_ptr(), transmute(data.as_ptr()), transmute(data.len()))
    }
  }

  pub fn eqs(&self, other: &str) -> bool {
    unsafe {
      hoedown_buffer_eq(self.as_ptr(), transmute(other.as_ptr()), transmute(other.len())) != 0
    }
  }

  pub fn slurp(&mut self, size: usize) {
    unsafe {
      hoedown_buffer_slurp(self.as_mut_ptr(), transmute(size))
    }
  }

  pub fn as_str<'a>(&'a self) -> &'a str {
    unsafe {
      let cstr_data = hoedown_buffer_cstr(transmute(self.as_ptr()));
      let cstr = CStr::from_ptr(transmute(cstr_data));
      from_utf8_unchecked(cstr.to_bytes())
    }
  }

  pub fn to_string(&self) -> String {
    String::from_str(self.as_str())
  }
}

impl Drop for Buffer {
  fn drop(&mut self) {
    unsafe {
      hoedown_buffer_free(self.as_mut_ptr())
    }
  }
}
