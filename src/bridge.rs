#![allow(dead_code)]

use libc::{c_int, c_uint, c_char, c_void, size_t};

type HoedownReallocCallback = fn(*mut c_void, size_t);
type HoedownFreeCallback = fn(*mut c_void);

#[repr(C)]
pub struct HoedownBuffer;

#[repr(C)]
pub struct HoedownDocument;

#[repr(C)]
pub struct HoedownRendererData {
  opaque: *mut c_void
}

bitflags! {
  flags Extensions: c_int {
    const TABLES = (1 << 0),
    const FENCED_CODE = (1 << 1),
    const FOOTNOTES = (1 << 2),
    const AUTOLINK = (1 << 3),
    const STRIKETHROUGH = (1 << 4),
    const UNDERLINE = (1 << 5),
    const HIGHLIGHT = (1 << 6),
    const QUOTE = (1 << 7),
    const SUPERSCRIPT = (1 << 8),
    const MATH = (1 << 9),
    const NO_INTRA_EMPHASIS = (1 << 11),
    const SPACE_HEADERS = (1 << 12),
    const MATH_EXPLICIT = (1 << 13),
    const DISABLE_INDENTED_CODE = (1 << 14)
  }
}

bitflags! {
  flags TableFlags: c_int {
    const ALIGN_LEFT = 1,
    const ALIGN_RIGHT = 2,
    const ALIGN_CENTER = 3,
    const ALIGNMASK = 3,
    const HEADER = 4
  }
}

bitflags! {
  flags ListFlags: c_int {
    const LIST_ORDERED = (1 << 0),
    const LI_BLOCK = (1 << 1) /* <li> containing block data */
  }
}

bitflags! {
  flags AutoLinkType: c_int {
    const NONE = 0,
    const NORMAL = 1,
    const EMAIL = 2,
  }
}

bitflags! {
  flags HtmlFlags: c_int {
    const SKIP_HTML = (1 << 0),
    const ESCAPE = (1 << 1),
    const HARD_WRAP = (1 << 2),
    const USE_XHTML = (1 << 3)
  }
}

#[repr(C)]
enum HtmlTag {
  TagNone = 0,
  TagOpen = 1,
  TagClose = 2,
}

