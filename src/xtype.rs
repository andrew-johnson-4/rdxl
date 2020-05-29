// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{TokenStreamExt, ToTokens};
use proc_macro2::{Punct, Ident, Spacing, Group, Delimiter};
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{Ident as SynIdent,Type,Token};
use syn::spanned::Spanned;

pub struct XTypeAttr {
   pub attr_name: Ident,
   pub eq: Token![:],
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
       let span = self.open.span.join(self.close.span).unwrap();

       tokens.append(Ident::new("struct", span.clone()));
       tokens.append(Ident::new(&self.tag_name.to_string(), span.clone()));

       let mut ts = proc_macro2::TokenStream::new();
       for attr in self.tag_attrs.iter() {
          let span = attr.attr_name.span().join(attr.attr_type.span()).unwrap();
          ts.append(Ident::new(&attr.attr_name.to_string(), span.clone()));
          ts.append(Punct::new(':', Spacing::Alone));
          attr.attr_type.to_tokens(&mut ts);
          ts.append(Punct::new(',', Spacing::Alone));
       }

       if self.tag_children.len()==1 {
          ts.append(Ident::new("children", span.clone()));
          ts.append(Punct::new(':', Spacing::Alone));
          ts.append(Ident::new("Vec", span.clone()));
          ts.append(Punct::new('<', Spacing::Alone));
          ts.append(Ident::new(&self.tag_children[0].tag_name.to_string(), span.clone()));
          ts.append(Punct::new('>', Spacing::Alone));
          ts.append(Punct::new(',', Spacing::Alone));
       } else if self.tag_children.len()>1 {
          ts.append(Ident::new("children", span.clone()));
          ts.append(Punct::new(':', Spacing::Alone));
          ts.append(Ident::new("Vec", span.clone()));
          ts.append(Punct::new('<', Spacing::Alone));
          ts.append(Ident::new(&format!("{}Children", self.tag_name.to_string()), span.clone()));
          ts.append(Punct::new('>', Spacing::Alone));
          ts.append(Punct::new(',', Spacing::Alone));
       }

       let gr = Group::new(Delimiter::Brace, ts);
       tokens.append(gr);

       if self.tag_children.len()>1 {
          tokens.append(Ident::new("enum", span.clone()));
          tokens.append(Ident::new(&format!("{}Children", self.tag_name.to_string()), span.clone()));

          let mut ts = proc_macro2::TokenStream::new();
          for child in self.tag_children.iter() {
             ts.append(Ident::new(&child.tag_name.to_string(), span.clone()));
             let mut ets = proc_macro2::TokenStream::new();
             ets.append(Ident::new(&child.tag_name.to_string(), span.clone()));
             let egr = Group::new(Delimiter::Parenthesis, ets);
             ts.append(egr);
             ts.append(Punct::new(',', Spacing::Alone));
          }

          let gr = Group::new(Delimiter::Brace, ts);
          tokens.append(gr);
       }

       for child in self.tag_children.iter() {
          child.to_tokens(tokens);
       }
    }
}

impl Parse for XType {
    fn parse(input: ParseStream) -> Result<Self> {
        let open: Token![<] = input.parse()?;
        let _exc: Token![!] = input.parse()?;
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
