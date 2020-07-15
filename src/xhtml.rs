
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{format_ident, quote_spanned, TokenStreamExt, ToTokens};
use proc_macro2::{Span, Literal};
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{Ident, Token, Expr, Pat, LitChar, LitBool, LitStr, LitInt, bracketed, braced};
use syn::token::{Bracket,Brace};
use syn::spanned::Spanned;

pub enum XhtmlDisplay {
   E(Expr),
   X(Xhtml)
}
impl ToTokens for XhtmlDisplay {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      match self {
         XhtmlDisplay::E(e) => { e.to_tokens(tokens); }
         XhtmlDisplay::X(xhtmls) => {
            let expanded = quote_spanned! { xhtmls.gen_span() =>
               {
                  let mut stream = String::new();
                  #xhtmls
                  stream
               }
            };
            expanded.to_tokens(tokens);
         }
      }
   }
}

pub struct XhtmlDisplayExpr {
   open: Token![<],
   expr: XhtmlDisplay,
   close: Token![>],
}
impl XhtmlDisplayExpr {
    pub fn gen_span(&self) -> Span {
       self.open.span.join(self.close.span).unwrap_or(self.open.span)
    }
}
impl Parse for XhtmlDisplayExpr {
    fn parse(input: ParseStream) -> Result<Self> {
       let open: Token![<] = input.parse()?;
       let _: Token![?] = input.parse()?;
       let _: Token![>] = input.parse()?;

       let expr = if input.peek(Brace) {
          let content;
          let content2;
          let _ = braced!(content in input);
          let _ = braced!(content2 in content);
          let expr: Expr = content2.parse()?;
          XhtmlDisplay::E(expr)
       } else {
          let xhtml: Xhtml = input.parse()?;
          XhtmlDisplay::X(xhtml)
       };

       let _: Token![<] = input.parse()?;
       let _: Token![/] = input.parse()?;
       let _: Token![?] = input.parse()?;
       let close: Token![>] = input.parse()?;

       Ok(XhtmlDisplayExpr {
          open: open,
          expr: expr,
          close: close,
       })
    }
}
impl ToTokens for XhtmlDisplayExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       self.expr.to_tokens(tokens);
    }
}

pub struct XhtmlExprF {
   bracket: Bracket,
   context: String,
   expr: Expr
}
impl ToTokens for XhtmlExprF {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       let ref expr = self.expr;
       let coerce = format_ident!("to_{}", self.context, span=self.bracket.span);

       (quote_spanned! {self.bracket.span=>
          stream.push_str(&#expr.#coerce());
       }).to_tokens(tokens);
    }
}
impl XhtmlExprF {
    fn gen_span(&self) -> Span {
       self.bracket.span
    }
    fn parse(context: String, input: ParseStream) -> Result<Self> {
       let content;
       let content2;
       let bracket1 = bracketed!(content in input);
       let _bracket2 = bracketed!(content2 in content);
       let expr: Expr = content2.parse()?;
       Ok(XhtmlExprF{ bracket:bracket1, context:context, expr:expr })
    }
}

enum XhtmlExprE {
   S(Expr),
   E(Expr),
   F(Token![for],Pat,Expr,Vec<XhtmlCrumb>),
   W(Token![while],Expr,Vec<XhtmlCrumb>),
   L(Token![let],Pat,Expr),
   I(Token![if],Expr,Vec<XhtmlCrumb>,Vec<(Expr,Vec<XhtmlCrumb>)>,Vec<XhtmlCrumb>),
   P(Token![loop],Vec<XhtmlCrumb>),
}
impl XhtmlExprE {
    fn does_emit(&self) -> bool {
       match self {
          XhtmlExprE::S(_) => { false },
          XhtmlExprE::E(_) => { true },
          XhtmlExprE::F(_,_,_,_) => { true },
          XhtmlExprE::P(_,_) => { true },
          XhtmlExprE::W(_,_,_) => { true },
          XhtmlExprE::L(_,_,_) => { false },
          XhtmlExprE::I(_,_,_,_,_) => { true },
       }
    }
}
impl ToTokens for XhtmlExprE {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           XhtmlExprE::E(e) => {
              (quote_spanned!{e.span()=>
                 stream.push_str(&#e.to_string());
              }).to_tokens(tokens);
           }, XhtmlExprE::S(e) => {
              (quote_spanned!{e.span()=>
                 #e;
              }).to_tokens(tokens);
           }, XhtmlExprE::F(f,p,i,cs) => {
              (quote_spanned!{f.span=>
                 for #p in #i { #(#cs)* stream.push_str(" "); }
              }).to_tokens(tokens);
           }, XhtmlExprE::P(l,cs) => {
              (quote_spanned!{l.span=>
                 loop { #(#cs)* stream.push_str(" "); }
              }).to_tokens(tokens);
           }, XhtmlExprE::I(i,c,bs,es,e) => {
              (quote_spanned!{i.span=>
                if #c { #(#bs)* stream.push_str(" "); }
              }).to_tokens(tokens);

              for (c,e) in es.iter() {
                 (quote_spanned!{i.span=>
                    else if #c { #(#e)* stream.push_str(" "); }
                 }).to_tokens(tokens);
              }

              if e.len() > 0 {
                 (quote_spanned!{i.span=>
                    else { #(#e)* stream.push_str(" "); }
                 }).to_tokens(tokens);
              }
           }, XhtmlExprE::W(w,i,cs) => {
              (quote_spanned!{w.span=>
                 while #i { #(#cs)* stream.push_str(" "); }
              }).to_tokens(tokens);
           }, XhtmlExprE::L(t,l,e) => {
              (quote_spanned!{t.span=>
                 let #l = #e;
              }).to_tokens(tokens);
           }
        }
    }
}
impl Parse for XhtmlExprE {
    fn parse(input: ParseStream) -> Result<Self> {
       if input.peek(Token![for]) {
          let _for: Token![for] = input.parse()?;
          let pat: Pat = input.parse()?;
          let _in: Token![in] = input.parse()?;
          let iter: Expr = input.parse()?;
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XhtmlCrumb> = content2.call(XhtmlCrumb::parse_outer)?;
          Ok(XhtmlExprE::F(_for,pat,iter,body))
       } else if input.peek(Token![loop]) {
          let _loop: Token![loop] = input.parse()?;
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XhtmlCrumb> = content2.call(XhtmlCrumb::parse_outer)?;
          Ok(XhtmlExprE::P(_loop,body))
       } else if input.peek(Token![while]) {
          let _while: Token![while] = input.parse()?;
          let iter: Expr = input.parse()?;
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XhtmlCrumb> = content2.call(XhtmlCrumb::parse_outer)?;
          Ok(XhtmlExprE::W(_while,iter,body))
       } else if input.peek(Token![if]) {
          let _if: Token![if] = input.parse()?;
          let b: Expr = input.parse()?;
          let mut es = Vec::new();
          let mut e = Vec::new();
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XhtmlCrumb> = content2.call(XhtmlCrumb::parse_outer)?;

          while input.peek(Token![else]) && input.peek2(Token![if]) {
             let _else: Token![else] = input.parse()?;
             let _if: Token![if] = input.parse()?;
             let b: Expr = input.parse()?;
             let content;
             let content2;
             let _brace1 = braced!(content in input);
             let _brace2 = braced!(content2 in content);
             let e = content2.call(XhtmlCrumb::parse_outer)?;
             es.push((b,e));
          }

          if input.peek(Token![else]) {
             let _else: Token![else] = input.parse()?;
             let content;
             let content2;
             let _brace1 = braced!(content in input);
             let _brace2 = braced!(content2 in content);
             e = content2.call(XhtmlCrumb::parse_outer)?;
          }

          Ok(XhtmlExprE::I(_if,b,body,es,e))
       } else if input.peek(Token![let]) {
          let _let: Token![let] = input.parse()?;
          let pat: Pat = input.parse()?;
          let _eq: Token![=] = input.parse()?;
          let expr: Expr = input.parse()?;
          if input.peek(Token![;]) {
             let _: Token![;] = input.parse()?;
          }
          Ok(XhtmlExprE::L(_let,pat,expr))
       } else {
          let e: Expr = input.parse()?;
          if input.peek(Token![;]) {
             let _semi: Token![;] = input.parse()?;
             Ok(XhtmlExprE::S(e))
          } else {
             Ok(XhtmlExprE::E(e))
          }
       }
    }
}

pub struct XhtmlExpr {
   brace_token1: Brace,
   _brace_token2: Brace,
   expr: XhtmlExprE
}
impl XhtmlExpr {
    fn does_emit(&self) -> bool {
       self.expr.does_emit()
    }
}
impl Parse for XhtmlExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let _content;
        let content2;
        Ok(XhtmlExpr {
           brace_token1: braced!(_content in input),
           _brace_token2: braced!(content2 in _content),
           expr: content2.call(XhtmlExprE::parse)?,
        })
    }
}
impl ToTokens for XhtmlExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.expr.to_tokens(tokens)
    }
}

