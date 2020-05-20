#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rdxl;

fn main(){
   let x = 5;

   println!("{}",rdxl!(<div>
      {{ if x > 2 {{
         {{ x }}
      }} }}
      {{ if x < 2 {{
         {{ x }}
      }} else {{
         {{ 2 }}
      }} }}
   </div>));
}
