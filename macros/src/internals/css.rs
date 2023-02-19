use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};
use syn::{parse::Parse, token};
use super::Build;

pub(super) struct CSSInput(
    Vec<Property>
); struct Property {
    key:   Ident,
    value: Ident,
}

impl Parse for CSSInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut properties = vec![];
        while !input.is_empty() {
            properties.push(Property::parse(&input)?)
        }
        Ok(Self(properties))
    }
}
impl Parse for Property {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<token::Colon>()?;
        let value = input.parse()?;
        input.parse::<token::Semi>()?;

        Ok(Self { key, value })
    }
}

/// keys of properties thats value is enum-type
const P: &'static [&'static str] = &[
    "align_items",
    "display",
];
#[inline] fn completion_for(enum_type_key: &Ident) -> Option<TokenStream> {
    if P.contains(&enum_type_key.to_string().as_str()) {
        let enum_name = camel_cased(enum_type_key);
        Some(quote!{ kabto::css::property::#enum_name:: })
    } else {
        None
    }
}
#[inline] fn camel_cased(snake_case: &Ident) -> Ident {
    format_ident!("{}", snake_case
        .to_string()
        .split('_')
        .map(|s| {
            let mut s = s.to_owned();
            unsafe { s.get_unchecked_mut(0..=0) }
                .make_ascii_uppercase();
            s
        })
        .collect::<String>()
    )
}

impl Build for CSSInput {
    fn build(self) -> TokenStream {
        let properties = self.0.into_iter().fold(TokenStream::new(), |mut token_stream, property| {
            let Property { key, value } = property;
            let value_str = value.to_string();

            token_stream.extend(
                if let Some(completion) = completion_for(&key) {
                    quote!{
                        #key: #completion #value,
                    }
                } else {
                    quote!{
                        #key: Some(#value_str),
                    }
                }
            );

            token_stream
        });

        quote!{
            kabto::css::CSS {
                #properties
                ..Default::default()
            }
        }
    }
}
