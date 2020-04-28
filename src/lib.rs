#![recursion_limit = "128"]
#![feature(type_ascription)]
#![crate_type = "proc-macro"]
extern crate proc_macro;
extern crate quote;
use self::proc_macro::TokenStream;

use quote::{quote, TokenStreamExt, ToTokens};
use quote::__private::{Spacing, Span, Punct, Literal, Ident, Group, Delimiter};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident as SynIdent, Token};

struct RdxlTag {
   tag: String,
   attrs: Vec<(String,String)>,
   inner: Rdxl,
   span: Span
}
impl ToTokens for RdxlTag {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.append(Ident::new("stream", self.span.clone()));
        tokens.append(Punct::new('.', Spacing::Alone));
        tokens.append(Ident::new("push_str", self.span.clone()));
        let mut ts = quote::__private::TokenStream::new();
        ts.append(Literal::string(&format!("<{}>", self.tag)));
        let gr = Group::new(Delimiter::Parenthesis, ts);
        tokens.append(gr);
        tokens.append(Punct::new(';', Spacing::Alone));

        self.inner.to_tokens(tokens);

        tokens.append(Ident::new("stream", self.span.clone()));
        tokens.append(Punct::new('.', Spacing::Alone));
        tokens.append(Ident::new("push_str", self.span.clone()));
        let mut ts = quote::__private::TokenStream::new();
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

        let mut attrs: Vec<(String,String)> = Vec::new();
        while input.peek(SynIdent) {
            let key: Ident = input.parse()?;
            let eq: Token![=] = input.parse()?;
            let val: Literal = input.parse()?;
            attrs.push(( key.to_string(), val.to_string() ));
        }

        let l2: Token![>] = input.parse()?;

        if t.to_string() == "br" {
           Ok(RdxlTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: Rdxl { crumbs: vec!() },
              span: l1.span.clone()
           })
        } else {
           let inner: Rdxl = input.parse()?;

           let r1: Token![<] = input.parse()?;
           let r2: Token![/] = input.parse()?;
           let t2: Ident = input.parse()?;
           let r3: Token![>] = input.parse()?;
        
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
   S(String, Span),
   T(RdxlTag)
}
impl RdxlCrumb {
    fn span(&self) -> Span {
        match self {
            RdxlCrumb::S(_,sp) => { sp.clone() }
            RdxlCrumb::T(t) => { t.span.clone() }
        }
    }
    fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
        let mut cs = vec!();
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
        } else if input.peek(Token![!]) {
           let id: Token![!] = input.parse()?;
           Ok(RdxlCrumb::S("!".to_string(), id.span.clone()))
        } else if input.peek(Token![@]) {
           let id: Token![@] = input.parse()?;
           Ok(RdxlCrumb::S("@".to_string(), id.span.clone()))
        } else if input.peek(Token![#]) {
           let id: Token![#] = input.parse()?;
           Ok(RdxlCrumb::S("#".to_string(), id.span.clone()))
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
           },
           RdxlCrumb::T(t) => {
              t.to_tokens(tokens);
           }
        }
    }
}

struct Rdxl {
    crumbs: Vec<RdxlCrumb>
}
impl ToTokens for Rdxl {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let mut prev: Option<Span> = None;
        for c in self.crumbs.iter() {
            let ss = c.span();
            if let Some(sp) = prev {
            if sp.end() != ss.start() {

              tokens.append(Ident::new("stream", ss.clone()));
              tokens.append(Punct::new('.', Spacing::Alone));
              tokens.append(Ident::new("push_str", ss.clone()));

              let mut ts = quote::__private::TokenStream::new();
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

