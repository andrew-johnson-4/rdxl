// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{format_ident, quote_spanned,TokenStreamExt, ToTokens};
use proc_macro2::{Ident, Group, Delimiter};
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
   pub defined: bool,
   pub tag_name: String,
   pub tag_attrs: Vec<XTypeAttr>,
   pub tag_children: Vec<XType>,
   pub close: Token![>],
}

impl ToTokens for XType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       if self.defined { return; }
       let span = self.open.span.join(self.close.span).unwrap();

       let tag_name = format_ident!("{}", self.tag_name, span=span);
       (quote_spanned! {span=>
          pub struct #tag_name
       }).to_tokens(tokens);

       let mut ts = proc_macro2::TokenStream::new();
       for XTypeAttr { attr_name, attr_type, .. } in self.tag_attrs.iter() {
          let span = attr_name.span().join(attr_type.span()).unwrap();
          (quote_spanned! {span=>
             pub #attr_name : #attr_type,
          }).to_tokens(&mut ts);
       }

       let child_type = format_ident!("{}Children", self.tag_name, span=span);
       (quote_spanned! {span=>
          pub children : Vec<#child_type>,
       }).to_tokens(&mut ts);

       (quote_spanned! {span=> {#ts}}).to_tokens(tokens);

       (quote_spanned! {span=> pub enum #child_type}).to_tokens(tokens);

       let mut ts = proc_macro2::TokenStream::new();
       for child in self.tag_children.iter() {
          if child.tag_name == "Display" {
             (quote_spanned! {span=> Display(Box<dyn std::fmt::Display>),}).to_tokens(&mut ts);
          } else {
             let child_tag = format_ident!("{}", child.tag_name, span=span);
             (quote_spanned! {span=> #child_tag(#child_tag),}).to_tokens(&mut ts);
          }
       }

       let gr = Group::new(Delimiter::Brace, ts);
       tokens.append(gr);

       for child in self.tag_children.iter() {
          child.to_tokens(tokens);
       }
    }
}

impl Parse for XType {
    fn parse(input: ParseStream) -> Result<Self> {
        let open: Token![<] = input.parse()?;

        if input.peek(Token![?]) && input.peek2(Token![/]) {
           let _q: Token![?] = input.parse()?;
           let _s: Token![/] = input.parse()?;
           let close: Token![>] = input.parse()?;

           return Ok(XType {
              open: open,
              defined: true,
              tag_name: "Display".to_string(),
              tag_attrs: Vec::new(),
              tag_children: Vec::new(),
              close: close
           })
        } else if input.peek(Token![?]) {
           let _q: Token![?] = input.parse()?;
           let tag_name: SynIdent = input.parse()?;
           let tag_name = tag_name.to_string();
           let _s: Token![/] = input.parse()?;
           let close: Token![>] = input.parse()?;

           return Ok(XType {
              open: open,
              defined: true,
              tag_name: tag_name,
              tag_attrs: Vec::new(),
              tag_children: Vec::new(),
              close: close
           })
        }

        let _exc: Token![!] = input.parse()?;
        let tag_name: SynIdent = input.parse()?;
        let tag_name = tag_name.to_string();

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
              defined: false,
              tag_name: tag_name,
              tag_attrs: tag_attrs,
              tag_children: Vec::new(),
              close: close
           })
        } else {
           let _close_opening_tag: Token![>] = input.parse()?;

           let mut children = Vec::new();
           while input.peek(Token![<]) && (input.peek2(Token![!]) || input.peek2(Token![?])) {
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
              defined: false,
              tag_name: tag_name,
              tag_attrs: tag_attrs,
              tag_children: children,
              close: close
           })
        }
    }
}
