#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

#[test]
fn tag1() {
   assert_eq!(
      xhtml!("<div></div>"),
      "<div></div>".to_string()
   );
}

#[test]
fn tag2() {
   assert_eq!(
      xhtml!(<div>dave</div><div>david</div>),
      "<div>dave</div><div>david</div>".to_string()
   );
}

#[test]
fn tag3() {
   assert_eq!(
      xhtml!(<a href="there">this that</a><br/><p attr="something.f()">that this</p>),
      "<a href=\"there\">this that</a><br/><p attr=\"something.f()\">that this</p>".to_string()
   );
}

#[test]
fn tag4() {
   assert_eq!(
      xhtml!(<input type="text"/>),
      "<input type=\"text\"/>".to_string()
   );
}

/* panics during compilation, not test; so I don't know how to unit test this
#[test]
#[should_panic]
fn tag5() {
   assert_eq!(
      xhtml!(<a></b>),
      "<a></b>".to_string()
   );
}
*/
