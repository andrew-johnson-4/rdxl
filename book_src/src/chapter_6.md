# Statement Reference

The statement forms are: if, let, for, while, and loop. Also, if
an expression ends with a semicolon its value will be discarded.

## Semicolon

```rust
extern crate rdxl;

fn main() {
   let mux x = 3;
   println!("{}", rdxl::xhtml!(
      {{ x += 2; }}
   ));
}
```

## If

```rust
extern crate rdxl;

fn main() {
   let x = 3;
   println!("{}", rdxl::xhtml!(
      {{ if x<2 {{
         Case 1
      }} else if x<5 {{
         Case 2
      }} else {{
         Case 3
      }}
   ));
}
```

## Let

```rust
extern crate rdxl;

fn main() {
   println!("{}", rdxl::xhtml!(
      {{ let mut x = 5; }}
      {{ x }}
      {{ x += 2; }}
      {{ x }}
   ));
}
```

## For

```rust
extern crate rdxl;

fn main() {
   println!("{}", rdxl::xhtml!(
      {{ for x in 0..10 {{
         {{x}}
      }} }}
   ));
}
```

## While

```rust
extern crate rdxl;

fn main() {
   let mut x = 3;
   println!("{}", rdxl::xhtml!(
      {{ while x>0 {{
         {{ x }}
         {{ x -= 1; }}
      }} }}
   ));
}
```

## Loop

```rust
extern crate rdxl;

fn main() {
   println!("{}", rdxl::xhtml!(
      {{ loop {{
         <p>inside loop</p>
         {{ break; }}
      }} }}
   ));
}
```
