use proc_macro2::{Ident};
use quote::format_ident;
use syn::{punctuated::Punctuated, parse::Parse, token, Token, bracketed, parenthesized};


mod keywords {
    syn::custom_keyword!(global);
    syn::custom_keyword!(children);
}

pub struct Definition {
    pub tags: Punctuated<Tag, Token!(;)>,
} impl Parse for Definition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            tags: input.parse_terminated(Tag::parse)?
        })
    }
}

pub struct Tag {
    pub name:           Ident,
    pub with_global:    bool,
    pub with_children:  bool,
    pub own_attributes: Vec<Attribute>,
} impl Parse for Tag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let mut with_global   = false;
        let mut with_children = false;
        while input.peek(Token!(@)) {
            input.parse::<Token!(@)>().unwrap();

            if input.peek(keywords::global) {
                input.parse::<keywords::global>().unwrap();
                with_global = true
            } else if input.peek(keywords::children) {
                input.parse::<keywords::children>().unwrap();
                with_children = true
            }
        }

        let own_attributes; bracketed!(own_attributes in input);
        let own_attributes = own_attributes
            .parse_terminated::<_, Token!(,)>(Attribute::parse)?
            .into_iter()
            .collect::<Vec<_>>();

        Ok(Self {name, with_global, with_children, own_attributes })
    }
}

pub struct Attribute {
    pub name:          Ident,
    pub argument_name: Ident,
} impl Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut argument_name = name.clone();
        if input.peek(token::Paren) {
            let buf; parenthesized!(buf in input);
            argument_name = buf.parse()?;
        }

        Ok(Self { name, argument_name })
    }
}

#[allow(non_snake_case)] pub fn GlobalAttributes() -> Vec<Attribute> {
    vec![
        Attribute {
            name:          format_ident!("class"),
            argument_name: format_ident!("class"),
        },
        Attribute {
            name:          format_ident!("id"),
            argument_name: format_ident!("id"),
        },
        Attribute {
            name:          format_ident!("style"),
            argument_name: format_ident!("css"),
        },
    ]
}
