use rdxl::{xhtml,xtype,xrender};

fn bs(s: String) -> String {
   s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

xtype!(<!MyDisplayList><?/></MyDisplayList>);
xrender!(MyDisplayList, <ul>
  {{ for d in self.children.iter() {{
    {{ let MyDisplayListChildren::Display(d) = d; }}
    <li>{{ d }}</li>
  }} }}
</ul>);

#[test]
fn display1() {
   assert_eq!(
     bs(xhtml!(<!MyDisplayList>
       <?>{{ format!("a:{}",2) }}</?>
       <?>{{ format!("b:{}",4) }}</?>
     </MyDisplayList>)),
     "<ul> <li>a:2</li> <li>b:4</li> </ul>"
   );
}

#[test]
fn display2() {
   assert_eq!(
     bs(xhtml!(<!MyDisplayList>
       <?><h2>nested</h2></?>
     </MyDisplayList>)),
     "<ul> <li><h2>nested</h2></li> </ul>"
   );
}
