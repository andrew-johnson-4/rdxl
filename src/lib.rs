#![recursion_limit = "128"]
#![feature(proc_macro)]
#![crate_type = "proc-macro"]
extern crate proc_macro;
#[macro_use] extern crate quote;
use self::proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, Ident, Token, Type, Visibility};

enum RDXL_crumb {
   S(String)
}

struct RDXL {
    crumbs: Vec<RDXL_crumb>
}

impl Parse for RDXL {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        Ok(RDXL {
            crumbs: vec!()
        })
    }
}

#[proc_macro]
pub fn rdxl(input: TokenStream) -> TokenStream {
    let rdxl = parse_macro_input!(input as RDXL);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        {
            "ident"
        }
    };

    TokenStream::from(expanded)
}

