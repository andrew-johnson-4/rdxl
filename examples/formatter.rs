#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

struct MyVal {}
impl MyVal {
   fn to_style(&self) -> String {
      "\"My:Val;\"".to_string()
   }
}

fn main(){
   println!("{}", xhtml!(<div style=[[ MyVal{} ]]>dave</div>) );
   println!("{}", xhtml!(<div an_attr={{ 2 }}>dave</div>) );
}