#[repr(C)]
pub struct HoedownRenderer {
  pub opaque: *mut c_void,
  pub blockcode: fn(ob: *mut HoedownBuffer, text: *const HoedownBuffer, lang: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub blockquote: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub header: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, level: c_int, data: *const HoedownRendererData) -> c_void,
  pub hrule: fn(ob: *mut HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub list: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, list_flags: c_int, data: *const HoedownRendererData) -> c_void,
  pub listitem: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, list_flags: c_int, data: *const HoedownRendererData) -> c_void,
  pub paragraph: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub table: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub table_header: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub table_body: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub table_row: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub table_cell: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, table_flags: c_int, data: *const HoedownRendererData) -> c_void,
  pub footnotes: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub footnote_def: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, num: c_uint, data: *const HoedownRendererData) -> c_void,
  pub blockhtml: fn(ob: *mut HoedownBuffer, text: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub autolink: fn(ob: *mut HoedownBuffer, link: *const HoedownBuffer, autolink_type: c_int, data: *const HoedownRendererData) -> c_int,
  pub codespan: fn(ob: *mut HoedownBuffer, text: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub double_emphasis: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub emphasis: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub underline: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub highlight: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub quote: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub image: fn(ob: *mut HoedownBuffer, link: *const HoedownBuffer, title: *const HoedownBuffer, alt: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub linebreak: fn(ob: *mut HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub link: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, link: *const HoedownBuffer, title: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub triple_emphasis: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub strikethrough: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub superscript: fn(ob: *mut HoedownBuffer, content: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub footnote_ref: fn(ob: *mut HoedownBuffer, num: c_uint, data: *const HoedownRendererData) -> c_int,
  pub math: fn(ob: *mut HoedownBuffer, text: *const HoedownBuffer, displaymode: c_int, data: *const HoedownRendererData) -> c_int,
  pub raw_html: fn(ob: *mut HoedownBuffer, text: *const HoedownBuffer, data: *const HoedownRendererData) -> c_int,
  pub entity: fn(ob: *mut HoedownBuffer, text: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub normal_text: fn(ob: *mut HoedownBuffer, text: *const HoedownBuffer, data: *const HoedownRendererData) -> c_void,
  pub doc_header: fn(ob: *mut HoedownBuffer, inline_render: c_int, data: *const HoedownRendererData) -> c_void,
  pub doc_footer: fn(ob: *mut HoedownBuffer, inline_render: c_int, data: *const HoedownRendererData) -> c_void,
}

#[link(name = "hoedown", kind = "static")]
extern {
  // autolink.h
  pub fn hoedown_autolink_is_safe(data: *const u8, size: size_t) -> c_int;
  pub fn hoedown_autolink__www(rewind_p: *mut size_t, link: *mut HoedownBuffer,
    data: *mut u8, offset: size_t, size: size_t, flags: c_int);
  pub fn hoedown_autolink__email(rewind_p: *mut size_t, link: *mut HoedownBuffer,
    data: *mut u8, offset: size_t, size: size_t, flags: c_int);
  pub fn hoedown_autolink__url(rewind_p: *mut size_t, link: *mut HoedownBuffer,
    data: *mut u8, offset: size_t, size: size_t, flags: c_int);

  // buffer.h
  pub fn hoedown_buffer_init(
    buffer: *mut HoedownBuffer,
    unit: size_t,
    data_realloc: HoedownReallocCallback,
    data_free: HoedownFreeCallback,
    buffer_free: HoedownFreeCallback
  );
  pub fn hoedown_buffer_new(unit: size_t) -> *mut HoedownBuffer;
  pub fn hoedown_buffer_reset(buf: *mut HoedownBuffer);
  pub fn hoedown_buffer_grow(buf: *mut HoedownBuffer, neosz: size_t);
  pub fn hoedown_buffer_put(buf: *mut HoedownBuffer, data: *const c_char, size: size_t);
  pub fn hoedown_buffer_puts(buf: *mut HoedownBuffer, s: *const c_char);
  pub fn hoedown_buffer_putc(buf: *mut HoedownBuffer, c: c_char);
  pub fn hoedown_buffer_set(buf: *mut HoedownBuffer, data: *const c_char, size: size_t);
  pub fn hoedown_buffer_sets(buf: *mut HoedownBuffer, s: *const c_char);
  pub fn hoedown_buffer_eq(buf: *const HoedownBuffer, data: *const c_char, size: size_t) -> c_int;
  pub fn hoedown_buffer_eqs(buf: *const HoedownBuffer, s: *const c_char) -> c_int;
  pub fn hoedown_buffer_prefix(buf: *const HoedownBuffer, prefix: *const c_char) -> c_int;
  pub fn hoedown_buffer_slurp(buf: *mut HoedownBuffer, size: size_t);
  pub fn hoedown_buffer_cstr(buf: *mut HoedownBuffer) -> *const c_char;
  // omitted: hoedown_buffer_printf
  pub fn hoedown_buffer_free(buf: *mut HoedownBuffer);

  // document.h
  pub fn hoedown_document_new(
    renderer: *const HoedownRenderer,
    extensions: c_int,
    max_nesting: size_t
  ) -> *mut HoedownDocument;
  pub fn hoedown_document_render(doc: *mut HoedownDocument, ob: *mut HoedownBuffer, data: *const u8, size: size_t);
  pub fn hoedown_document_render_inline(doc: *mut HoedownDocument, ob: *mut HoedownBuffer, data: *const u8, size: size_t);
  pub fn hoedown_document_free(doc: *mut HoedownDocument);

  // escape.h
  pub fn hoedown_escape_href(ob: *mut HoedownBuffer, data: *const u8, size: size_t);
  pub fn hoedown_escape_html(ob: *mut HoedownBuffer, data: *const u8, size: size_t, secure: c_int);

  // html.h
  pub fn hoedown_html_smartypants(ob: *mut HoedownBuffer, data: *const u8, size: size_t);
  pub fn hoedown_html_is_tag(data: *const u8, size: size_t, tagname: *const c_char);
  pub fn hoedown_html_renderer_new(render_flags: c_int, nesting_level: c_int) -> *mut HoedownRenderer;
  pub fn hoedown_html_toc_renderer_new(nesting_level: c_int) -> *mut HoedownRenderer;
  pub fn hoedown_html_renderer_free(renderer: *mut HoedownRenderer);

  // version.h
  pub fn hoedown_version(major: *mut c_int, minor: *mut c_int, revision: *mut c_int);
}
