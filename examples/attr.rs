#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

fn main(){
   println!("{}", xhtml!(<div style="color:#FFFFFF; background-color:#000000;">dave</div>) );
}
