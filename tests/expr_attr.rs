#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

#[test]
fn misc1(){
   let mut x = 5;
   assert_eq!(xhtml!(
      <script {{ if x>2 {{ "async" }} }} {{ if x<3 {{ defer }} else {{ src="abc" }} }}>
         a b c
      </script>
   ),
   r#"<script async src="abc"> a b c </script>"#);
}
