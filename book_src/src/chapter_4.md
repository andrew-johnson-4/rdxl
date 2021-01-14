# Markup Reference

All html tags may be used directly in the body of the macro invocation.
Most tag attributes may be used normally as well. Some tag attribute
names, for example those with dashes in them, confuse the Rust
lexer and therefore should be quoted. All strings are valid attribute
names as long as they are quoted as a string literal.

```rust
extern crate rdxl;

fn main() {
   println!("{}", rdxl::xhtml!(
      <a href="/this_is_ok" "this-must-be-quoted"="abcd">body of link</a>
   ));
}
```

Rust expressions may be interpolated as attribute values. To insert a
Rust expression in attribute position, use the double braces format.
When rust expressions are used as attributes, the string value is
quoted and escape characters are inserted in place of double quotes
etc.

```rust
extern crate rdxl;

fn main() {
   let a_class = "my_class";
   let a_style = "position:absolute; top:0; left:0;";
   println!("{}", rdxl::xhtml!(
      <div class={{a_class}} style={{a_style}}>inner html</div>
   ));
}
```
