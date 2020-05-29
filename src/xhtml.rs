// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{TokenStreamExt, ToTokens};
use proc_macro2::{Spacing, Span, Punct, Literal, Ident, Group, Delimiter};
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{Ident as SynIdent, Token, Expr, Pat, LitStr, bracketed, braced};
use syn::token::{Bracket,Brace};

pub struct XhtmlExprF {
   context: String,
   expr: Expr
}
impl ToTokens for XhtmlExprF {
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
impl XhtmlExprF {
    fn parse(context: String, input: ParseStream) -> Result<Self> {
       let content;
       let content2;
       let _bracket1 = bracketed!(content in input);
       let _bracket2 = bracketed!(content2 in content);
       let expr: Expr = content2.parse()?;
       Ok(XhtmlExprF{ context:context, expr:expr })
    }
}

enum XhtmlExprE {
   S(Expr),
   E(Expr),
   F(Token![for],Pat,Expr,Vec<XhtmlCrumb>),
   W(Token![while],Expr,Vec<XhtmlCrumb>),
   L(Token![let],Pat,Expr),
   I(Token![if],Expr,Vec<XhtmlCrumb>,Vec<(Expr,Vec<XhtmlCrumb>)>,Vec<XhtmlCrumb>)
}
impl ToTokens for XhtmlExprE {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           XhtmlExprE::E(e) => {
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
           }, XhtmlExprE::S(e) => {
              e.to_tokens(tokens);
              tokens.append(Punct::new(';', Spacing::Alone));
           }, XhtmlExprE::F(f,p,i,cs) => {
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
           }, XhtmlExprE::I(i,c,bs,es,e) => {
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
           }, XhtmlExprE::W(w,i,cs) => {
              tokens.append(Ident::new("while", w.span.clone()));
              i.to_tokens(tokens);

              let mut ets = proc_macro2::TokenStream::new();
              for c in cs.iter() {
                 c.to_tokens(&mut ets);
              }
              let egr = Group::new(Delimiter::Brace, ets);
              tokens.append(egr);
           }, XhtmlExprE::L(t,l,e) => {
              tokens.append(Ident::new("let", t.span.clone()));
              l.to_tokens(tokens);
              tokens.append(Punct::new('=', Spacing::Alone));
              e.to_tokens(tokens);
              tokens.append(Punct::new(';', Spacing::Alone));
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

pub enum XhtmlAttr {
   S(String),
   F(XhtmlExprF),
   E(XhtmlExpr)
}

pub struct XhtmlTag {
   tag: String,
   attrs: Vec<(String,XhtmlAttr)>,
   inner: Xhtml,
   span: Span
}
impl ToTokens for XhtmlTag {
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
               XhtmlAttr::S(s) => {
                  ts.append(Literal::string(&format!(" {}={}", k, s)));
                  let gr = Group::new(Delimiter::Parenthesis, ts);
                  tokens.append(gr);
                  tokens.append(Punct::new(';', Spacing::Alone));
               }, XhtmlAttr::F(f) => {
                  ts.append(Literal::string(&format!(" {}=", k)));
                  let gr = Group::new(Delimiter::Parenthesis, ts);
                  tokens.append(gr);
                  tokens.append(Punct::new(';', Spacing::Alone));
                  f.to_tokens(tokens);
               }, XhtmlAttr::E(e) => {
                  ts.append(Literal::string(&format!(" {}=", k)));
                  let gr = Group::new(Delimiter::Parenthesis, ts);
                  tokens.append(gr);
                  tokens.append(Punct::new(';', Spacing::Alone));
                  e.to_tokens(tokens);
               }
            }
        }

        if self.inner.crumbs.len()==0 {
           tokens.append(Ident::new("stream", self.span.clone()));
           tokens.append(Punct::new('.', Spacing::Alone));
           tokens.append(Ident::new("push_str", self.span.clone()));
           let mut ts = proc_macro2::TokenStream::new();
           ts.append(Literal::string("/>"));
           let gr = Group::new(Delimiter::Parenthesis, ts);
           tokens.append(gr);
           tokens.append(Punct::new(';', Spacing::Alone));
        } else {
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
}
impl Parse for XhtmlTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let l1: Token![<] = input.parse()?;
        let t: Ident = input.parse()?;

