use rdxl::{xtype,xrender,xhtml};

xtype!(<!A><?/></A>);
xrender!(A,<br/>);

#[test]
fn static1(){
   fn a(x: String) -> String {
      xhtml!(<!A><?>{{ x }}</?></A>)
   }
   assert_eq!(a("abc".to_string()), "<br/>");
}

/* should panic? because it does rn
#[test]
fn static2(){
   fn a(x: &str) -> String {
      xhtml!(<!A><?>{{ x }}</?></A>)
   }
   assert_eq!(a("abc"), "<br/>");
}
*/
