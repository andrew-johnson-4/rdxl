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

fn main(){
   let my_int = 22;
   let my_str = "ndklasfjkli";
   let my_struct = MyStruct { a:1, b:2 };
   println!("{}", xhtml!(<div>{{ my_int }}</div>) );
   println!("{}", xhtml!(<div>{{ my_str }}</div>) );
   println!("{}", xhtml!(<div>{{ my_struct }}</div>) );
}
