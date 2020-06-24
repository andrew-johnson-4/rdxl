#![feature(proc_macro_hygiene)]
use rdxl::{xhtml,xtype,xrender};

xtype!(<!MyAttr field:u64><!MyAttrChild field:u64/></MyAttr>);
impl std::default::Default for MyAttr {
   fn default() -> MyAttr {
      MyAttr {
         field:0,
         children:Vec::new()
      }
   }
}
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
