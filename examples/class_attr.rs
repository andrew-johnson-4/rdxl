#![feature(proc_macro_hygiene)]
use rdxl::{xhtml,xtype,xrender};

xtype!(<!MyType field1:MyField field2:MyField2/>);
xtype!(<!MyField x:u64/>);
xtype!(<!MyField2 x:String/>);
impl std::default::Default for MyField {
   fn default() -> MyField {
      MyField {
         x:0,
         children:Vec::new()
      }
   }
}
impl std::default::Default for MyField2 {
   fn default() -> MyField2 {
      MyField2 {
         x:"".to_string(),
         children:Vec::new()
      }
   }
}

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
     field1=<!MyField x=1/>
     field2=<!MyField2 x="2"/>
   />));

   println!("{}", xhtml!(<!MyType
     field1={{ MyField{x:3, children:vec![]} }}
     field2=[[ MyField{x:4, children:vec![]} ]]
   />));
}
