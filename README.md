# Rusty Domain Extensible Language

Domain specific language macros for Rust to generate xhtml.

```rust
let my_int = 3;
let my_str = "asdf";
let my_vec = vec![true, false, true, true];

println!("{}",xhtml!(<ul>
   {{ for v in my_vec.iter() {{
      <li>{{my_int}}, {{my_str}}, {{v}}</li>
   }} }}
</ul>));
```

Modularized templating is encouraged through custom XML elements:

```rust
xtype!(<!MyList my_string:String my_int:u64>
   <!MyItem my_bool:bool/>
   <!MyOtherItem my_char:char/>
</MyList>);

xrender!(MyList, <ul>
  <li>{{ self.my_string }}</li>
  <li>{{ self.my_int }}</li>
  {{ for i in self.children.iter() {{
    {{ if let MyListChildren::MyItem(my_item) = i {{
      <li>MyItem: {{ my_item.my_bool }}</li>
    }} else if let MyListChildren::MyOtherItem(my_other_item) = i {{
      <li>MyOtherItem: {{ my_other_item.char }}</li>
    }} }}
  }} }}
</ul>);
```

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in rdxl by you,
shall be dual licensed under the MIT and Apache 2.0 license without any additional terms or conditions.
