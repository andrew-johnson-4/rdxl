#![feature(proc_macro_hygiene)]
use rdxl::xhtml;

fn bs(s: String) -> String {
   s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[test]
fn breaking_for() {
   assert_eq!(
      &bs(xhtml!(<ul>
        <li>1</li>
        <li>{{ 2 }}</li>
        {{ for i in 3..5 {{
          <li>{{ i }}</li>
        }} }}
      </ul>)),
      "<ul> <li>1</li> <li>2</li> <li>3</li> <li>4</li> </ul>"
   );
}
