#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

#[test]
fn misc1(){
   let x = 5;
   assert_eq!(xhtml!(
      <script {{if x>2 {{ "async" }}}} defer>
         a b c
      </script>
   ),
   r#"<script async defer> a b c </script>"#);
}
