#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

fn main(){
   let my_int = 3;
   let my_str = "asdf";
   let my_vec = vec![true, false, true, true];

   println!("{}",rdxl!(<div>
      {{ for v in my_vec.iter() {{
         <span>{{my_int}}, {{my_str}}, {{v}}</span>
      }} }}
   </div>));
}
