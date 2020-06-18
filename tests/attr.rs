#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

#[test]
fn attribute_no_spaces(){
   assert_eq!(
      xhtml!(<div style="color:#FFFFFF; background-color:#000000;">dave</div>),
      "<div style=\"color:#FFFFFF; background-color:#000000;\">dave</div>".to_string()
   );
}

#[test]
fn attribute_autoquote(){
   assert_eq!(
      xhtml!(<div style={{ "color:#FFFFFF; background-color:#000000;" }}>dave</div>),
      "<div style=\"color:#FFFFFF; background-color:#000000;\">dave</div>".to_string()
   );
}
