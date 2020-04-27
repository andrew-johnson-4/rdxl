#![recursion_limit = "128"]
#![feature(type_ascription)]
#![crate_type = "proc-macro"]
extern crate proc_macro;
extern crate quote;
use self::proc_macro::TokenStream;

use quote::{quote, TokenStreamExt, ToTokens};
use quote::__private::{Spacing, Span, Punct, Literal, Ident, Group, Delimiter};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident as SynIdent};

enum RdxlCrumb {
   S(String, Span)
}
impl RdxlCrumb {
    fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
        let mut cs = vec!();
        while input.peek(SynIdent) {
           let c: RdxlCrumb = input.parse()?;
           cs.push(c);
        }
        Ok(cs)
    }
}
impl Parse for RdxlCrumb {
    fn parse(input: ParseStream) -> Result<Self> {
        let id: Ident = input.parse()?;
        Ok(RdxlCrumb::S(id.to_string(), id.span().clone()))
    }
}
impl ToTokens for RdxlCrumb {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
           RdxlCrumb::S(s,ss) => {
              tokens.append(Ident::new("stream", ss.clone()));
              tokens.append(Punct::new('.', Spacing::Alone));
              tokens.append(Ident::new("push_str", ss.clone()));

              let mut ts = quote::__private::TokenStream::new();
              ts.append(Literal::string(&s));
              let gr = Group::new(Delimiter::Parenthesis, ts);
              tokens.append(gr);

              tokens.append(Punct::new(';', Spacing::Alone));
           }
        }
    }
}

struct Rdxl {
    crumbs: Vec<RdxlCrumb>
}
impl ToTokens for Rdxl {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        for c in self.crumbs.iter() {
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

