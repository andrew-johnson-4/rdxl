#![recursion_limit = "128"]
#![feature(proc_macro)]
#![feature(type_ascription)]
#![crate_type = "proc-macro"]
extern crate proc_macro;
extern crate quote;
use self::proc_macro::TokenStream;

use quote::{quote, quote_spanned, TokenStreamExt, ToTokens};
use quote::__private::{TokenTree, Spacing, Span, Punct};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Ident, Token, Type, Visibility};

enum RdxlCrumb {
   S(String)
}
impl ToTokens for RdxlCrumb {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
           RdxlCrumb::S(s) => {
              ()
           }
        }
    }
}

struct Rdxl {
    crumbs: Vec<RdxlCrumb>
}
impl ToTokens for Rdxl {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        for (i, c) in self.crumbs.iter().enumerate() {
            c.to_tokens(tokens);
        }
    }
}

impl Parse for Rdxl {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut crumbs = vec!();
        while let Some(c) = input.parse()?: Option<Ident> {
           crumbs.push( RdxlCrumb::S(c.to_string()) );
        }
        Ok(Rdxl {
            crumbs: crumbs
        })
    }
}

#[proc_macro]
pub fn rdxl(input: TokenStream) -> TokenStream {
    let rdxl = parse_macro_input!(input as Rdxl);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        {
            let stream = String::new();
            #rdxl
            stream
        }
    };

    TokenStream::from(expanded)
}