pub enum XhtmlClassAttr {
   Cl(XhtmlClass),
   F(Bracket,String,Expr),
   E(Brace,Expr),
   B(LitBool,bool),
   C(LitChar,char),
   U(LitInt,u64),
   S(LitStr,String),
}
impl XhtmlClassAttr {
   fn gen_span(&self) -> Span {
      match self {
         XhtmlClassAttr::Cl(cl) => { cl.span() },
         XhtmlClassAttr::F(b,_,_) => { b.span },
         XhtmlClassAttr::E(b,_) => { b.span },
         XhtmlClassAttr::B(v,_) => { v.span() },
         XhtmlClassAttr::C(v,_) => { v.span() },
         XhtmlClassAttr::U(v,_) => { v.span() },
         XhtmlClassAttr::S(v,_) => { v.span() },
      }
   }
   fn parse(input: ParseStream, key: String) -> Result<Self> {
      if input.peek(Bracket) {
         let _content;
         let content2;
         let bracket_token1:Bracket = bracketed!(_content in input);
         let _bracket_token2:Bracket = bracketed!(content2 in _content);
         let e: Expr = content2.parse()?;
         Ok(XhtmlClassAttr::F(bracket_token1,key,e))
      } else if input.peek(Brace) {
         let _content;
         let content2;
         let brace_token1:Brace = braced!(_content in input);
         let _brace_token2:Brace = braced!(content2 in _content);
         let e: Expr = content2.parse()?;
         Ok(XhtmlClassAttr::E(brace_token1,e))
      } else if input.peek(LitBool) {
         let b: LitBool = input.parse()?;
         Ok(XhtmlClassAttr::B(b.clone(),b.value))
      } else if input.peek(LitInt) {
         let b: LitInt = input.parse()?;
         let u: u64 = b.base10_parse()?;
         Ok(XhtmlClassAttr::U(b.clone(),u))
      } else if input.peek(LitChar) {
         let b: LitChar = input.parse()?;
         Ok(XhtmlClassAttr::C(b.clone(),b.value()))
      } else if input.peek(Token![<]) && input.peek2(Token![!]) {
         let cl: XhtmlClass = input.parse()?;
         Ok(XhtmlClassAttr::Cl(cl))
      } else {
         let val: LitStr = input.parse()?;
         Ok(XhtmlClassAttr::S(val.clone(),val.value()))
      }
   }
}
impl ToTokens for XhtmlClassAttr {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let span = self.gen_span();
      match self {
         XhtmlClassAttr::S(_,s) => {
            let l: Literal = Literal::string(&s);
            (quote_spanned!{span=>
               #l.to_string()
            }).to_tokens(tokens);
         }, XhtmlClassAttr::B(_,e) => {
            let e = format_ident!("{}", e, span=span);
            e.to_tokens(tokens);
         }, XhtmlClassAttr::Cl(cl) => {
            cl.to_tokens(tokens);
         }, XhtmlClassAttr::C(_,e) => {
            let l: Literal = Literal::character(*e);
            tokens.append(l);
         }, XhtmlClassAttr::U(_,e) => {
            let l: Literal = Literal::u64_unsuffixed(*e);
            tokens.append(l);
         }, XhtmlClassAttr::F(_,f,e) => {
            let coerce = format_ident!("to_{}", f, span=span);
            (quote_spanned!{span=>
               #e.#coerce()
            }).to_tokens(tokens);
         }, XhtmlClassAttr::E(_,e) => {
            e.to_tokens(tokens);
         }
      }
   }
}

