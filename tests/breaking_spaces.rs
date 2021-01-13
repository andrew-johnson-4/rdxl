use rdxl::{xhtml,xtext};

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
#[test]
fn nonbreaking_for() {
   assert_eq!(
      &xtext!(<ul>
        <li>1</li>
        <li>{{ 2 }}</li>
        {{ for i in 3..5 {{
          <li>{{ i }}</li>
        }} }}
      </ul>),
      "<ul><li>1</li><li>2</li><li>3</li><li>4</li></ul>"
   );
}

#[test]
fn breaking_while() {
   assert_eq!(
      &bs(xhtml!(<ul>
        {{ let mut i = 3; }}
        {{ while i>0 {{
          <li>{{ i }}</li>
          {{ i -= 1; }}
        }} }}
      </ul>)),
      "<ul> <li>3</li> <li>2</li> <li>1</li> </ul>"
   );
}
#[test]
fn nonbreaking_while() {
   assert_eq!(
      &xtext!(<ul>
        {{ let mut i = 3; }}
        {{ while i>0 {{
          <li>{{ i }}</li>
          {{ i -= 1; }}
        }} }}
      </ul>),
      "<ul><li>3</li><li>2</li><li>1</li></ul>"
   );
}

#[test]
fn breaking_if() {
   assert_eq!(
      &bs(xhtml!(<ul>
        {{ let x = 5; }}
        {{ if x>4 {{
          <li>1</li>
        }} }}
        {{ if x<4 {{
          <li>1</li>
        }} else if x>4 {{
          <li>2</li>
        }} }}
        {{ if x<4 {{
          <li>1</li>
        }} else {{
          <li>3</li>
        }} }}
      </ul>)),
      "<ul> <li>1</li> <li>2</li> <li>3</li> </ul>"
   );
}
#[test]
fn nonbreaking_if() {
   assert_eq!(
      &xtext!(<ul>
        {{ let x = 5; }}
        {{ if x>4 {{
          <li>1</li>
        }} }}
        {{ if x<4 {{
          <li>1</li>
        }} else if x>4 {{
          <li>2</li>
        }} }}
        {{ if x<4 {{
          <li>1</li>
        }} else {{
          <li>3</li>
        }} }}
      </ul>),
      "<ul><li>1</li><li>2</li><li>3</li></ul>"
   );
}

#[test]
fn nonbreaking_lex2() {
   assert_eq!(
     xhtml!(macro),
     "macro"
   );
}