        let mut attrs: Vec<(String,XhtmlAttr)> = Vec::new();
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
               let f: XhtmlExprF = XhtmlExprF::parse(key.clone(),input)?;
               attrs.push(( key.clone(), XhtmlAttr::F(f) ));
            } else if input.peek(Brace) {
               let e: XhtmlExpr = input.parse()?;
               attrs.push(( key, XhtmlAttr::E(e) ));
            } else {
               let val: Literal = input.parse()?;
               attrs.push(( key, XhtmlAttr::S(val.to_string()) ));
            }
        }

        if input.peek(Token![/]) {
           let _r1: Token![/] = input.parse()?;
           let _r2: Token![>] = input.parse()?;

           Ok(XhtmlTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: Xhtml { crumbs: vec!() },
              span: l1.span.clone()
           })
        } else {
           let _l2: Token![>] = input.parse()?;

           let inner: Xhtml = input.parse()?;

           let _r1: Token![<] = input.parse()?;
           let _r2: Token![/] = input.parse()?;
           let t2: Ident = input.parse()?;
           if t.to_string() != t2.to_string() {
              let msg = format!("Expected </{}> found </{}>", t, t2);
              let r = Error::new(t2.span(), msg);
              return Err(r)
           }
           let _r3: Token![>] = input.parse()?;
        
           Ok(XhtmlTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: inner,
              span: l1.span.clone()
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
}
impl XhtmlCrumb {
    fn span(&self) -> Span {
        match self {
            XhtmlCrumb::S(_,sp) => { sp.clone() }
            XhtmlCrumb::T(t) => { t.span.clone() }
            XhtmlCrumb::E(e) => { e.brace_token1.span.clone() }
            XhtmlCrumb::F(_) => { Span::call_site() }
            XhtmlCrumb::L(l) => { l.span() }
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


           let c: XhtmlCrumb = input.parse()?;
           cs.push(c);
        }
        Ok(cs)
    }
}
impl Parse for XhtmlCrumb {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) {
           let t: XhtmlTag = input.parse()?;
           Ok(XhtmlCrumb::T(t))
        } else if input.peek(Bracket) {
           let f: XhtmlExprF = XhtmlExprF::parse("markup".to_string(),input)?;
           Ok(XhtmlCrumb::F(f))
        } else if input.peek(Brace) {
           let e: XhtmlExpr = input.parse()?;
           Ok(XhtmlCrumb::E(e))
        } else if input.peek(LitStr) {
           let lit: LitStr = input.parse()?;
           Ok(XhtmlCrumb::L(lit))
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
        } else if input.peek(Token![as]) {
           let id: Token![as] = input.parse()?;
           Ok(XhtmlCrumb::S("as".to_string(), id.span.clone()))
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
        } else if input.peek(Token![else]) {
           let id: Token![else] = input.parse()?;
           Ok(XhtmlCrumb::S("else".to_string(), id.span.clone()))
        } else if input.peek(Token![enum]) {
           let id: Token![enum] = input.parse()?;
           Ok(XhtmlCrumb::S("enum".to_string(), id.span.clone()))
        } else if input.peek(Token![extern]) {
           let id: Token![extern] = input.parse()?;
           Ok(XhtmlCrumb::S("extern".to_string(), id.span.clone()))
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
        } else if input.peek(Token![unsafe]) {
           let id: Token![unsafe] = input.parse()?;
           Ok(XhtmlCrumb::S("unsafe".to_string(), id.span.clone()))
        } else if input.peek(Token![use]) {
           let id: Token![use] = input.parse()?;
           Ok(XhtmlCrumb::S("use".to_string(), id.span.clone()))
        } else if input.peek(Token![where]) {
           let id: Token![where] = input.parse()?;
           Ok(XhtmlCrumb::S("where".to_string(), id.span.clone()))
        } else if input.peek(Token![while]) {
           let id: Token![while] = input.parse()?;
           Ok(XhtmlCrumb::S("while".to_string(), id.span.clone()))
        } else {
           let id: Ident = input.parse()?;
           Ok(XhtmlCrumb::S(id.to_string(), id.span().clone()))
        }
    }
}
impl ToTokens for XhtmlCrumb {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           XhtmlCrumb::S(s,ss) => {
              tokens.append(Ident::new("stream", ss.clone()));
              tokens.append(Punct::new('.', Spacing::Alone));
              tokens.append(Ident::new("push_str", ss.clone()));

              let mut ts = proc_macro2::TokenStream::new();
              ts.append(Literal::string(&s));
              let gr = Group::new(Delimiter::Parenthesis, ts);
              tokens.append(gr);

              tokens.append(Punct::new(';', Spacing::Alone));
           },
           XhtmlCrumb::L(l) => {
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
           XhtmlCrumb::T(t) => {
              t.to_tokens(tokens);
           }
           XhtmlCrumb::E(e) => {
              e.to_tokens(tokens);
           }
           XhtmlCrumb::F(e) => {
              e.to_tokens(tokens);
           }
        }
    }
}

pub struct Xhtml {
    crumbs: Vec<XhtmlCrumb>
}
impl ToTokens for Xhtml {
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

impl Parse for Xhtml {
    fn parse(input: ParseStream) -> Result<Self> {
        let crumbs: Vec<XhtmlCrumb> = input.call(XhtmlCrumb::parse_outer)?;

        Ok(Xhtml {
            crumbs: crumbs
        })
    }
}