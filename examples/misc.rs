#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

fn main(){
   let mut x = 5;
   println!("{}",rdxl!(<div>
      {{ x }},
      {{ ; x = 3 }}
      {{ x }},
      {{ ; x = 7 }}
      {{ x }}
   </div>));
}
