use rdxl::xhtml;

#[test]
fn keyword_list1() {
   assert_eq!(
      xhtml!(try abstract become box do final macro override priv
         typeof unsized virtual yield as break const continue crate
         else enum extern false fn for if impl in let loop match mod
         move mut pub ref return self Self static struct super trait
         true type unsafe use where while async await dyn),
      "try abstract become box do final macro override priv typeof unsized virtual yield as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn"
   );
}

