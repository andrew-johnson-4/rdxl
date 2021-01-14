# Introduction

Rdxl is a set of macros for generating xhtml from Rust. By using rdxl
it is possible to intermix xhtml and Rust code to generate interactive documents.

In this book we will document the exact grammar of each rdxl macro and
provide many motivating examples and several cheat sheets.

```rust
extern crate rdxl;

fn main() {
  println!("{}",rdxl::xhtml!(
    <p>This copy, version {{rdxl::version}}, is the latest copy of this document</p>
    <p>We hope that this will satisfy all your web templating needs</p>
  ));
}
```
