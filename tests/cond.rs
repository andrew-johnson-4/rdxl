#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

fn main(){
   let x = 5;
   let y = Some(2);

   println!("{}",xhtml!(<div>
      {{ if x > 2 {{
         {{ x }}
      }} }}
      {{ if x < 2 {{
         {{ x }}
      }} else {{
         {{ 2 }}
      }} }}
      {{ if x < 2 {{
         {{ x }}
      }} else if x < 9 {{
         {{ 7 }}
      }} else if x < 10 {{
         {{ 8 }}
      }} }}
      {{ if let None = y {{
         None
      }} else if let Some(yy) = y {{
         {{ yy }}
      }} }}
   </div>));
}
