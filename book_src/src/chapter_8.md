# xrender

The xrender! macro defines a Display trait for a
type. The type is supplied as the first argument,
and the rest of the macro defines the XHTML that
is to be formatted as output.

All fields of the type can be accessed through the
*self* value which is defined over the body of
the macro invocation.

```rust
extern crate rdxl;

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
