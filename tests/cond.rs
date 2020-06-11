#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

fn bs(s: String) -> String {
   s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[test]
fn conditional1(){
   let x = 5;
   let y = Some(2);

   assert_eq!(bs(xhtml!(<div>
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
   </div>)), 
   "<div> 5 2 7 2 </div>".to_string());
}
