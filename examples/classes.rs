#![feature(proc_macro_hygiene)]
use rdxl::{xhtml,xtype,xrender};

xtype!(<!MyList my_string:String my_int:u64>
   <!MyItem my_bool:bool/>
   <!MyOtherItem my_char:char/>
</MyList>)

xrender!(MyList, |my_list| {
  <ul>
    <li>{{ my_list.my_string }}</li>
    <li>{{ my_list.my_int }}</li>
    {{ for i in my_list.children.iter() {{
      {{ if let MyListChildren::MyItem(my_item) = i {{
        <li>MyItem: {{ my_item.my_bool }}</li>
      }} else if let MyListChildren::MyOtherItem(my_other_item) = i {{
        <li>MyOtherItem: {{ my_other_item.char }}</li>
      }} }}
    }} }}
  </ul>
})

fn main(){
   println!("{}", xhtml!(<!MyList my_string="abcdefg" my_int=33>
     <MyItem my_bool=true/>
     <MyItem my_bool=false/>
     <MyOtherItem='a'/>
     <MyItem my_bool=false/>
     <MyOtherItem='c'/>
   </MyList>));
}
