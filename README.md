# Rusty Domain Extensible Language

Domain specific language macros for Rust to generate xhtml.

```rust
let my_int = 3;
let my_str = "asdf";
let my_vec = vec![true, false, true, true];

println!("{}",rdxl!(<div>
   {{ for v in my_vec.iter() {{
      <span>{{my_int}}, {{my_str}}, {{v}}</span>
   }} }}
</div>));
```

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in rdxl by you,
shall be dual licensed under the MIT and Apache 2.0 license without any additional terms or conditions.
