#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

fn main(){
   println!("{}", rdxl!(Hello World) );
   println!("{}", rdxl!(World) );

}
