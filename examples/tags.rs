#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

fn main(){
   println!("{}", rdxl!(<div>dave</div>) );
   println!("{}", rdxl!(<a href="there">this that</a><br><p attr="something.f()">that this</p>) );
}
