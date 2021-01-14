# Expression Reference

All interpolated Rust code is interpreted as an expression
provided that

1. The snippet does not start with a reserved statement keyword
2. The snippet does not end with a semicolon

The statement keywords are: if, let, for, while, and loop.

```rust
extern crate rdxl;

fn main() {
   let a_flag = true;
   println!("{}", rdxl::xhtml!(
      {{ if a_flag {{ this is a statement }} }}
      {{ (if a_flag { "this is" } else { "an expression" }) }}
   ));
}
```

Aside from these reservations, all Rust code may be used as
expressions as long as they return a value that implements the
Display trait.

```rust
extern crate rdxl;

fn main() {
   println!("{}", rdxl::xhtml!(
     {{ "ab" }}
     {{ 'c' }}
     {{ format!("{} {}", 2, 3) }}
     {{ true }}
   ));
}
```
