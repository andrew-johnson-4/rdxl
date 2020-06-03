#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

/* what is the expected behaviour here
#[test]
fn misc1(){
   let mut x = 5;
   assert_eq!(xhtml!(<div>
      {{ x }},
      {{ x = 3; }}
      {{ x }},
      {{ x = 7; }}
      {{ x }},
      {{ let mut y = 2; }}
      {{ y }},
      {{ y = 1; }}
      {{ y }}
   </div>),
   "<div> 5, 3, 7, 2, 1 </div>".to_string());
}
*/
