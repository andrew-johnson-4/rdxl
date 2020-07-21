use rdxl::xhtml;

struct MyMarkup {
   a:u64
}
impl MyMarkup {
   fn to_markup(&self) -> String {
      format!("{{a:{}}}", self.a)
   }
}

#[test]
fn context_test1(){
   let x = MyMarkup { a:22 };
   assert_eq!(
      xhtml!([[ x ]]),
      "{a:22}".to_string()
   );
}

/* fails during compilation due to no method "to_markup" on integer
#[test]
#[should_panic]
fn context_test2(){
   assert_eq!(
      xhtml!([[ 5 ]]),
      "5".to_string()
   );
}
*/
