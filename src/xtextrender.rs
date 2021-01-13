// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;
use crate::xtext::Xtext;

pub struct XtextRender {
   pub name: Ident,
   pub comma: Token![,],
   pub xtext: Xtext
}
impl Parse for XtextRender {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(XtextRender {
            name: input.parse()?,
            comma: input.parse()?,
            xtext: input.parse()?
        })
    }
}