pub enum XhtmlAttr {
   S(String),
   F(XhtmlExprF),
   E(XhtmlExpr)
}
impl XhtmlAttr {
   fn parse(input: ParseStream, key: String) -> Result<Self> {
      if input.peek(Bracket) {
         let f: XhtmlExprF = XhtmlExprF::parse(key.clone(),input)?;
         Ok(XhtmlAttr::F(f))
      } else if input.peek(Brace) {
         let e: XhtmlExpr = input.parse()?;
         Ok(XhtmlAttr::E(e))
      } else if input.peek(LitBool) {
         let b: LitBool = input.parse()?;
         Ok(XhtmlAttr::S(format!("{}", b.value)))
      } else if input.peek(LitInt) {
         let b: LitInt = input.parse()?;
         Ok(XhtmlAttr::S(format!("{}", b.base10_digits())))
      } else if input.peek(LitChar) {
         let b: LitChar = input.parse()?;
         Ok(XhtmlAttr::S(format!("'{}'", b.value())))
      } else {
         let val: LitStr = input.parse()?;
         Ok(XhtmlAttr::S(format!("{:?}",val.value())))
      }
   }
}


pub enum XhtmlClassChild {
   C(XhtmlClass),
   D(XhtmlDisplayExpr)
}
impl Parse for XhtmlClassChild {
    fn parse(input: ParseStream) -> Result<Self> {
       if input.peek(Token![<]) && input.peek2(Token![?]) {
          let d: XhtmlDisplayExpr = input.parse()?;
          Ok(XhtmlClassChild::D(d))
       } else {
          let c: XhtmlClass = input.parse()?;
          Ok(XhtmlClassChild::C(c))
       }
    }
}

pub struct XhtmlClass {
   open: Token![<],
   name: String,
   attrs: Vec<(String,XhtmlClassAttr)>,
   children: Vec<XhtmlClassChild>,
   close: Token![>]
}
impl XhtmlClass {
    fn gen_span(&self) -> Span {
       self.open.span.join(self.close.span).unwrap_or(self.open.span)
    }
}
impl ToTokens for XhtmlClass {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       let span = self.gen_span();

       let name = format_ident!("{}", self.name, span=span);
       (quote_spanned!{span=>
          #name::new()
       }).to_tokens(tokens);

       for (k,v) in self.attrs.iter() {
          let setter = format_ident!("set_{}", k, span=span);
          (quote_spanned!{span=>
               .#setter(#v)
          }).to_tokens(tokens);
       }

       let mut cs = proc_macro2::TokenStream::new();
       for c in self.children.iter() {
          match c {
             XhtmlClassChild::C(c) => {
                let span = c.gen_span();
                let child_enum = format_ident!("{}Children", self.name, span=span);
                let child_tag = format_ident!("{}", c.name, span=span);
                (quote_spanned!{span=>
                   #child_enum::#child_tag(#c),
                }).to_tokens(&mut cs);
             },
             XhtmlClassChild::D(d) => {
                let span = d.gen_span();
                let child_enum = format_ident!("{}Children", self.name, span=span);
                (quote_spanned!{span=>
                   #child_enum::Display(Box::new(#d)),
                }).to_tokens(&mut cs);
             }
          }
       }

