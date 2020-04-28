#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

fn main(){
   println!("{}", rdxl!(World) );
   println!("{}", rdxl!(Hello World) );
   println!("{}", rdxl!(Hello World ,.!@#$%^*|:;,.?/~) );
   println!("{}", rdxl!(while if let continue break) );
}
