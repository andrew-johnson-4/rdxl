use rdxl::{xhtml,xtype,xrender};

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

#[test]
fn class_complex_some_spaces1(){
   assert_eq!(
     xhtml!(<!MyType
       field1=<!MyField x=1/>
       field2=<!MyField2 x="2"/>
     />),
     "<div> <b>field1:</b> 1 <br/> <b>field2:</b> 2 </div>".to_string()
   );
}

#[test]
fn class_complex_some_spaces2(){
   assert_eq!(
     xhtml!(<!MyType
       field1={{ MyField{x:3, children:vec![]} }}
       field2=[[ MyField{x:4, children:vec![]} ]]
     />),
     "<div> <b>field1:</b> 3 <br/> <b>field2:</b> 4 </div>".to_string()
   );
}
