# HTML Markup

When generating markup, the most common macro to be used is xhtml!. This macro
takes mixed markup and rust code to output markup formatted as a String.

Inside xhtml! there are several possible types of markup that can be sent to
the formatter:
1. XHTML Tags
2. Unquoted Text Data
3. Quoted Text Data

To generate XHTML Tags, place the tags in angle brackets like normal markup. The
tags can be self-closing or contain inner HTML.

```rust
extern crate rdxl;

fn main() {
  println!("{}",rdxl::xhtml!(
    <br/>
  ));
}
```

To generate Unquoted Text Data it is usually sufficient to place the text
directly in the markup. Note that macros are lexed as Rust code before being
sent to the macro procedure. This means that not all XHTML can be placed directly
inline.

```rust
extern crate rdxl;

fn main() {
  println!("{}",rdxl::xhtml!(
    <p>This paragraph is formatted normally. The breaking spaces are recognized
       as part of the macro rules.</p>
  ));
}
```

For Text Data that does not work well with the Rust lexer, the text may be placed
inside a rust quoted string literal. This string will be placed directly into the
formatted output. It should be noted that raw strings are very a good way to fit
even more text data into one quote without the need for escape characters.

```rust
extern crate rdxl;

fn main() {
  println!("{}",rdxl::xhtml!(
    r#"((((( the lexer expects closing braces, brackets, and parentheses [[[[
       but what do we care. {{"#
  ));
}
```
