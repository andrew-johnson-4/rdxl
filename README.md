Rusty Domain Extensible Language
================================

[![Crates.IO](https://img.shields.io/crates/v/rdxl.svg)](https://crates.rs/crates/rdxl)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://andrew-johnson-4.github.io/rdxl/)
[![Build Nightly](https://github.com/andrew-johnson-4/rdxl/workflows/BuildNightly/badge.svg)](https://github.com/andrew-johnson-4/rdxl)
[![Build](https://github.com/andrew-johnson-4/rdxl/workflows/Build/badge.svg)](https://github.com/andrew-johnson-4/rdxl)

Domain specific language macros for Rust to generate xhtml. (pronounced "Rad Axle")

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

Modularized templating is encouraged through custom XML elements that implement the Display property.
Foreign xhtml snippets or miscellaneous content can be inserted inline as long as it also implements
the Display property.

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
      <li>MyOtherItem: {{ my_other_item.my_char }}</li>
    }} }}
  }} }}
</ul>);
```

Foreign syntaxes, like Javascript, may be quoted inline or inserted as CDATA.

```rust
xrender!(BarGraph,
  <script src="https://d3js.org/d3.v4.min.js"></script>
  <script>
    "var margin = {top: 20, right: 20, bottom: 30, left: 40},"
        "width = 960 - margin.left - margin.right,"
        "height = 500 - margin.top - margin.bottom;"
    ...
  </script>
);
```

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in rdxl by you,
shall be dual licensed under the MIT and Apache 2.0 license without any additional terms or conditions.
