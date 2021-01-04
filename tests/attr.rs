use rdxl::{xhtml,xtext};

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

#[test]
fn tattribute_no_spaces(){
   assert_eq!(
      xtext!(<div style="color:#FFFFFF; background-color:#000000;">"dave"</div>),
      "<div style=\"color:#FFFFFF; background-color:#000000;\">dave</div>".to_string()
   );
}

#[test]
fn tattribute_autoquote(){
   assert_eq!(
      xtext!(<div style={{ "color:#FFFFFF; background-color:#000000;" }}>"dave"</div>),
      "<div style=\"color:#FFFFFF; background-color:#000000;\">dave</div>".to_string()
   );
}
