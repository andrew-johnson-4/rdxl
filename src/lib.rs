// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

//! # HTML Templating Macros
//!
//! [Rdxl](https://andrew-johnson-4.github.io/rdxl) provides JSX style inline HTML macros for Rust. The only
//! dependencies are the ubiquitous proc-macro2, syn, and quote.
//!
//! The library is semantic versioned, so expect small fixes and many additions until a 1.0 release is reached.
//!
//! # Inline Rust expressions
//!
//! ```
//! # #![feature(proc_macro_hygiene)]
//! # use rdxl::xhtml;
//! # fn main() {
//! let x = 5;
//! let y = "asdf";
//! xhtml!({{ x }}, {{ y }})
//! # ;}
//! ```
//!
//! # Conditional expressions
//!
//! ```
//! # #![feature(proc_macro_hygiene)]
//! # use rdxl::xhtml;
//! # fn main() {
//! let x = 5;
//! let y = "asdf";
//! xhtml!({{ if x>3 {{
//!   Case One
//! }} else if x>2 {{
//!   Case Two
//! }} else {{
//!   Case Three
//! }} }})
//! # ;}
//! ```
//!
//! # Loop expressions
//!
//! ```
//! # #![feature(proc_macro_hygiene)]
//! # use rdxl::xhtml;
//! # fn main() {
//! xhtml!(<ul>{{ for i in 0..10 {{
//!   <li>{{ i }}</li>
//! }} }}</ul>)
//! # ;}
//! ```
//!
//! # Miscellaneous expressions
//!
//! ```
//! # #![feature(proc_macro_hygiene)]
//! # use rdxl::xhtml;
//! # fn main() {
//! xhtml!(
//!   {{ let x = 5; }}
//!   {{ x }}
//! )
//! # ;}
//! ```
//!
//! # Html attributes
//! ```
//! # #![feature(proc_macro_hygiene)]
//! # use rdxl::xhtml;
//! # fn main() {
//! xhtml!(<div style={{ "\"color:red;\"" }}>
//!   inside div
//! </div>);
//! # ;}
//! ```

#![recursion_limit = "128"]
#![crate_type = "proc-macro"]

mod xhtml;
mod xtype;
mod xrender;

use proc_macro::{TokenStream};
use syn::{parse_macro_input};
use quote::{quote};

/// The [xhtml!](https://andrew-johnson-4.github.io/rdxl) macro is the primary mechanism for templating in rdxl
///
/// <b>xhtml!</b> consumes mixed Rust code and XML markup as input and emits rendered xhtml to a string buffer.
/// Rust code is usually delimited by {{double braces}} or [[double brackets]]. The <b>syn</b> module is used to
/// allow most Rust expressions to be used inside the correct delimited contexts. Control flow structures
/// such as if/else blocks, loops, and let statements may be used inline as well.
///
/// Aside from standard XML syntax, custom types may be defined with <b>xtype!</b> and <b>xrender!</b> facilities. This
/// encourages typesafe modular templates to be created and shared.
///
/// Use of <b>xhtml!</b> usually looks something like this:
/// ```
/// # #![feature(proc_macro_hygiene)]
/// # use rdxl::xhtml;
/// # fn main() {
/// let mut x = 5;
///
/// println!("{}",xhtml!(<div>
///    {{ x }},
///    {{ x = 3; }}
///    {{ x }},
///    {{ x = 7; }}
///    {{ x }},
///    {{ let mut y = 2 }}
///    {{ y }},
///    {{ y = 1; }}
///    {{ y }}
///    {{ for i in (0..x) {{
///       <span>{{i}}</span>
///    }} }}
/// </div>));
/// # }
/// ```
#[proc_macro]
pub fn xhtml(input: TokenStream) -> TokenStream {
    let xhtmls = parse_macro_input!(input as rdxl_internals::xhtml::Xhtml);

    let expanded = quote! {
        {
            let mut stream = String::new();
            #xhtmls
            stream
        }
    };

    TokenStream::from(expanded)
}

