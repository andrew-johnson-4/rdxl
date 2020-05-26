#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

fn main(){
   println!("{}", rdxl!(<div>dave</div><div>david</div>) );
   println!("{}", rdxl!(<a href="there">this that</a><br/><p attr="something.f()">that this</p>) );
   println!("{}", rdxl!(<input type="text"/>))
   //println!("{}", rdxl!(<a></b>) );
}
