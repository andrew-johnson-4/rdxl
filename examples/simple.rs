#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

fn main(){
   println!("{}", xhtml!(World) );
   println!("{}", xhtml!(Hello World) );
   println!("{}", xhtml!(Hello World ,.!@$#%^*|:;,.?/~) );
   println!("{}", xhtml!(while if let continue break) );
   println!("{}", xhtml!("#""{""}#"));
}
