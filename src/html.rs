use bridge::*;
use document::Renderer;

pub struct HtmlRenderer {
  renderer: *mut HoedownRenderer
}

impl HtmlRenderer {
  pub fn new() -> HtmlRenderer {
    HtmlRenderer::with_flags(ESCAPE)
  }

  pub fn new_toc() -> HtmlRenderer {
    unsafe {
      let r = hoedown_html_toc_renderer_new(32);
      HtmlRenderer { renderer: r }
    }
  }

  pub fn with_flags(flags: HtmlFlags) -> HtmlRenderer {
    unsafe {
      let r = hoedown_html_renderer_new(flags.bits(), 32);
      HtmlRenderer { renderer: r }
    }
  }
}

impl Renderer for HtmlRenderer {
  fn hoedown_renderer(&self) -> *const HoedownRenderer {
    self.renderer
  }
}

impl Drop for HtmlRenderer {
  fn drop(&mut self) {
    unsafe {
      hoedown_html_renderer_free(self.renderer)
    }
  }
}

