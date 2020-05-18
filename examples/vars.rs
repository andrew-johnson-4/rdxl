#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

#[derive(Debug)]
struct MyStruct {
   a: u64,
   b: u64
}

fn main(){
   let my_int = 22;
   let my_str = "ndklasfjkli";
   let my_struct = MyStruct { a:1, b:2 };
   println!("{}", rdxl!(<div>{{ my_var }}</div>) );
   println!("{}", rdxl!(<div>{{ my_str }}</div>) );
   println!("{}", rdxl!(<div>{{ my_struct }}</div>) );
}
