#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

#[test]
fn test_fragment1(){
   assert_eq!(
      xhtml!(World),
      "World".to_string()
   );
}

#[test]
fn test_fragment2(){
   assert_eq!(
      xhtml!(Hello World),
      "Hello World".to_string()
   );
}

#[test]
fn test_fragment3(){
   assert_eq!(
      xhtml!(Hello World ,.!@$#%^*|:;,.?/~),
      "Hello World ,.!@$#%^*|:;,.?/~".to_string()
   );
}

#[test]
fn test_fragment4(){
   assert_eq!(
      xhtml!(while if let continue break),
      "while if let continue break".to_string()
   );
}

#[test]
fn test_fragment5(){
   assert_eq!(
      xhtml!("#""{""}#"),
      "#{}#".to_string()
   );
}
