// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{ToTokens};
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{Ident as SynIdent,Type,Token};

pub struct XTypeAttr {
   pub attr_name: Ident,
   pub eq: Token![=],
   pub attr_type: Type
}
impl Parse for XTypeAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(XTypeAttr {
            attr_name: input.parse()?,
            eq: input.parse()?,
            attr_type: input.parse()?
        })
    }
}

pub struct XType {
   pub open: Token![<],
   pub tag_name: SynIdent,
   pub tag_attrs: Vec<XTypeAttr>,
   pub tag_children: Vec<XType>,
   pub close: Token![>],
}
impl ToTokens for XType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    }
}
impl Parse for XType {
    fn parse(input: ParseStream) -> Result<Self> {
        let open: Token![<] = input.parse()?;
        let tag_name: SynIdent = input.parse()?;

        let mut tag_attrs = Vec::new();
        while input.peek(SynIdent) {
           let attr: XTypeAttr = input.parse()?;
           tag_attrs.push(attr);
        }

        if input.peek(Token![/]) {
           let _backslash: Token![/] = input.parse()?;
           let close: Token![>] = input.parse()?;
           Ok(XType {
              open: open,
              tag_name: tag_name,
              tag_attrs: tag_attrs,
              tag_children: Vec::new(),
              close: close
           })
        } else {
           let _close_opening_tag: Token![>] = input.parse()?;

           let mut children = Vec::new();
           while input.peek(Token![<]) && input.peek2(Token![!]) {
              let child: XType = input.parse()?;
              children.push(child);
           }

           let _open_close: Token![<] = input.parse()?;
           let _open_close2: Token![/] = input.parse()?;
           let close_ident: Ident = input.parse()?;

           if tag_name.to_string() != close_ident.to_string() {
              let msg = format!("Expected </{}> found </{}>", tag_name, close_ident);
              let r = Error::new(close_ident.span(), msg);
              return Err(r)
           }

           let close: Token![>] = input.parse()?;
           Ok(XType {
              open: open,
              tag_name: tag_name,
              tag_attrs: tag_attrs,
              tag_children: children,
              close: close
           })
        }
    }
}
