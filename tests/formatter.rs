#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

struct MyVal {}
impl MyVal {
   fn to_style(&self) -> String {
      "My:Val;".to_string()
   }
}

#[test]
fn formatter1(){
   assert_eq!(
      xhtml!(<div style=[[ MyVal{} ]]>dave</div>),
      "<div style=\"My:Val;\">dave</div>".to_string()
   );
}

#[test]
fn formatter2(){
   assert_eq!(
      xhtml!(<div an_attr={{ 2 }}>dave</div>),
      "<div an_attr=\"2\">dave</div>".to_string()
   );
}
