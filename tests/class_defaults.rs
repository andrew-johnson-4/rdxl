use rdxl::{xhtml,xtype,xrender};

xtype!(<!A b:String c:u64/>);
xrender!(A,{{self.b}}{{self.c}});

#[test]
fn default1() {
   assert_eq!(xhtml!(
     <!A b="bb" c=12/>
   ),"bb12");
}

#[test]
fn default2() {
   assert_eq!(xhtml!(
     <!A c=12/>
   ),"12");
}

#[test]
fn default3() {
   assert_eq!(xhtml!(
     <!A b="bb"/>
   ),"bb0");
}

#[test]
fn default4() {
   assert_eq!(xhtml!(
     <!A/>
   ),"0");
}

