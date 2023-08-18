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
    pub name:               Ident,
    pub with_global:        bool,
    pub with_children:      bool,
    pub boolean_attributes: Vec<BooleanAttribute>,
    pub normal_attributes:  Vec<NormalAttribute>,
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

        let boolean_attributes = if input.peek(token::Paren) {
            let boolean_attributes; parenthesized!(boolean_attributes in input);
            boolean_attributes.parse_terminated::<_, Token!(,)>(BooleanAttribute::parse)?
                .into_iter().collect::<Vec<_>>()
        } else {Vec::new()};

        let normal_attributes; bracketed!(normal_attributes in input);
        let mut normal_attributes = normal_attributes
            .parse_terminated::<_, Token!(,)>(NormalAttribute::parse)?
            .into_iter().collect::<Vec<_>>();
        if with_global {
            normal_attributes.append(&mut GlobalAttributes())
        }

        Ok(Self {name, with_global, with_children, boolean_attributes, normal_attributes })
    }
}

pub struct NormalAttribute {
    pub name:          Ident,
    pub argument_name: Ident,
} impl Parse for NormalAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut argument_name = name.clone();
        if input.peek(token::Paren) {
            let buf; parenthesized!(buf in input);
            argument_name = buf.parse()?;
        }

        Ok(Self { name, argument_name })
    }
} impl NormalAttribute {
    fn new(name: &'static str) -> Self {
        let name          = format_ident!("{name}");
        let argument_name = name.clone();
        Self { name, argument_name }
    }
    fn argument(mut self, argument_name: &'static str) -> Self {
        self.argument_name = format_ident!("{argument_name}");
        self
    }
}

pub struct BooleanAttribute {
    pub name: Ident
} impl Parse for BooleanAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        Ok(Self { name })
    }
}

#[allow(non_snake_case)] pub fn GlobalAttributes() -> Vec<NormalAttribute> {
    vec![
        NormalAttribute::new("class"),
        NormalAttribute::new("id"),
        NormalAttribute::new("style").argument("css"),
    ]
}
