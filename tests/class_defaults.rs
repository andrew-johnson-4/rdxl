use rdxl::{xhtml,xtype,xrender};

xtype!(<!A b:String c:u64/>);
xtype!(<!B a:bool c:u64/>);
xtype!(<!C a:bool b:String/>);
xrender!(A,{{self.b}}{{self.c}});
xrender!(B,{{self.a}}{{self.c}});
xrender!(C,{{self.a}}{{self.b}});

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
   ),"12");
}

#[test]
fn default4() {
   assert_eq!(xhtml!(
     <!A/>
   ),"0");
}

