# rust-markdown

Markdown rendering for Rust, using [Hoedown](https://github.com/hoedown/hoedown).

This is basically a 1-to-1 mapping of the Hoedown API, with added Rust semantics.

## How to use it

```rust
extern crate "rust-markdown" as markdown;

let renderer = markdown::HtmlRenderer::new();
let mut document = markdown::Document::new(&renderer);
let result = document.render("Hello, World!").to_string();
println!("{}", result); // => "<p>Hello, World!</p>\n"
```
