use std::time::{Instant};

struct Display1 {}
impl std::fmt::Display for Display1 {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      for _ in 1..1000000000 {
         f.write_str("a")?;
      }
      Ok(())
   }
}

struct Display2 {}
impl std::fmt::Display for Display2 {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      let mut buf = String::new();
      for _ in 1..1000000000 {
         buf.push_str("a");
      }
      f.write_str(&buf)
   }
}

fn main() {
   let start = Instant::now();
   let _ = Display1{}.to_string();
   println!("many Formatter writes: {}.{}s", start.elapsed().as_secs(), start.elapsed().subsec_millis());

   let start = Instant::now();
   let _ = Display2{}.to_string();
   println!("many String writes: {}.{}s", start.elapsed().as_secs(), start.elapsed().subsec_millis());
}


