#![feature(proc_macro_hygiene)]
use rdxl::{xhtml,xtype,xrender};
use std::fmt;

xtype!(<!MyType field1:MyField field2:MyField2/>);
xtype!(<!MyField x:u64/>);
xtype!(<!MyField2 x:String/>);

impl MyField {
   fn to_field2(&self) -> MyField2 {
      MyField2 { x: format!("{}",self.x), children:vec![] }
   }
}

xrender!(MyType, <div>
   <b>field1:</b> {{self.field1.x}} <br/>
   <b>field2:</b> {{self.field2.x}}
</div>);

fn main(){
   println!("{}", xhtml!(<!MyType
     field1={{ MyField{x:1} }}
     field2=[[ MyField{x:2} ]]
   />));
}
