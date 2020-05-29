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

use proc_macro::{TokenStream};
use syn::{parse_macro_input};
use quote::{quote};

#[proc_macro]
pub fn xhtml(input: TokenStream) -> TokenStream {
    let rdxls = parse_macro_input!(input as xhtml::Rdxl);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        {
            let mut stream = String::new();
            #rdxls
            stream
        }
    };

    TokenStream::from(expanded)
}
