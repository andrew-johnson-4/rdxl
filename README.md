# Rusty Domain Extensible Language

Domain specific language macros for Rust to generate xhtml.

```rust
let my_int = 3;
let my_str = "asdf";
let my_vec = vec![true, false, true, true];

println!("{}",rdxl!(<ul>
   {{ for v in my_vec.iter() {{
      <li>{{my_int}}, {{my_str}}, {{v}}</li>
   }} }}
</ul>));
```

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in rdxl by you,
shall be dual licensed under the MIT and Apache 2.0 license without any additional terms or conditions.
