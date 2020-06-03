#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
use std::fmt;

struct MyStruct {
   a: u64,
   b: u64
}
impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyStruct{{a:{}, b:{}}}", self.a, self.b)
    }
}

#[test]
fn vars1(){
   let my_int = 22;
   assert_eq!(
      xhtml!(<div>{{ my_int }}</div>),
      "<div>22</div>".to_string()
   );
}

#[test]
fn vars2() {
   let my_str = "ndklasfjkli";
   assert_eq!(
      xhtml!(<div>{{ my_str }}</div>),
      "<div>ndklasfjkli</div>".to_string()
   );
}

#[test]
fn vars3() {
   let my_struct = MyStruct { a:1, b:2 };
   assert_eq!(
      xhtml!(<div>{{ my_struct }}</div>),
      "<div>MyStruct{a:1, b:2}</div>".to_string()
   );
}
