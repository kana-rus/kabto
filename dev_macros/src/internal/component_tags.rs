use super::{Expand, BASE_ATTRIBUTES};
use quote::{quote, format_ident};
use proc_macro2::{Ident, TokenStream};
use syn::{punctuated::Punctuated, parse::Parse, Token, bracketed};


mod keywords {
    syn::custom_keyword!(base);
    syn::custom_keyword!(children);
}

pub struct Tags {
    tags: Punctuated<Tag, Token!(;)>,
} impl Parse for Tags {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            tags: input.parse_terminated(Tag::parse)?
        })
    }
} impl Expand for Tags {
    fn expand(&self) -> TokenStream {
        todo!()
    }
}

struct Tag {

} impl Parse for Tag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}