/// The [xtext!](https://andrew-johnson-4.github.io/rdxl) macro is the primary mechanism for templating in rdxl
///
/// <b>xtext!</b> consumes mixed Rust code and XML markup as input and emits rendered xhtml to a string buffer.
/// Rust code is usually delimited by {{double braces}} or [[double brackets]]. The <b>syn</b> module is used to
/// allow most Rust expressions to be used inside the correct delimited contexts. Control flow structures
/// such as if/else blocks, loops, and let statements may be used inline as well.
///
/// Aside from standard XML syntax, custom types may be defined with <b>xtext!</b> and <b>xrender!</b> facilities. This
/// encourages typesafe modular templates to be created and shared.
///
/// Use of <b>xtext!</b> usually looks something like this:
/// ```
/// # #![feature(proc_macro_hygiene)]
/// # use rdxl::xtext;
/// # fn main() {
/// let mut x = 5;
///
/// println!("{}",xtext!(<div>
///    {{ x }},
///    {{ x = 3; }}
///    {{ x }},
///    {{ x = 7; }}
///    {{ x }},
///    {{ let mut y = 2 }}
///    {{ y }},
///    {{ y = 1; }}
///    {{ y }}
///    {{ for i in (0..x) {{
///       <span>{{i}}</span>
///    }} }}
/// </div>));
/// # }
/// ```
#[proc_macro]
pub fn xtext(input: TokenStream) -> TokenStream {
    panic!("xtext unimplemented");
    let xhtmls = parse_macro_input!(input as rdxl_internals::xhtml::Xhtml);

    let expanded = quote! {
        {
            let mut stream = String::new();
            #xhtmls
            stream
        }
    };

    TokenStream::from(expanded)
}

/// The [xtype!](https://andrew-johnson-4.github.io/rdxl) macro defines an xml element and subelements
///
/// <b>xtype!</b> removes some of the redundancy of defining types having many attribute fields
/// and many heterogenous children elements. A typical type definition that is a good fit for
/// this macro would be *coincidentally* most XML elements.
///
/// In <b>xtype!</b>, a definition might look like this:
/// ```
/// # use rdxl::xtype;
/// # pub struct MyPredefinedType {}
/// xtype!(<!MyList my_string:String my_int:u64>
///   <!MyItem my_bool:bool/>
///   <!MyOtherItem my_char:char/>
///   <?MyPredefinedType/>
/// </MyList>);
/// # fn main() {}
/// ```
///
/// In sugar-free Rust this would become like this:
/// ```
/// struct MyPredefinedType {}
/// struct MyItem { my_bool: bool }
/// struct MyOtherItem { my_char: char }
/// enum MyListChildren {
///    MyItem(MyItem),
///    MyOtherItem(MyOtherItem),
///    MyPredefinedType(MyPredefinedType)
/// }
/// struct MyList {
///    my_string: String,
///    my_int: u64,
///    children: Vec<MyListChildren>
/// }
/// # fn main() {}
/// ```
#[proc_macro]
pub fn xtype(input: TokenStream) -> TokenStream {
    let xtype = parse_macro_input!(input as xtype::XType);

    let expanded = quote! {
       #xtype
    };

    TokenStream::from(expanded)
}

/// The [xrender!](https://andrew-johnson-4.github.io/rdxl) macro defines a Display implementation for a type
///
/// <b>xrender!</b> implements the <b>Display</b> property for XHtml-like types.
/// The type definition is separate from the display logic for in the case that
/// a separate backend is desired
///
/// A typical invocation would look like this:
/// ```
/// # #![feature(proc_macro_hygiene)]
/// # use rdxl::{xtype,xrender};
/// # pub struct MyPredefinedType {}
/// # xtype!(<!MyList my_string:String my_int:u64>
/// #   <!MyItem my_bool:bool/>
/// #   <!MyOtherItem my_char:char/>
/// #   <?MyPredefinedType/>
/// # </MyList>);
/// xrender!(MyList, <ul>
///   <li>{{ self.my_string }}</li>
///   <li>{{ self.my_int }}</li>
///   {{ for i in self.children.iter() {{
///     {{ if let MyListChildren::MyItem(my_item) = i {{
///       <li>MyItem: {{ my_item.my_bool }}</li>
///     }} else if let MyListChildren::MyOtherItem(my_other_item) = i {{
///       <li>MyOtherItem: {{ my_other_item.my_char }}</li>
///     }} }}
///   }} }}
/// </ul>);
/// # fn main() {}
/// ```

#[proc_macro]
pub fn xrender(input: TokenStream) -> TokenStream {
    let xrender = parse_macro_input!(input as xrender::XRender);

    let xname = xrender.name;
    let xxhtml = xrender.xhtml;

    let expanded = quote! {
       impl std::fmt::Display for #xname {
          fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
              let mut stream = String::new();
              #xxhtml
     
              //buffering to a String is faster than many writes to the Formatter
              f.write_str(&stream)
          }
       }
    };

    TokenStream::from(expanded)
}

