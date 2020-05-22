#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

struct MyVal {}
impl MyVal {
   fn to_style(&self) -> String {
      "\"My:Val;\"".to_string()
   }
}

fn main(){
   println!("{}", rdxl!(<div style=[[ MyVal{} ]]>dave</div>) );
   println!("{}", rdxl!(<div an_attr={{ 2 }}>dave</div>) );
}
