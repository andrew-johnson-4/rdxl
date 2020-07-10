#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

#[test]
fn nonbreaking_lex1() {
   assert_eq!(
     xhtml!(::),
     "::"
   );
}
