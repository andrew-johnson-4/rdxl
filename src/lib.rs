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
//! rdxl!({{ x }}, {{ y }})
//! ```
//!
//! # Conditional expressions
//!
//! ```no_run
//! let x = 5;
//! let y = "asdf";
//! rdxl!({{ if x>3 {{
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
//! rdxl!(<ul>{{ for i in 0..10 {{
//!   <li>{{ i }}</li>
//! }} }}</ul>)
//! ```
//!
//! # Miscellaneous expressions
//!
//! ```no_run
//! rdxl!(
//!   {{ let x = 5; }}
//!   {{ x }}
//! )
//! ```
//!
//! # Html attributes
//! ```no_run
//! rdxl!(<div style={{ "\"color:red;\"" }}>
//!   inside div
//! </div>)
//! ```

#![recursion_limit = "128"]
#![feature(type_ascription)]
#![crate_type = "proc-macro"]

use quote::{quote, TokenStreamExt, ToTokens};
use proc_macro::{TokenStream};
use proc_macro2::{Spacing, Span, Punct, Literal, Ident, Group, Delimiter};
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{parse_macro_input, Ident as SynIdent, Token, Expr, Pat, LitStr, bracketed, braced};
use syn::token::{Bracket,Brace};

struct RdxlExprF {
   context: String,
   expr: Expr
}
impl ToTokens for RdxlExprF {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       let ss = Span::call_site();

       tokens.append(Ident::new("stream", ss.clone()));
       tokens.append(Punct::new('.', Spacing::Alone));
       tokens.append(Ident::new("push_str", ss.clone()));

       let mut ts = proc_macro2::TokenStream::new();

       ts.append(Punct::new('&', Spacing::Alone));
       self.expr.to_tokens(&mut ts);
       ts.append(Punct::new('.', Spacing::Alone));
       ts.append(Ident::new(&format!("to_{}", self.context), ss.clone()));
       let ets = proc_macro2::TokenStream::new();
       let egr = Group::new(Delimiter::Parenthesis, ets);
       ts.append(egr);

       let gr = Group::new(Delimiter::Parenthesis, ts);
       tokens.append(gr);
       tokens.append(Punct::new(';', Spacing::Alone));
    }
}
impl RdxlExprF {
    fn parse(context: String, input: ParseStream) -> Result<Self> {
       let content;
       let content2;
       let _bracket1 = bracketed!(content in input);
       let _bracket2 = bracketed!(content2 in content);
       let expr: Expr = content2.parse()?;
       Ok(RdxlExprF{ context:context, expr:expr })
    }
}

enum RdxlExprE {
   S(Expr),
   E(Expr),
   F(Token![for],Pat,Expr,Vec<RdxlCrumb>),
   W(Token![while],Expr,Vec<RdxlCrumb>),
   L(Token![let],Pat,Expr),
   I(Token![if],Expr,Vec<RdxlCrumb>,Vec<(Expr,Vec<RdxlCrumb>)>,Vec<RdxlCrumb>)
}
impl ToTokens for RdxlExprE {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           RdxlExprE::E(e) => {
              let ss = Span::call_site();

              tokens.append(Ident::new("stream", ss.clone()));
              tokens.append(Punct::new('.', Spacing::Alone));
              tokens.append(Ident::new("push_str", ss.clone()));

              let mut ts = proc_macro2::TokenStream::new();

              ts.append(Punct::new('&', Spacing::Alone));
              e.to_tokens(&mut ts);
              ts.append(Punct::new('.', Spacing::Alone));
              ts.append(Ident::new("to_string", ss.clone()));
              let ets = proc_macro2::TokenStream::new();
              let egr = Group::new(Delimiter::Parenthesis, ets);
              ts.append(egr);

