#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

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
   println!("{}", rdxl!([[ x ]]) );

   //fails due to no method "to_markup" on integer
   //println!("{}", rdxl!([[ 5 ]]) );
}
