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

fn main(){
   println!("{}", xhtml!(<!MyList my_string="abcdefg" my_int=33>
     <!MyItem my_bool=true/>
     <!MyItem my_bool=false/>
     <!MyOtherItem my_char='a'/>
     <!MyItem my_bool=false/>
     <!MyOtherItem my_char='c'/>
   </MyList>));
}
