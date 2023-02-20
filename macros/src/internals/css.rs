use proc_macro2::{TokenStream, Span, Ident};
use quote::{quote};
use syn::{parse::Parse, token, Error};
use super::Build;

pub(super) struct CSSInput(
    Vec<Property>
); struct Property {
    key:   Key,
    value: Ident,
}
#[allow(non_camel_case_types)]
enum Key {
    display,
    align_items
}
mod property {
    use syn::custom_keyword;

    custom_keyword!(display);
    custom_keyword!(align_items);
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
impl Parse for Key {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(property::align_items) {
            input.parse::<property::align_items>().ok();
            Ok(Self::align_items)
        } else if input.peek(property::display) {
            input.parse::<property::display>().ok();
            Ok(Self::display)
        } else {
            Err(Error::new(Span::call_site(), "unknown property"))
        }
    }
}

impl Build for CSSInput {
    fn build(self) -> TokenStream {
        let properties = self.0.into_iter().fold(TokenStream::new(), |mut token_stream, property| {
            let Property { key, value } = property;

            token_stream.extend(match key {
                Key::align_items => quote!{
                    align_items: kabto::css::property::align_items::#value,
                },
                Key::display => quote!{
                    display: kabto::css::property::display::#value,
                },
            });

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
