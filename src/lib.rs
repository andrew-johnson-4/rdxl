// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

//! # HTML Templating Macros
//!
//! This library contains JSX style inline HTML macros for Rust. The only
//! dependencies are the ubiquitous proc-macro2, syn, and quote.
//!
//! The library is semantic versioned, so expect small fixes and many additions until a 1.0 release is reached.
//!
//! # Inline Rust expressions
//!
//! ```no_run
//! let x = 5;
//! let y = "asdf";
//! xhtml!({{ x }}, {{ y }})
//! ```
//!
//! # Conditional expressions
//!
//! ```no_run
//! let x = 5;
//! let y = "asdf";
//! xhtml!({{ if x>3 {{
//!   Case One
//! }} else if x>2 {{
//!   Case Two
//! }} else {{
//!   Case Three
//! }} }})
//! ```
//!
//! # Loop expressions
//!
//! ```no_run
//! xhtml!(<ul>{{ for i in 0..10 {{
//!   <li>{{ i }}</li>
//! }} }}</ul>)
//! ```
//!
//! # Miscellaneous expressions
//!
//! ```no_run
//! xhtml!(
//!   {{ let x = 5; }}
//!   {{ x }}
//! )
//! ```
//!
//! # Html attributes
//! ```no_run
//! xhtml!(<div style={{ "\"color:red;\"" }}>
//!   inside div
//! </div>)
//! ```

#![recursion_limit = "128"]
#![feature(type_ascription)]
#![crate_type = "proc-macro"]

mod xhtml;
mod xtype;
mod xrender;

use proc_macro::{TokenStream};
use syn::{parse_macro_input};
use quote::{quote};

/// The xhtml! macro is the primary mechanism for templating in rdxl
///
/// <b>xhtml!</b> consumes mixed Rust code and XML markup as input and emits rendered xhtml to a string buffer.
/// Rust code is usually delimited by {{double braces}} or [[double brackets]]. The <b>syn</b> module is used to
/// allow most Rust expressions to be used inside the correct delimited contexts. Control flow structures
/// such as if/else blocks, loops, and let statements to be used inline.
///
/// Aside from standard XML syntax, custom types may be defined with <b>xtype!</b> and <b>xrender!</b> facilities. This
/// encourages typesafe modular templates to be created and shared.
///
/// Use of <b>xhtml!</b> usually looks something like this:
/// ```no_run
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
/// </div>));
/// ```
#[proc_macro]
pub fn xhtml(input: TokenStream) -> TokenStream {
    let xhtmls = parse_macro_input!(input as xhtml::Xhtml);

    let expanded = quote! {
        {
            let mut stream = String::new();
            #xhtmls
            stream
        }
    };

    TokenStream::from(expanded)
}

/// The xtype! macro defines an xml element and subelements
#[proc_macro]
pub fn xtype(input: TokenStream) -> TokenStream {
    let xtype = parse_macro_input!(input as xtype::XType);

    let expanded = quote! {
       #xtype
    };

    TokenStream::from(expanded)
}

/// The xrender! macro defines a Display implementation for a type
#[proc_macro]
pub fn xrender(input: TokenStream) -> TokenStream {
    let xrender = parse_macro_input!(input as xrender::XRender);

    let xname = xrender.name;
    let xxhtml = xrender.xhtml;

    let expanded = quote! {
       impl std::fmt::Display for #xname {
          fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
              let mut stream = String::new();
              #xxhtml
     
              f.write_str(&stream)
          }
       }
    };

    TokenStream::from(expanded)
}

