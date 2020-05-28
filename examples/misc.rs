#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

fn main(){
   let mut x = 5;
   println!("{}",xhtml!(<div>
      {{ x }},
      {{ x = 3; }}
      {{ x }},
      {{ x = 7; }}
      {{ x }},
      {{ let mut y = 2 }}
      {{ y }},
      {{ y = 1; }}
      {{ y }}
   </div>));
}
