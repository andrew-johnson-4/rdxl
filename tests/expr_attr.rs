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

#[test]
fn misc2(){
   let x = 5;
   assert_eq!(xhtml!(
      <script {{if x>2 {{ "src"="abc" }}}} {{if x>3 {{ "type"={{x}} }}}}>
         a b c
      </script>
   ),
   r#"<script src="abc" type="5"> a b c </script>"#);
}