       (quote_spanned!{span=>
         .set_children(vec![#cs])
       }).to_tokens(tokens);
    }
}
impl Parse for XhtmlClass {
    fn parse(input: ParseStream) -> Result<Self> {
       let open: Token![<] = input.parse()?;
       let _ex: Token![!] = input.parse()?;
       let name: Ident = input.parse()?;

       let mut attrs = Vec::new();
       while input.peek(Ident) {
          let attr_name: Ident = input.parse()?;
          let _eq: Token![=] = input.parse()?;
          let attr_val = XhtmlClassAttr::parse(input, attr_name.to_string())?;
          attrs.push((attr_name.to_string(), attr_val));
       }

       if input.peek(Token![/]) {
          let _slash: Token![/] = input.parse()?;
          let close: Token![>] = input.parse()?;
          Ok(XhtmlClass {
             open: open,
             name: name.to_string(),
             attrs: attrs,
             children: Vec::new(),
             close: close
          })
       } else {
          let _gt: Token![>] = input.parse()?;
          
          let mut children = Vec::new();
          while !(input.peek(Token![<]) && input.peek2(Token![/])) {
             let c: XhtmlClassChild = input.parse()?;
             children.push(c);
          }

          let _lt: Token![<] = input.parse()?;
          let _slash: Token![/] = input.parse()?;

          let close_tag: Ident = input.parse()?;
          if name.to_string() != close_tag.to_string() {
              let msg = format!("Expected </{}> found </{}>", name, close_tag);
              let r = Error::new(close_tag.span(), msg);
              return Err(r)
           }

          let close: Token![>] = input.parse()?;

          Ok(XhtmlClass {
             open: open,
             name: name.to_string(),
             attrs: attrs,
             children: children,
             close: close
          })
       }
    }
}

pub enum XhtmlAttrKey {
   S(String),
   G(Expr,String)
}

pub struct XhtmlTag {
   tag: String,
   attrs: Vec<(XhtmlAttrKey,Option<XhtmlAttr>)>,
   inner: Xhtml,
   outer_span: Span,
   inner_span_start: Span,
   inner_span_end: Span,
}
impl ToTokens for XhtmlTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let open_tag = Literal::string(&format!("<{}", self.tag));
        (quote_spanned!{self.outer_span=>
           stream.push_str(#open_tag);
        }).to_tokens(tokens);

        for (k,v) in self.attrs.iter() {
            match (k,v) {
               (XhtmlAttrKey::S(k),None) => {
                  let l = Literal::string(&format!(" {}", k));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::S(k),Some(XhtmlAttr::S(s))) => {
                  let l = Literal::string(&format!(" {}={}", k, s));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::S(k),Some(XhtmlAttr::F(f))) => {
                  let l = Literal::string(&format!(" {}=", k));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                     stream.push_str("\""); 
                     stream.push_str(&{
                       let mut stream = String::new();
                       #f
                       stream.replace("\"", "\\\"")
                     });
                     stream.push_str("\""); 
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::S(k),Some(XhtmlAttr::E(e))) => {
                  let l = Literal::string(&format!(" {}=", k));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                     stream.push_str("\""); 
                     stream.push_str(&{
                       let mut stream = String::new();
                       #e
                       stream.replace("\"", "\\\"")
                     });
                     stream.push_str("\""); 
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::G(g,k),None) => {
                  let l = Literal::string(&format!(" {}", k));
                  (quote_spanned!{self.outer_span=>
                     if #g { stream.push_str(#l); }
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::G(g,k),Some(XhtmlAttr::S(s))) => {
                  let l = Literal::string(&format!(" {}={}", k, s));
                  (quote_spanned!{self.outer_span=>
                     if #g { stream.push_str(#l); }
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::G(g,k),Some(XhtmlAttr::F(f))) => {
                  let l = Literal::string(&format!(" {}=", k));
                  (quote_spanned!{self.outer_span=>
                     if #g {
                        stream.push_str(#l);
                        stream.push_str("\""); 
                        stream.push_str(&{
                          let mut stream = String::new();
                          #f
                          stream.replace("\"", "\\\"")
                        });
                        stream.push_str("\""); 
                     }
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::G(g,k),Some(XhtmlAttr::E(e))) => {
                  let l = Literal::string(&format!(" {}=", k));
                  (quote_spanned!{self.outer_span=>
                     if #g {
                        stream.push_str(#l);
                        stream.push_str("\""); 
                        stream.push_str(&{
                          let mut stream = String::new();
                          #e
                          stream.replace("\"", "\\\"")
                        });
                        stream.push_str("\""); 
                     }
                  }).to_tokens(tokens);
               }
            }
        }

        let self_closing = vec!["area","base","br","embed","hr","iframe","img",
           "input","link","meta","param","source","track"];
        if self.inner.crumbs.len()==0 && self_closing.iter().any(|s| (&self.tag)==s) {
           let l = Literal::string("/>");
           (quote_spanned!{self.outer_span=>
              stream.push_str(#l);
           }).to_tokens(tokens);
        } else {
           let l = Literal::string(">");
           (quote_spanned!{self.outer_span=>
              stream.push_str(#l);
           }).to_tokens(tokens);

           if self.inner.crumbs.len()>0 && self.inner.gen_span().start() > self.inner_span_start.end() {
              let l = Literal::string(" ");
              (quote_spanned!{self.outer_span=>
                 stream.push_str(#l);
              }).to_tokens(tokens);
           }

           self.inner.to_tokens(tokens);

           if self.inner.crumbs.len()>0 && self.inner.gen_span().end() < self.inner_span_end.start() {
              let l = Literal::string(" ");
              (quote_spanned!{self.outer_span=>
                 stream.push_str(#l);
              }).to_tokens(tokens);
           }

           let l = Literal::string(&format!("</{}>", self.tag));
           (quote_spanned!{self.outer_span=>
              stream.push_str(#l);
           }).to_tokens(tokens);
        }
    }
}
impl Parse for XhtmlTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let l1: Token![<] = input.parse()?;
        let t: Ident = input.parse()?;

        let mut attrs: Vec<(XhtmlAttrKey,Option<XhtmlAttr>)> = Vec::new();
        while input.peek(Ident) ||
              input.peek(LitStr) ||
              input.peek(Token![as]) ||
              input.peek(Token![break]) ||
              input.peek(Token![const]) ||
              input.peek(Token![continue]) ||
              input.peek(Token![crate]) ||
              input.peek(Token![else]) ||
              input.peek(Token![enum]) ||
              input.peek(Token![extern]) ||
              input.peek(Token![fn]) ||
              input.peek(Token![for]) ||
              input.peek(Token![if]) ||
              input.peek(Token![impl]) ||
              input.peek(Token![in]) ||
              input.peek(Token![let]) ||
              input.peek(Token![loop]) ||
              input.peek(Token![match]) ||
              input.peek(Token![mod]) ||
              input.peek(Token![move]) ||
              input.peek(Token![mut]) ||
              input.peek(Token![pub]) ||
              input.peek(Token![ref]) ||
              input.peek(Token![return]) ||
              input.peek(Token![self]) ||
              input.peek(Token![Self]) ||
              input.peek(Token![static]) ||
              input.peek(Token![struct]) ||
              input.peek(Token![super]) ||
              input.peek(Token![trait]) ||
              input.peek(Token![type]) ||
              input.peek(Token![unsafe]) ||
              input.peek(Token![use]) ||
              input.peek(Token![where]) ||
              input.peek(Token![while]) ||
              input.peek(Brace) {
            if input.peek(Brace) {
               let content1;
               let content2;
               let _brace1: Brace = braced!(content1 in input);
               let _brace2: Brace = braced!(content2 in content1);
               let _if: Token![if] = content2.parse()?;
               let expr: Expr = content2.parse()?;

               let content3;
               let content4;
               let _brace3: Brace = braced!(content3 in content2);
               let _brace4: Brace = braced!(content4 in content3);
               let key = if content4.peek(LitStr) { let s:LitStr = content4.parse()?; s.value()
                         } else { let key: Ident = content4.parse()?; key.to_string() };
               let v = if content4.peek(Token![=]) {
                  let _eq: Token![=] = content4.parse()?;
                  let attr_expr: XhtmlAttr = XhtmlAttr::parse(&content4, key.clone())?;
                  Some(attr_expr)
               } else { None };
               attrs.push(( XhtmlAttrKey::G(expr,key), v ));
            } else {
               let key = if input.peek(Token![as]) { let _:Token![as] = input.parse()?; "as".to_string()
               } else if input.peek(Token![abstract]) { let _:Token![abstract] = input.parse()?; "abstract".to_string()
               } else if input.peek(Token![become]) { let _:Token![become] = input.parse()?; "become".to_string()
               } else if input.peek(Token![box]) { let _:Token![box] = input.parse()?; "box".to_string()
               } else if input.peek(Token![break]) { let _:Token![break] = input.parse()?; "break".to_string()
               } else if input.peek(Token![const]) { let _:Token![const] = input.parse()?; "const".to_string()
               } else if input.peek(Token![continue]) { let _:Token![continue] = input.parse()?; "continue".to_string()
               } else if input.peek(Token![crate]) { let _:Token![crate] = input.parse()?; "crate".to_string()
               } else if input.peek(Token![do]) { let _:Token![do] = input.parse()?; "do".to_string()
               } else if input.peek(Token![else]) { let _:Token![else] = input.parse()?; "else".to_string()
               } else if input.peek(Token![enum]) { let _:Token![enum] = input.parse()?; "enum".to_string()
               } else if input.peek(Token![extern]) { let _:Token![extern] = input.parse()?; "extern".to_string()
               } else if input.peek(Token![final]) { let _:Token![final] = input.parse()?; "final".to_string()
               } else if input.peek(Token![fn]) { let _:Token![fn] = input.parse()?; "fn".to_string()
               } else if input.peek(Token![for]) { let _:Token![for] = input.parse()?; "for".to_string()
               } else if input.peek(Token![if]) { let _:Token![if] = input.parse()?; "if".to_string()
               } else if input.peek(Token![impl]) { let _:Token![impl] = input.parse()?; "impl".to_string()
               } else if input.peek(Token![in]) { let _:Token![in] = input.parse()?; "in".to_string()
               } else if input.peek(Token![let]) { let _:Token![let] = input.parse()?; "let".to_string()
               } else if input.peek(Token![loop]) { let _:Token![loop] = input.parse()?; "loop".to_string()
               } else if input.peek(Token![macro]) { let _:Token![macro] = input.parse()?; "macro".to_string()
               } else if input.peek(Token![match]) { let _:Token![match] = input.parse()?; "match".to_string()
               } else if input.peek(Token![mod]) { let _:Token![mod] = input.parse()?; "mod".to_string()
               } else if input.peek(Token![move]) { let _:Token![move] = input.parse()?; "move".to_string()
               } else if input.peek(Token![mut]) { let _:Token![mut] = input.parse()?; "mut".to_string()
               } else if input.peek(Token![override]) { let _:Token![override] = input.parse()?; "override".to_string()
               } else if input.peek(Token![priv]) { let _:Token![priv] = input.parse()?; "priv".to_string()
               } else if input.peek(Token![pub]) { let _:Token![pub] = input.parse()?; "pub".to_string()
               } else if input.peek(Token![ref]) { let _:Token![ref] = input.parse()?; "ref".to_string()
               } else if input.peek(Token![return]) { let _:Token![return] = input.parse()?; "return".to_string()
               } else if input.peek(Token![self]) { let _:Token![self] = input.parse()?; "self".to_string()
               } else if input.peek(Token![Self]) { let _:Token![Self] = input.parse()?; "Self".to_string()
               } else if input.peek(Token![static]) { let _:Token![static] = input.parse()?; "static".to_string()
               } else if input.peek(Token![struct]) { let _:Token![struct] = input.parse()?; "struct".to_string()
               } else if input.peek(Token![super]) { let _:Token![super] = input.parse()?; "super".to_string()
               } else if input.peek(Token![trait]) { let _:Token![trait] = input.parse()?; "trait".to_string()
               } else if input.peek(Token![type]) { let _:Token![type] = input.parse()?; "type".to_string()
               } else if input.peek(Token![typeof]) { let _:Token![typeof] = input.parse()?; "typeof".to_string()
               } else if input.peek(Token![unsafe]) { let _:Token![unsafe] = input.parse()?; "unsafe".to_string()
               } else if input.peek(Token![unsized]) { let _:Token![unsized] = input.parse()?; "unsized".to_string()
               } else if input.peek(Token![use]) { let _:Token![use] = input.parse()?; "use".to_string()
               } else if input.peek(Token![virtual]) { let _:Token![virtual] = input.parse()?; "virtual".to_string()
               } else if input.peek(Token![where]) { let _:Token![where] = input.parse()?; "where".to_string()
               } else if input.peek(Token![while]) { let _:Token![while] = input.parse()?; "while".to_string()
               } else if input.peek(Token![yield]) { let _:Token![yield] = input.parse()?; "yield".to_string()
               } else if input.peek(LitStr) { let s:LitStr = input.parse()?; s.value()
               } else { let key: Ident = input.parse()?; key.to_string() };
               let v = if input.peek(Token![=]) {
                  let _eq: Token![=] = input.parse()?;
                  let attr_expr: XhtmlAttr = XhtmlAttr::parse(input, key.clone())?;
                  Some(attr_expr)
               } else { None };
               attrs.push(( XhtmlAttrKey::S(key), v ));
           }
        }

        if input.peek(Token![/]) {
           let r1: Token![/] = input.parse()?;
           let r2: Token![>] = input.parse()?;

           Ok(XhtmlTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: Xhtml { crumbs: vec!() },
              outer_span: l1.span.join(r2.span).unwrap_or(l1.span),
              inner_span_start: r1.span.clone(),
              inner_span_end: l1.span.clone(),
           })
        } else {
           let l2: Token![>] = input.parse()?;

           let inner: Xhtml = input.parse()?;

           let r1: Token![<] = input.parse()?;
           let _r2: Token![/] = input.parse()?;
           let t2: Ident = input.parse()?;
           if t.to_string() != t2.to_string() {
              let msg = format!("Expected </{}> found </{}>", t, t2);
              let r = Error::new(t2.span(), msg);
              return Err(r)
           }
           let r3: Token![>] = input.parse()?;
        
           Ok(XhtmlTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: inner,
              outer_span: l1.span.join(r3.span).unwrap_or(l1.span),
              inner_span_start: l2.span.clone(),
              inner_span_end: r1.span.clone(),
           })
       }
    }
}

enum XhtmlCrumb {
   L(LitStr),
   S(String, Span),
   T(XhtmlTag),
   E(XhtmlExpr),
   F(XhtmlExprF),
   C(XhtmlClass)
}
impl XhtmlCrumb {
    fn does_emit(&self) -> bool {
       match self {
          XhtmlCrumb::L(_) => { true },
          XhtmlCrumb::S(_,_) => { true },
          XhtmlCrumb::T(_) => { true },
          XhtmlCrumb::E(e) => { e.does_emit() },
          XhtmlCrumb::F(_) => { true },
          XhtmlCrumb::C(_) => { true },
       }
    }
    fn span(&self) -> Span {
        match self {
            XhtmlCrumb::S(_,sp) => { sp.clone() }
            XhtmlCrumb::T(t) => { t.outer_span.clone() }
            XhtmlCrumb::E(e) => { e.brace_token1.span.clone() }
            XhtmlCrumb::F(f) => { f.gen_span() }
            XhtmlCrumb::L(l) => { l.span() }
            XhtmlCrumb::C(c) => { c.open.span.join(c.close.span).unwrap_or(c.open.span) }
        }
    }
    fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
        let mut cs = vec!();
        while input.peek(Ident) ||
              input.peek(LitBool) ||
              input.peek(LitChar) ||
              input.peek(LitInt) ||
              input.peek(LitStr) ||
              input.peek(Brace) ||
              input.peek(Bracket) ||
              input.peek(Token![abstract]) ||
              input.peek(Token![as]) ||
              input.peek(Token![become]) ||
              input.peek(Token![box]) ||
              input.peek(Token![break]) ||
              input.peek(Token![const]) ||
              input.peek(Token![continue]) ||
              input.peek(Token![crate]) ||
              input.peek(Token![do]) ||
              input.peek(Token![else]) ||
              input.peek(Token![enum]) ||
              input.peek(Token![extern]) ||
              input.peek(Token![final]) ||
              input.peek(Token![fn]) ||
              input.peek(Token![for]) ||
              input.peek(Token![if]) ||
              input.peek(Token![impl]) ||
              input.peek(Token![in]) ||
              input.peek(Token![let]) ||
              input.peek(Token![loop]) ||
              input.peek(Token![macro]) ||
              input.peek(Token![match]) ||
              input.peek(Token![mod]) ||
              input.peek(Token![move]) ||
              input.peek(Token![mut]) ||
              input.peek(Token![override]) ||
              input.peek(Token![priv]) ||
              input.peek(Token![pub]) ||
              input.peek(Token![ref]) ||
              input.peek(Token![return]) ||
              input.peek(Token![self]) ||
              input.peek(Token![Self]) ||
              input.peek(Token![static]) ||
              input.peek(Token![struct]) ||
              input.peek(Token![super]) ||
              input.peek(Token![trait]) ||
              input.peek(Token![type]) ||
              input.peek(Token![typeof]) ||
              input.peek(Token![unsafe]) ||
              input.peek(Token![unsized]) ||
              input.peek(Token![use]) ||
              input.peek(Token![virtual]) ||
              input.peek(Token![where]) ||
              input.peek(Token![while]) ||
              input.peek(Token![yield]) ||
              input.peek(Token![~]) ||
              input.peek(Token![!]) ||
              input.peek(Token![@]) ||
              input.peek(Token![#]) ||
              input.peek(Token![$]) ||
              input.peek(Token![%]) ||
              input.peek(Token![^]) ||
              input.peek(Token![&]) ||
              input.peek(Token![/]) ||
              input.peek(Token![*]) ||
              input.peek(Token![-]) ||
              input.peek(Token![+]) ||
              input.peek(Token![=]) ||
              input.peek(Token![|]) ||
              input.peek(Token![:]) ||
              input.peek(Token![;]) ||
              input.peek(Token![,]) ||
              input.peek(Token![.]) ||
              input.peek(Token![?]) ||
              (input.peek(Token![<]) && !input.peek2(Token![/])) {


           let c: XhtmlCrumb = input.parse()?;
           cs.push(c);
        }
        Ok(cs)
    }
}
impl Parse for XhtmlCrumb {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) && input.peek2(Token![!]) {
           let c: XhtmlClass = input.parse()?;
           Ok(XhtmlCrumb::C(c))
        } else if input.peek(LitBool) {
           let b: LitBool = input.parse()?;
           Ok(XhtmlCrumb::S(format!("{}",b.value), b.span()))
        } else if input.peek(LitInt) {
           let b: LitInt = input.parse()?;
           Ok(XhtmlCrumb::S(b.base10_digits().to_string(), b.span()))
        } else if input.peek(LitChar) {
           let b: LitChar = input.parse()?;
           Ok(XhtmlCrumb::S(format!("{}",b.value()), b.span()))
        } else if input.peek(LitStr) {
           let lit: LitStr = input.parse()?;
           Ok(XhtmlCrumb::L(lit))
        } else if input.peek(Token![<]) {
           let t: XhtmlTag = input.parse()?;
           Ok(XhtmlCrumb::T(t))
        } else if input.peek(Bracket) {
           let f: XhtmlExprF = XhtmlExprF::parse("markup".to_string(),input)?;
           Ok(XhtmlCrumb::F(f))
        } else if input.peek(Brace) {
           let e: XhtmlExpr = input.parse()?;
           Ok(XhtmlCrumb::E(e))
        } else if input.peek(Token![!]) {
           let id: Token![!] = input.parse()?;
           Ok(XhtmlCrumb::S("!".to_string(), id.span.clone()))
        } else if input.peek(Token![#]) {
           let id: Token![#] = input.parse()?;
           Ok(XhtmlCrumb::S("#".to_string(), id.span.clone()))
        } else if input.peek(Token![@]) {
           let id: Token![@] = input.parse()?;
           Ok(XhtmlCrumb::S("@".to_string(), id.span.clone()))
        } else if input.peek(Token![$]) {
           let id: Token![$] = input.parse()?;
           Ok(XhtmlCrumb::S("$".to_string(), id.span.clone()))
        } else if input.peek(Token![%]) {
           let id: Token![%] = input.parse()?;
           Ok(XhtmlCrumb::S("%".to_string(), id.span.clone()))
        } else if input.peek(Token![^]) {
           let id: Token![^] = input.parse()?;
           Ok(XhtmlCrumb::S("^".to_string(), id.span.clone()))
        } else if input.peek(Token![*]) {
           let id: Token![*] = input.parse()?;
           Ok(XhtmlCrumb::S("*".to_string(), id.span.clone()))
        } else if input.peek(Token![-]) {
           let id: Token![-] = input.parse()?;
           Ok(XhtmlCrumb::S("-".to_string(), id.span.clone()))
        } else if input.peek(Token![+]) {
           let id: Token![+] = input.parse()?;
           Ok(XhtmlCrumb::S("+".to_string(), id.span.clone()))
        } else if input.peek(Token![=]) {
           let id: Token![=] = input.parse()?;
           Ok(XhtmlCrumb::S("=".to_string(), id.span.clone()))
        } else if input.peek(Token![|]) {
           let id: Token![|] = input.parse()?;
           Ok(XhtmlCrumb::S("|".to_string(), id.span.clone()))
        } else if input.peek(Token![:]) {
           let id: Token![:] = input.parse()?;
           Ok(XhtmlCrumb::S(":".to_string(), id.span.clone()))
        } else if input.peek(Token![;]) {
           let id: Token![;] = input.parse()?;
           Ok(XhtmlCrumb::S(";".to_string(), id.span.clone()))
        } else if input.peek(Token![,]) {
           let id: Token![,] = input.parse()?;
           Ok(XhtmlCrumb::S(",".to_string(), id.span.clone()))
        } else if input.peek(Token![.]) {
           let id: Token![.] = input.parse()?;
           Ok(XhtmlCrumb::S(".".to_string(), id.span.clone()))
        } else if input.peek(Token![?]) {
           let id: Token![?] = input.parse()?;
           Ok(XhtmlCrumb::S("?".to_string(), id.span.clone()))
        } else if input.peek(Token![&]) {
           let id: Token![&] = input.parse()?;
           Ok(XhtmlCrumb::S("&".to_string(), id.span.clone()))
        } else if input.peek(Token![/]) {
           let id: Token![/] = input.parse()?;
           Ok(XhtmlCrumb::S("/".to_string(), id.span.clone()))
        } else if input.peek(Token![~]) {
           let id: Token![~] = input.parse()?;
           Ok(XhtmlCrumb::S("~".to_string(), id.span.clone()))
        } else if input.peek(Token![abstract]) {
           let id: Token![abstract] = input.parse()?;
           Ok(XhtmlCrumb::S("abstract".to_string(), id.span.clone()))
        } else if input.peek(Token![as]) {
           let id: Token![as] = input.parse()?;
           Ok(XhtmlCrumb::S("as".to_string(), id.span.clone()))
        } else if input.peek(Token![become]) {
           let id: Token![become] = input.parse()?;
           Ok(XhtmlCrumb::S("become".to_string(), id.span.clone()))
        } else if input.peek(Token![box]) {
           let id: Token![box] = input.parse()?;
           Ok(XhtmlCrumb::S("box".to_string(), id.span.clone()))
        } else if input.peek(Token![break]) {
           let id: Token![break] = input.parse()?;
           Ok(XhtmlCrumb::S("break".to_string(), id.span.clone()))
        } else if input.peek(Token![const]) {
           let id: Token![const] = input.parse()?;
           Ok(XhtmlCrumb::S("const".to_string(), id.span.clone()))
        } else if input.peek(Token![continue]) {
           let id: Token![continue] = input.parse()?;
           Ok(XhtmlCrumb::S("continue".to_string(), id.span.clone()))
        } else if input.peek(Token![crate]) {
           let id: Token![crate] = input.parse()?;
           Ok(XhtmlCrumb::S("crate".to_string(), id.span.clone()))
        } else if input.peek(Token![do]) {
           let id: Token![do] = input.parse()?;
           Ok(XhtmlCrumb::S("do".to_string(), id.span.clone()))
        } else if input.peek(Token![else]) {
           let id: Token![else] = input.parse()?;
           Ok(XhtmlCrumb::S("else".to_string(), id.span.clone()))
        } else if input.peek(Token![enum]) {
           let id: Token![enum] = input.parse()?;
           Ok(XhtmlCrumb::S("enum".to_string(), id.span.clone()))
        } else if input.peek(Token![extern]) {
           let id: Token![extern] = input.parse()?;
           Ok(XhtmlCrumb::S("extern".to_string(), id.span.clone()))
        } else if input.peek(Token![final]) {
           let id: Token![final] = input.parse()?;
           Ok(XhtmlCrumb::S("final".to_string(), id.span.clone()))
        } else if input.peek(Token![fn]) {
           let id: Token![fn] = input.parse()?;
           Ok(XhtmlCrumb::S("fn".to_string(), id.span.clone()))
        } else if input.peek(Token![for]) {
           let id: Token![for] = input.parse()?;
           Ok(XhtmlCrumb::S("for".to_string(), id.span.clone()))
        } else if input.peek(Token![if]) {
           let id: Token![if] = input.parse()?;
           Ok(XhtmlCrumb::S("if".to_string(), id.span.clone()))
        } else if input.peek(Token![impl]) {
           let id: Token![impl] = input.parse()?;
           Ok(XhtmlCrumb::S("impl".to_string(), id.span.clone()))
        } else if input.peek(Token![in]) {
           let id: Token![in] = input.parse()?;
           Ok(XhtmlCrumb::S("in".to_string(), id.span.clone()))
        } else if input.peek(Token![let]) {
           let id: Token![let] = input.parse()?;
           Ok(XhtmlCrumb::S("let".to_string(), id.span.clone()))
        } else if input.peek(Token![loop]) {
           let id: Token![loop] = input.parse()?;
           Ok(XhtmlCrumb::S("loop".to_string(), id.span.clone()))
        } else if input.peek(Token![macro]) {
           let id: Token![macro] = input.parse()?;
           Ok(XhtmlCrumb::S("macro".to_string(), id.span.clone()))
        } else if input.peek(Token![match]) {
           let id: Token![match] = input.parse()?;
           Ok(XhtmlCrumb::S("match".to_string(), id.span.clone()))
        } else if input.peek(Token![mod]) {
           let id: Token![mod] = input.parse()?;
           Ok(XhtmlCrumb::S("mod".to_string(), id.span.clone()))
        } else if input.peek(Token![move]) {
           let id: Token![move] = input.parse()?;
           Ok(XhtmlCrumb::S("move".to_string(), id.span.clone()))
        } else if input.peek(Token![mut]) {
           let id: Token![mut] = input.parse()?;
           Ok(XhtmlCrumb::S("mut".to_string(), id.span.clone()))
        } else if input.peek(Token![override]) {
           let id: Token![override] = input.parse()?;
           Ok(XhtmlCrumb::S("override".to_string(), id.span.clone()))
        } else if input.peek(Token![priv]) {
           let id: Token![priv] = input.parse()?;
           Ok(XhtmlCrumb::S("priv".to_string(), id.span.clone()))
        } else if input.peek(Token![pub]) {
           let id: Token![pub] = input.parse()?;
           Ok(XhtmlCrumb::S("pub".to_string(), id.span.clone()))
        } else if input.peek(Token![ref]) {
           let id: Token![ref] = input.parse()?;
           Ok(XhtmlCrumb::S("ref".to_string(), id.span.clone()))
        } else if input.peek(Token![return]) {
           let id: Token![return] = input.parse()?;
           Ok(XhtmlCrumb::S("return".to_string(), id.span.clone()))
        } else if input.peek(Token![self]) {
           let id: Token![self] = input.parse()?;
           Ok(XhtmlCrumb::S("self".to_string(), id.span.clone()))
        } else if input.peek(Token![Self]) {
           let id: Token![Self] = input.parse()?;
           Ok(XhtmlCrumb::S("Self".to_string(), id.span.clone()))
        } else if input.peek(Token![static]) {
           let id: Token![static] = input.parse()?;
           Ok(XhtmlCrumb::S("static".to_string(), id.span.clone()))
        } else if input.peek(Token![struct]) {
           let id: Token![struct] = input.parse()?;
           Ok(XhtmlCrumb::S("struct".to_string(), id.span.clone()))
        } else if input.peek(Token![super]) {
           let id: Token![super] = input.parse()?;
           Ok(XhtmlCrumb::S("super".to_string(), id.span.clone()))
        } else if input.peek(Token![trait]) {
           let id: Token![trait] = input.parse()?;
           Ok(XhtmlCrumb::S("trait".to_string(), id.span.clone()))
        } else if input.peek(Token![type]) {
           let id: Token![type] = input.parse()?;
           Ok(XhtmlCrumb::S("type".to_string(), id.span.clone()))
        } else if input.peek(Token![typeof]) {
           let id: Token![typeof] = input.parse()?;
           Ok(XhtmlCrumb::S("typeof".to_string(), id.span.clone()))
        } else if input.peek(Token![unsafe]) {
           let id: Token![unsafe] = input.parse()?;
           Ok(XhtmlCrumb::S("unsafe".to_string(), id.span.clone()))
        } else if input.peek(Token![unsized]) {
           let id: Token![unsized] = input.parse()?;
           Ok(XhtmlCrumb::S("unsized".to_string(), id.span.clone()))
        } else if input.peek(Token![use]) {
           let id: Token![use] = input.parse()?;
           Ok(XhtmlCrumb::S("use".to_string(), id.span.clone()))
        } else if input.peek(Token![virtual]) {
           let id: Token![virtual] = input.parse()?;
           Ok(XhtmlCrumb::S("virtual".to_string(), id.span.clone()))
        } else if input.peek(Token![where]) {
           let id: Token![where] = input.parse()?;
           Ok(XhtmlCrumb::S("where".to_string(), id.span.clone()))
        } else if input.peek(Token![while]) {
           let id: Token![while] = input.parse()?;
           Ok(XhtmlCrumb::S("while".to_string(), id.span.clone()))
        } else if input.peek(Token![yield]) {
           let id: Token![yield] = input.parse()?;
           Ok(XhtmlCrumb::S("yield".to_string(), id.span.clone()))
        } else {
           let id: Ident = input.parse()?;
           Ok(XhtmlCrumb::S(id.to_string(), id.span().clone()))
        }
    }
}
impl ToTokens for XhtmlCrumb {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           XhtmlCrumb::S(s,span) => {
              let l = Literal::string(&s);
              (quote_spanned!{span.clone()=>
                 stream.push_str(#l);
              }).to_tokens(tokens);
           },
           XhtmlCrumb::L(l) => {
              let span = l.span().clone();
              (quote_spanned!{span=>
                 stream.push_str(#l);
              }).to_tokens(tokens);
           },
           XhtmlCrumb::T(t) => {
              t.to_tokens(tokens);
           }
           XhtmlCrumb::E(e) => {
              e.to_tokens(tokens);
           }
           XhtmlCrumb::F(e) => {
              e.to_tokens(tokens);
           }
           XhtmlCrumb::C(c) => {
              let span = c.gen_span();
              (quote_spanned!{span=>
                 stream.push_str(&#c.to_string());
              }).to_tokens(tokens);
           }
        }
    }
}

pub struct Xhtml {
    crumbs: Vec<XhtmlCrumb>
}
impl Xhtml {
    fn gen_span(&self) -> Span {
       if self.crumbs.len() > 0 {
          let mut span = self.crumbs[0].span();
          for c in self.crumbs[1..].iter() {
             span = span.join(c.span()).unwrap_or(c.span());
          }
          span
       } else {
          Span::call_site()
       }
    }
}
impl ToTokens for Xhtml {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut prev: Option<Span> = None;
        for c in self.crumbs.iter() {
            let span = c.span();
            if let Some(sp) = prev {
            if c.does_emit() && span.start() > sp.end() {
               (quote_spanned!{span.clone()=>
                  stream.push_str(" ");
               }).to_tokens(tokens);
            }}

            prev = Some(span.clone());
            c.to_tokens(tokens);
        }
    }
}

impl Parse for Xhtml {
    fn parse(input: ParseStream) -> Result<Self> {
        let crumbs: Vec<XhtmlCrumb> = input.call(XhtmlCrumb::parse_outer)?;

        Ok(Xhtml {
            crumbs: crumbs
        })
    }
}