              let gr = Group::new(Delimiter::Parenthesis, ts);
              tokens.append(gr);
              tokens.append(Punct::new(';', Spacing::Alone));
           }, RdxlExprE::S(e) => {
              e.to_tokens(tokens);
              tokens.append(Punct::new(';', Spacing::Alone));
           }, RdxlExprE::F(f,p,i,cs) => {
              tokens.append(Ident::new("for", f.span.clone()));
              p.to_tokens(tokens);
              tokens.append(Ident::new("in", f.span.clone())); 
              i.to_tokens(tokens);

              let mut ets = proc_macro2::TokenStream::new();
              for c in cs.iter() {
                 c.to_tokens(&mut ets);
              }
              let egr = Group::new(Delimiter::Brace, ets);
              tokens.append(egr);
           }, RdxlExprE::I(i,c,bs,es,e) => {
              tokens.append(Ident::new("if", i.span.clone()));
              c.to_tokens(tokens);

              let mut ets = proc_macro2::TokenStream::new();
              for b in bs.iter() {
                 b.to_tokens(&mut ets);
              }
              let egr = Group::new(Delimiter::Brace, ets);
              tokens.append(egr);

              for (c,e) in es.iter() {
                 tokens.append(Ident::new("else", i.span.clone()));
                 tokens.append(Ident::new("if", i.span.clone()));
                 c.to_tokens(tokens);
                 let mut ets = proc_macro2::TokenStream::new();
                 for b in e.iter() {
                    b.to_tokens(&mut ets);
                 }
                 let egr = Group::new(Delimiter::Brace, ets);
                 tokens.append(egr);
              }

              if e.len() > 0 {
                 tokens.append(Ident::new("else", i.span.clone()));
                 let mut ets = proc_macro2::TokenStream::new();
                 for b in e.iter() {
                    b.to_tokens(&mut ets);
                 }
                 let egr = Group::new(Delimiter::Brace, ets);
                 tokens.append(egr);
              }
           }, RdxlExprE::W(w,i,cs) => {
              tokens.append(Ident::new("while", w.span.clone()));
              i.to_tokens(tokens);

              let mut ets = proc_macro2::TokenStream::new();
              for c in cs.iter() {
                 c.to_tokens(&mut ets);
              }
              let egr = Group::new(Delimiter::Brace, ets);
              tokens.append(egr);
           }, RdxlExprE::L(t,l,e) => {
              tokens.append(Ident::new("let", t.span.clone()));
              l.to_tokens(tokens);
              tokens.append(Punct::new('=', Spacing::Alone));
              e.to_tokens(tokens);
              tokens.append(Punct::new(';', Spacing::Alone));
           }
        }
    }
}
impl Parse for RdxlExprE {
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
          let body: Vec<RdxlCrumb> = content2.call(RdxlCrumb::parse_outer)?;
          Ok(RdxlExprE::F(_for,pat,iter,body))
       } else if input.peek(Token![while]) {
          let _while: Token![while] = input.parse()?;
          let iter: Expr = input.parse()?;
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<RdxlCrumb> = content2.call(RdxlCrumb::parse_outer)?;
          Ok(RdxlExprE::W(_while,iter,body))
       } else if input.peek(Token![if]) {
          let _if: Token![if] = input.parse()?;
          let b: Expr = input.parse()?;
          let mut es = Vec::new();
          let mut e = Vec::new();
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<RdxlCrumb> = content2.call(RdxlCrumb::parse_outer)?;

          while input.peek(Token![else]) && input.peek2(Token![if]) {
             let _else: Token![else] = input.parse()?;
             let _if: Token![if] = input.parse()?;
             let b: Expr = input.parse()?;
             let content;
             let content2;
             let _brace1 = braced!(content in input);
             let _brace2 = braced!(content2 in content);
             let e = content2.call(RdxlCrumb::parse_outer)?;
             es.push((b,e));
          }

          if input.peek(Token![else]) {
             let _else: Token![else] = input.parse()?;
             let content;
             let content2;
             let _brace1 = braced!(content in input);
             let _brace2 = braced!(content2 in content);
             e = content2.call(RdxlCrumb::parse_outer)?;
          }

          Ok(RdxlExprE::I(_if,b,body,es,e))
       } else if input.peek(Token![let]) {
          let _let: Token![let] = input.parse()?;
          let pat: Pat = input.parse()?;
          let _eq: Token![=] = input.parse()?;
          let expr: Expr = input.parse()?;
          Ok(RdxlExprE::L(_let,pat,expr))
       } else if input.peek(Token![;]) {
          let _semi: Token![;] = input.parse()?;
          Ok(RdxlExprE::S(input.call(Expr::parse)?))
       } else {
          Ok(RdxlExprE::E(input.call(Expr::parse)?))
       }
    }
}

