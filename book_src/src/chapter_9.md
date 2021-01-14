# xtype

The xtype! macro defines an XML-like struct. An XML-like struct
has zero or more attributes, possibly having default values, and
zero or more possible children types.

Attributes that do not supply a default value must have a type
which implements the std::default::Default trait. This is
an implementation quirk, so it is notable, although undesirable.

Children can have any type, which may be defined inline as another
tag, or reference an existing type. A boxed Display type is
available by using the ? tag, which is a handy way to include
miscellaneous content which does not require special rendering
logic.

```rust
extern crate rdxl;

rdxl::xtype!(<!MyTag a:u64={{32}} b:String>
  <?>
</MyTag>);
```

the above code generates the following items

```rust
struct MyTag {
   a: u64,
   b: String,
   children: Vec<MyTagChildren>
}
enum MyTagChildren {
   Display(Box<dyn std::fmt::Display>)
}

impl MyTag {
   pub fn new() -> MyTag {
      MyTag {
         a: 32,
         b: std::default::Default::default(),
         children: Vec::new(),
      }
   }
   pub fn set_a(mut self, v: u64) -> MyTag {
      self.a = v;
      self
   }
   pub fn set_b(mut self, v: String) -> MyTag {
      self.b = v;
      self
   }
   pub fn set_children(mut self, v: Vec<MyTagChildren>) -> MyTag {
      self.children = v;
      self
   }
}
```

