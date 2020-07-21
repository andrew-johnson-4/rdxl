use rdxl::xhtml;

fn main(){
   println!("{}", xhtml!(<div>dave</div><div>david</div>) );
   println!("{}", xhtml!(<a href="there">this that</a><br/><p attr="something.f()">that this</p>) );
   println!("{}", xhtml!(<input type="text"/>))
   //println!("{}", xhtml!(<a></b>) );
}