struct RdxlExpr {
   brace_token1: Brace,
   _brace_token2: Brace,
   expr: RdxlExprE
}
impl Parse for RdxlExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let _content;
        let content2;
        Ok(RdxlExpr {
           brace_token1: braced!(_content in input),
           _brace_token2: braced!(content2 in _content),
           expr: content2.call(RdxlExprE::parse)?,
        })
    }
}
impl ToTokens for RdxlExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.expr.to_tokens(tokens)
    }
}

enum RdxlAttr {
   S(String),
   F(RdxlExprF),
   E(RdxlExpr)
}

struct RdxlTag {
   tag: String,
   attrs: Vec<(String,RdxlAttr)>,
   inner: Rdxl,
   span: Span
}
impl ToTokens for RdxlTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(Ident::new("stream", self.span.clone()));
        tokens.append(Punct::new('.', Spacing::Alone));
        tokens.append(Ident::new("push_str", self.span.clone()));
        let mut ts = proc_macro2::TokenStream::new();
        ts.append(Literal::string(&format!("<{}", self.tag)));
        let gr = Group::new(Delimiter::Parenthesis, ts);
        tokens.append(gr);
        tokens.append(Punct::new(';', Spacing::Alone));

        for (k,v) in self.attrs.iter() {
            tokens.append(Ident::new("stream", self.span.clone()));
            tokens.append(Punct::new('.', Spacing::Alone));
            tokens.append(Ident::new("push_str", self.span.clone()));
            let mut ts = proc_macro2::TokenStream::new();

            match v {
               RdxlAttr::S(s) => {
                  ts.append(Literal::string(&format!(" {}={}", k, s)));
                  let gr = Group::new(Delimiter::Parenthesis, ts);
                  tokens.append(gr);
                  tokens.append(Punct::new(';', Spacing::Alone));
               }, RdxlAttr::F(f) => {
                  ts.append(Literal::string(&format!(" {}=", k)));
                  let gr = Group::new(Delimiter::Parenthesis, ts);
                  tokens.append(gr);
                  tokens.append(Punct::new(';', Spacing::Alone));
                  f.to_tokens(tokens);
               }, RdxlAttr::E(e) => {
                  ts.append(Literal::string(&format!(" {}=", k)));
                  let gr = Group::new(Delimiter::Parenthesis, ts);
                  tokens.append(gr);
                  tokens.append(Punct::new(';', Spacing::Alone));
                  e.to_tokens(tokens);
               }
            }
        }

        tokens.append(Ident::new("stream", self.span.clone()));
        tokens.append(Punct::new('.', Spacing::Alone));
        tokens.append(Ident::new("push_str", self.span.clone()));
        let mut ts = proc_macro2::TokenStream::new();
        ts.append(Literal::string(">"));
        let gr = Group::new(Delimiter::Parenthesis, ts);
        tokens.append(gr);
        tokens.append(Punct::new(';', Spacing::Alone));

        self.inner.to_tokens(tokens);

        tokens.append(Ident::new("stream", self.span.clone()));
        tokens.append(Punct::new('.', Spacing::Alone));
        tokens.append(Ident::new("push_str", self.span.clone()));
        let mut ts = proc_macro2::TokenStream::new();
        ts.append(Literal::string(&format!("</{}>", self.tag)));
        let gr = Group::new(Delimiter::Parenthesis, ts);
        tokens.append(gr);
        tokens.append(Punct::new(';', Spacing::Alone));
    }
}
impl Parse for RdxlTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let l1: Token![<] = input.parse()?;
        let t: Ident = input.parse()?;

        let mut attrs: Vec<(String,RdxlAttr)> = Vec::new();
        while input.peek(SynIdent) ||
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
              input.peek(Token![while]) {
            let key = if input.peek(Token![as]) { let _:Token![as] = input.parse()?; "as".to_string()
            } else if input.peek(Token![break]) { let _:Token![break] = input.parse()?; "break".to_string()
            } else if input.peek(Token![const]) { let _:Token![const] = input.parse()?; "const".to_string()
            } else if input.peek(Token![continue]) { let _:Token![continue] = input.parse()?; "continue".to_string()
            } else if input.peek(Token![crate]) { let _:Token![crate] = input.parse()?; "crate".to_string()
            } else if input.peek(Token![else]) { let _:Token![else] = input.parse()?; "else".to_string()
            } else if input.peek(Token![enum]) { let _:Token![enum] = input.parse()?; "enum".to_string()
            } else if input.peek(Token![extern]) { let _:Token![extern] = input.parse()?; "extern".to_string()
            } else if input.peek(Token![fn]) { let _:Token![fn] = input.parse()?; "fn".to_string()
            } else if input.peek(Token![for]) { let _:Token![for] = input.parse()?; "for".to_string()
            } else if input.peek(Token![if]) { let _:Token![if] = input.parse()?; "if".to_string()
            } else if input.peek(Token![impl]) { let _:Token![impl] = input.parse()?; "impl".to_string()
            } else if input.peek(Token![in]) { let _:Token![in] = input.parse()?; "in".to_string()
            } else if input.peek(Token![let]) { let _:Token![let] = input.parse()?; "let".to_string()
            } else if input.peek(Token![loop]) { let _:Token![loop] = input.parse()?; "loop".to_string()
            } else if input.peek(Token![match]) { let _:Token![match] = input.parse()?; "match".to_string()
            } else if input.peek(Token![mod]) { let _:Token![mod] = input.parse()?; "mod".to_string()
            } else if input.peek(Token![move]) { let _:Token![move] = input.parse()?; "move".to_string()
            } else if input.peek(Token![mut]) { let _:Token![mut] = input.parse()?; "mut".to_string()
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
            } else if input.peek(Token![unsafe]) { let _:Token![unsafe] = input.parse()?; "unsafe".to_string()
            } else if input.peek(Token![use]) { let _:Token![use] = input.parse()?; "use".to_string()
            } else if input.peek(Token![where]) { let _:Token![where] = input.parse()?; "where".to_string()
            } else if input.peek(Token![while]) { let _:Token![while] = input.parse()?; "while".to_string()
            } else { let key: Ident = input.parse()?; key.to_string() };
            let _eq: Token![=] = input.parse()?;
            if input.peek(Bracket) {
               let f: RdxlExprF = RdxlExprF::parse(key.clone(),input)?;
               attrs.push(( key.clone(), RdxlAttr::F(f) ));
            } else if input.peek(Brace) {
               let e: RdxlExpr = input.parse()?;
               attrs.push(( key, RdxlAttr::E(e) ));
            } else {
               let val: Literal = input.parse()?;
               attrs.push(( key, RdxlAttr::S(val.to_string()) ));
            }
        }

        let _l2: Token![>] = input.parse()?;

        if t.to_string() == "br" {
           Ok(RdxlTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: Rdxl { crumbs: vec!() },
              span: l1.span.clone()
           })
        } else {
           let inner: Rdxl = input.parse()?;

           let _r1: Token![<] = input.parse()?;
           let _r2: Token![/] = input.parse()?;
           let t2: Ident = input.parse()?;
           if t.to_string() != t2.to_string() {
              let msg = format!("Expected </{}> found </{}>", t, t2);
              let r = Error::new(t2.span(), msg);
              return Err(r)
           }
           let _r3: Token![>] = input.parse()?;
        
           Ok(RdxlTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: inner,
              span: l1.span.clone()
           })
       }
    }
}

