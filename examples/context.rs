#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

struct MyMarkup {
   a:u64
}
impl MyMarkup {
   fn to_markup(&self) -> String {
      format!("{{a:{}}}", self.a)
   }
}

fn main(){
   let x = MyMarkup { a:22 };
   println!("{}", xhtml!([[ x ]]) );

   //fails due to no method "to_markup" on integer
   //println!("{}", xhtml!([[ 5 ]]) );
}
