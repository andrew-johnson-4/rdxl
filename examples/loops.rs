use rdxl::xhtml;

fn main(){
   let my_int = 3;
   let my_str = "asdf";
   let my_vec = vec![true, false, true, true];

   println!("{}",xhtml!(<div>
      {{ for v in my_vec.iter() {{
         <span>{{my_int}}, {{my_str}}, {{v}}</span>
      }} }}
   </div>));

   let mut my_counter = 3;

   println!("{}",xhtml!(<div>
      {{ while my_counter > 0 {{
         <span>{{my_counter}}</span>
         {{ my_counter -= 1; }}
      }} }}
   </div>));

   let mut my_some = Some(23);
   println!("{}",xhtml!(<div>
      {{ while let Some(my_num) = my_some {{
         <span>{{my_num}}</span>
         {{ my_some = None; }}
      }} }}
   </div>));
}