enum RdxlCrumb {
   L(LitStr),
   S(String, Span),
   T(RdxlTag),
   E(RdxlExpr),
   F(RdxlExprF),
}
impl RdxlCrumb {
    fn span(&self) -> Span {
        match self {
            RdxlCrumb::S(_,sp) => { sp.clone() }
            RdxlCrumb::T(t) => { t.span.clone() }
            RdxlCrumb::E(e) => { e.brace_token1.span.clone() }
            RdxlCrumb::F(_) => { Span::call_site() }
            RdxlCrumb::L(l) => { l.span() }
        }
    }
    fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
        let mut cs = vec!();
        while input.peek(SynIdent) ||
              input.peek(LitStr) ||
              input.peek(Brace) ||
              input.peek(Bracket) ||
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


           let c: RdxlCrumb = input.parse()?;
           cs.push(c);
        }
        Ok(cs)
    }
}
impl Parse for RdxlCrumb {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) {
           let t: RdxlTag = input.parse()?;
           Ok(RdxlCrumb::T(t))
        } else if input.peek(Bracket) {
           let f: RdxlExprF = RdxlExprF::parse("markup".to_string(),input)?;
           Ok(RdxlCrumb::F(f))
        } else if input.peek(Brace) {
           let e: RdxlExpr = input.parse()?;
           Ok(RdxlCrumb::E(e))
        } else if input.peek(LitStr) {
           let lit: LitStr = input.parse()?;
           Ok(RdxlCrumb::L(lit))
        } else if input.peek(Token![!]) {
           let id: Token![!] = input.parse()?;
           Ok(RdxlCrumb::S("!".to_string(), id.span.clone()))
        } else if input.peek(Token![#]) {
           let id: Token![#] = input.parse()?;
           Ok(RdxlCrumb::S("#".to_string(), id.span.clone()))
        } else if input.peek(Token![@]) {
           let id: Token![@] = input.parse()?;
           Ok(RdxlCrumb::S("@".to_string(), id.span.clone()))
        } else if input.peek(Token![$]) {
           let id: Token![$] = input.parse()?;
           Ok(RdxlCrumb::S("$".to_string(), id.span.clone()))
        } else if input.peek(Token![%]) {
           let id: Token![%] = input.parse()?;
           Ok(RdxlCrumb::S("%".to_string(), id.span.clone()))
        } else if input.peek(Token![^]) {
           let id: Token![^] = input.parse()?;
           Ok(RdxlCrumb::S("^".to_string(), id.span.clone()))
        } else if input.peek(Token![*]) {
           let id: Token![*] = input.parse()?;
           Ok(RdxlCrumb::S("*".to_string(), id.span.clone()))
        } else if input.peek(Token![-]) {
           let id: Token![-] = input.parse()?;
           Ok(RdxlCrumb::S("-".to_string(), id.span.clone()))
        } else if input.peek(Token![+]) {
           let id: Token![+] = input.parse()?;
           Ok(RdxlCrumb::S("+".to_string(), id.span.clone()))
        } else if input.peek(Token![=]) {
           let id: Token![=] = input.parse()?;
           Ok(RdxlCrumb::S("=".to_string(), id.span.clone()))
        } else if input.peek(Token![|]) {
           let id: Token![|] = input.parse()?;
           Ok(RdxlCrumb::S("|".to_string(), id.span.clone()))
        } else if input.peek(Token![:]) {
           let id: Token![:] = input.parse()?;
           Ok(RdxlCrumb::S(":".to_string(), id.span.clone()))
        } else if input.peek(Token![;]) {
           let id: Token![;] = input.parse()?;
           Ok(RdxlCrumb::S(";".to_string(), id.span.clone()))
        } else if input.peek(Token![,]) {
           let id: Token![,] = input.parse()?;
           Ok(RdxlCrumb::S(",".to_string(), id.span.clone()))
        } else if input.peek(Token![.]) {
           let id: Token![.] = input.parse()?;
           Ok(RdxlCrumb::S(".".to_string(), id.span.clone()))
        } else if input.peek(Token![?]) {
           let id: Token![?] = input.parse()?;
           Ok(RdxlCrumb::S("?".to_string(), id.span.clone()))
        } else if input.peek(Token![&]) {
           let id: Token![&] = input.parse()?;
           Ok(RdxlCrumb::S("&".to_string(), id.span.clone()))
        } else if input.peek(Token![/]) {
           let id: Token![/] = input.parse()?;
           Ok(RdxlCrumb::S("/".to_string(), id.span.clone()))
        } else if input.peek(Token![~]) {
           let id: Token![~] = input.parse()?;
           Ok(RdxlCrumb::S("~".to_string(), id.span.clone()))
        } else if input.peek(Token![as]) {
           let id: Token![as] = input.parse()?;
           Ok(RdxlCrumb::S("as".to_string(), id.span.clone()))
        } else if input.peek(Token![break]) {
           let id: Token![break] = input.parse()?;
           Ok(RdxlCrumb::S("break".to_string(), id.span.clone()))
        } else if input.peek(Token![const]) {
           let id: Token![const] = input.parse()?;
           Ok(RdxlCrumb::S("const".to_string(), id.span.clone()))
        } else if input.peek(Token![continue]) {
           let id: Token![continue] = input.parse()?;
           Ok(RdxlCrumb::S("continue".to_string(), id.span.clone()))
        } else if input.peek(Token![crate]) {
           let id: Token![crate] = input.parse()?;
           Ok(RdxlCrumb::S("crate".to_string(), id.span.clone()))
        } else if input.peek(Token![else]) {
           let id: Token![else] = input.parse()?;
           Ok(RdxlCrumb::S("else".to_string(), id.span.clone()))
        } else if input.peek(Token![enum]) {
           let id: Token![enum] = input.parse()?;
           Ok(RdxlCrumb::S("enum".to_string(), id.span.clone()))
        } else if input.peek(Token![extern]) {
           let id: Token![extern] = input.parse()?;
           Ok(RdxlCrumb::S("extern".to_string(), id.span.clone()))
        } else if input.peek(Token![fn]) {
           let id: Token![fn] = input.parse()?;
           Ok(RdxlCrumb::S("fn".to_string(), id.span.clone()))
        } else if input.peek(Token![for]) {
           let id: Token![for] = input.parse()?;
           Ok(RdxlCrumb::S("for".to_string(), id.span.clone()))
        } else if input.peek(Token![if]) {
           let id: Token![if] = input.parse()?;
           Ok(RdxlCrumb::S("if".to_string(), id.span.clone()))
        } else if input.peek(Token![impl]) {
           let id: Token![impl] = input.parse()?;
           Ok(RdxlCrumb::S("impl".to_string(), id.span.clone()))
        } else if input.peek(Token![in]) {
           let id: Token![in] = input.parse()?;
           Ok(RdxlCrumb::S("in".to_string(), id.span.clone()))
        } else if input.peek(Token![let]) {
           let id: Token![let] = input.parse()?;
           Ok(RdxlCrumb::S("let".to_string(), id.span.clone()))
        } else if input.peek(Token![loop]) {
           let id: Token![loop] = input.parse()?;
           Ok(RdxlCrumb::S("loop".to_string(), id.span.clone()))
        } else if input.peek(Token![match]) {
           let id: Token![match] = input.parse()?;
           Ok(RdxlCrumb::S("match".to_string(), id.span.clone()))
        } else if input.peek(Token![mod]) {
           let id: Token![mod] = input.parse()?;
           Ok(RdxlCrumb::S("mod".to_string(), id.span.clone()))
        } else if input.peek(Token![move]) {
           let id: Token![move] = input.parse()?;
           Ok(RdxlCrumb::S("move".to_string(), id.span.clone()))
        } else if input.peek(Token![mut]) {
           let id: Token![mut] = input.parse()?;
           Ok(RdxlCrumb::S("mut".to_string(), id.span.clone()))
        } else if input.peek(Token![pub]) {
           let id: Token![pub] = input.parse()?;
           Ok(RdxlCrumb::S("pub".to_string(), id.span.clone()))
        } else if input.peek(Token![ref]) {
           let id: Token![ref] = input.parse()?;
           Ok(RdxlCrumb::S("ref".to_string(), id.span.clone()))
        } else if input.peek(Token![return]) {
           let id: Token![return] = input.parse()?;
           Ok(RdxlCrumb::S("return".to_string(), id.span.clone()))
        } else if input.peek(Token![self]) {
           let id: Token![self] = input.parse()?;
           Ok(RdxlCrumb::S("self".to_string(), id.span.clone()))
        } else if input.peek(Token![Self]) {
           let id: Token![Self] = input.parse()?;
           Ok(RdxlCrumb::S("Self".to_string(), id.span.clone()))
        } else if input.peek(Token![static]) {
           let id: Token![static] = input.parse()?;
           Ok(RdxlCrumb::S("static".to_string(), id.span.clone()))
        } else if input.peek(Token![struct]) {
           let id: Token![struct] = input.parse()?;
           Ok(RdxlCrumb::S("struct".to_string(), id.span.clone()))
        } else if input.peek(Token![super]) {
           let id: Token![super] = input.parse()?;
           Ok(RdxlCrumb::S("super".to_string(), id.span.clone()))
        } else if input.peek(Token![trait]) {
           let id: Token![trait] = input.parse()?;
           Ok(RdxlCrumb::S("trait".to_string(), id.span.clone()))
        } else if input.peek(Token![type]) {
           let id: Token![type] = input.parse()?;
           Ok(RdxlCrumb::S("type".to_string(), id.span.clone()))
        } else if input.peek(Token![unsafe]) {
           let id: Token![unsafe] = input.parse()?;
           Ok(RdxlCrumb::S("unsafe".to_string(), id.span.clone()))
        } else if input.peek(Token![use]) {
           let id: Token![use] = input.parse()?;
           Ok(RdxlCrumb::S("use".to_string(), id.span.clone()))
        } else if input.peek(Token![where]) {
           let id: Token![where] = input.parse()?;
           Ok(RdxlCrumb::S("where".to_string(), id.span.clone()))
        } else if input.peek(Token![while]) {
           let id: Token![while] = input.parse()?;
           Ok(RdxlCrumb::S("while".to_string(), id.span.clone()))
        } else {
           let id: Ident = input.parse()?;
           Ok(RdxlCrumb::S(id.to_string(), id.span().clone()))
        }
    }
}
impl ToTokens for RdxlCrumb {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           RdxlCrumb::S(s,ss) => {
              tokens.append(Ident::new("stream", ss.clone()));
              tokens.append(Punct::new('.', Spacing::Alone));
              tokens.append(Ident::new("push_str", ss.clone()));

