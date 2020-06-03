#![feature(proc_macro_hygiene)]
use rdxl::{xhtml,xtype,xrender};
use std::fmt;

xtype!(<!MyAttr field:u64><!MyAttrChild field:u64/></MyAttr>);
xtype!(<!MyType attr:MyAttr/>);
xrender!(MyType, <ul>
  {{ for MyAttrChildren::MyAttrChild(c) in self.attr.children.iter() {{
    <li>{{ self.attr.field }}:{{ c.field }}</li>
  }} }}
</ul>);

fn main(){
   println!("{}", xhtml!(<!MyType attr=<!MyAttr field=3>
     <!MyAttrChild field=2/>
     <!MyAttrChild field=1/>
   </MyAttr> />));
}
