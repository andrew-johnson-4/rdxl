#![feature(proc_macro_hygiene)]
use rdxl::{xhtml,xtype,xrender};

xtype!(<!MyPredefinedType/>);
xtype!(<!MyList my_string:String my_int:u64>
   <!MyItem my_bool:bool/>
   <!MyOtherItem my_char:char/>
   <?MyPredefinedType/>
</MyList>);

xrender!(MyList, <ul>
  <li>{{ self.my_string }}</li>
  <li>{{ self.my_int }}</li>
  {{ for i in self.children.iter() {{
    {{ if let MyListChildren::MyItem(my_item) = i {{
      <li>MyItem: {{ my_item.my_bool }}</li>
    }} else if let MyListChildren::MyOtherItem(my_other_item) = i {{
      <li>MyOtherItem: {{ my_other_item.my_char }}</li>
    }} }}
  }} }}
</ul>);

xrender!(MyItem, <span>my_bool: {{ self.my_bool }}</span>);
xrender!(MyOtherItem, <span>my_char: {{ self.my_char }}</span>);

#[test]
fn simple_class1() {
   assert_eq!(
      xhtml!(<!MyItem my_bool=true/>),
      "<span>my_bool: true</span>".to_string()
   );
}

#[test]
fn simple_class2() {
   assert_eq!(
      xhtml!(<!MyOtherItem my_char='c'></MyOtherItem>),
      "<span>my_char: c</span>".to_string()
   );
}

#[test]
fn complex_class1(){
   assert_eq!(xhtml!(<!MyList my_string="abcdefg" my_int=33>
       <!MyItem my_bool=true/>
       <!MyItem my_bool=false/>
       <!MyOtherItem my_char='a'/>
     </MyList>),
     "<ul> <li>abcdefg</li> <li>33</li> <li>MyItem: true</li><li>MyItem: false</li><li>MyOtherItem: a</li> </ul>".to_string()
   );
}