              let mut ts = proc_macro2::TokenStream::new();
              ts.append(Literal::string(&s));
              let gr = Group::new(Delimiter::Parenthesis, ts);
              tokens.append(gr);

              tokens.append(Punct::new(';', Spacing::Alone));
           },
           RdxlCrumb::L(l) => {
              let ss = l.span().clone();
              tokens.append(Ident::new("stream", ss.clone()));
              tokens.append(Punct::new('.', Spacing::Alone));
              tokens.append(Ident::new("push_str", ss.clone()));

              let mut ts = proc_macro2::TokenStream::new();
              l.to_tokens(&mut ts);
              let gr = Group::new(Delimiter::Parenthesis, ts);
              tokens.append(gr);

              tokens.append(Punct::new(';', Spacing::Alone));
           },
           RdxlCrumb::T(t) => {
              t.to_tokens(tokens);
           }
           RdxlCrumb::E(e) => {
              e.to_tokens(tokens);
           }
           RdxlCrumb::F(e) => {
              e.to_tokens(tokens);
           }
        }
    }
}

struct Rdxl {
    crumbs: Vec<RdxlCrumb>
}
impl ToTokens for Rdxl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut prev: Option<Span> = None;
        for c in self.crumbs.iter() {
            let ss = c.span();
            if let Some(sp) = prev {
            if sp.end() != ss.start() {

              tokens.append(Ident::new("stream", ss.clone()));
              tokens.append(Punct::new('.', Spacing::Alone));
              tokens.append(Ident::new("push_str", ss.clone()));

              let mut ts = proc_macro2::TokenStream::new();
              ts.append(Literal::string(" "));
              let gr = Group::new(Delimiter::Parenthesis, ts);
              tokens.append(gr);

              tokens.append(Punct::new(';', Spacing::Alone));
                
            }}
            prev = Some(ss.clone());
            c.to_tokens(tokens);
        }
    }
}

impl Parse for Rdxl {
    fn parse(input: ParseStream) -> Result<Self> {
        let crumbs: Vec<RdxlCrumb> = input.call(RdxlCrumb::parse_outer)?;

        Ok(Rdxl {
            crumbs: crumbs
        })
    }
}

#[proc_macro]
pub fn rdxl(input: TokenStream) -> TokenStream {
    let rdxls = parse_macro_input!(input as Rdxl);

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

