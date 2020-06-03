#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

#[test]
fn loops1(){
   let my_int = 3;
   let my_str = "asdf";
   let my_vec = vec![true, false, true, true];

   assert_eq!(xhtml!(<div>
      {{ for v in my_vec.iter() {{
         <span>{{my_int}}, {{my_str}}, {{v}}</span>
      }} }}
   </div>),
   "<div> <span>3, asdf, true</span><span>3, asdf, false</span><span>3, asdf, true</span><span>3, asdf, true</span> </div>".to_string());
}

#[test]
fn loops2(){
   let mut my_counter = 3;

   assert_eq!(xhtml!(<div>
      {{ while my_counter > 0 {{
         <span>{{my_counter}}</span>
         {{ my_counter -= 1; }}
      }} }}
   </div>),
   "<div> <span>3</span><span>2</span><span>1</span> </div>".to_string());
}

#[test]
fn loops3(){
   let mut my_some = Some(23);
   assert_eq!(xhtml!(<div>
      {{ while let Some(my_num) = my_some {{
         <span>{{my_num}}</span>
         {{ my_some = None; }}
      }} }}
   </div>),
   "<div> <span>23</span> </div>".to_string());
}
