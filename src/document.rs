use bridge::*;
use buffer::Buffer;
use std::mem::transmute;
use std::marker;

pub trait Renderer {
  unsafe fn hoedown_renderer(&self) -> *const HoedownRenderer;
}

pub struct Document<'r> {
  doc: *mut HoedownDocument,

  // HoedownDocument internally contains a reference to
  // Renderer. Let the borrow checker know.
  _marker: marker::PhantomData<&'r ()>
}

impl<'_> Document<'_> {
  pub fn new<'r>(renderer: &'r Renderer) -> Document<'r> {
    Document::with_ext(renderer, Extensions::empty())
  }

  pub fn with_ext<'r>(renderer: &'r Renderer, extensions: Extensions) -> Document<'r> {
    unsafe {
      let r = renderer.hoedown_renderer();
      let doc = hoedown_document_new(r, extensions.bits(), 32);
      Document { doc: doc, _marker: marker::PhantomData }
    }
  }

  pub fn render_into(&mut self, input: &str, buffer: &mut Buffer) {
    unsafe {
      hoedown_document_render(self.doc, buffer.as_mut_ptr(), transmute(input.as_ptr()), transmute(input.len()))
    }
  }

  pub fn render_inline_into(&mut self, input: &str, buffer: &mut Buffer) {
    unsafe {
      hoedown_document_render_inline(self.doc, buffer.as_mut_ptr(), transmute(input.as_ptr()), transmute(input.len()))
    }
  }

  pub fn render(&mut self, input: &str) -> Buffer {
    let mut buffer = Buffer::new();
    self.render_into(input, &mut buffer);
    buffer
  }

  pub fn render_inline(&mut self, input: &str) -> Buffer {
    let mut buffer = Buffer::new();
    self.render_inline_into(input, &mut buffer);
    buffer
  }
}
