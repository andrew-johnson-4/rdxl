// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;
use crate::xhtml::Xhtml;

pub struct XRender {
   pub name: Ident,
   pub comma: Token![,],
   pub xhtml: Xhtml
}
impl Parse for XRender {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(XRender {
            name: input.parse()?,
            comma: input.parse()?,
            xhtml: input.parse()?
        })
    }
}
