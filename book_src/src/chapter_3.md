# Rust Code

Rust code greatly enhances the generative capabilities of the Rdxl macro rules.
Most rust expressions and statements may be placed inside of Rdxl markup and
generate code nearly identical to what is stated. This is very helpful not
only for creating new powerful abstractions, but also for fixing bugs: error
messages for syntax or type errors are correctly tracked and blamed with the
same helpful error formatting that Rust is so well known for.

In Rdxl, Rust code is divided into two kinds of syntax expressions: Rust statements
and Rust expressions. The difference between a statement or expression is always
determined by syntax rather than inference. Expressions always emit a value that
implements the Display trait as the return value of the expression. Statements
may emit data directly to the string buffer but never as the return value of the
statement.

Both expressions and statements are surrounded by double braces to signify that
they should be interpreted as Rust code rather than xhtml.

A simple example of a rust expression is a variable interpolated into the markup:

```rust
extern crate rdxl;

fn main() {
   let x = 5;
   println!("{}", rdxl::xhtml!(
      These strings are literals, but {{x}} is a variable.
   ));
}
```

For a complete reference of all expressions, see [Chapter 5: Expression Reference](./chapter_5.md).

A simple statement would be a for loop:

```rust
   println!("{}", rdxl::xhtml!(
      {{ for x in 0..10 {{
         These strings are literals, but {{x}} is a variable.
      }} }}
   ));
```

For a complete reference of all statements, see [Chapter 6: Statement Reference](./chapter_6.md).
