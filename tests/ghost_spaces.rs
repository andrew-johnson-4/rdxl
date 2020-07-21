use rdxl::xhtml;

#[test]
fn nonbreaking_lex1() {
   assert_eq!(
     xhtml!(::),
     "::"
   );
}
