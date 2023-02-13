use proc_macro2::{TokenStream, Ident};
use quote::{quote, ToTokens};
use syn::{parse::Parse, LitChar, LitStr, token, bracketed};
use super::Build;

/// ```
/// escape!(text by [
///     '\t': "Tab"
///     '\n': "NewLine"
/// 
///     //...
/// ]);
/// ```
/// ↓
/// ```
/// let mut pos = usize;
/// for ch in &mut text.clone().chars() {
///     match ch {
///         '\t' => {text.replace_range(pos..=pos, "&Tab;"); pos += 5}
///         '\n' => {"text.replace_range(pos..=pos, "&NewLine;"); pos += 9}
/// 
///         // ...
/// 
///         _ => pos += 1
///     }
/// }
/// ```
pub(super) struct HtmlEscape {
    text_name: Ident,
    pairs: Vec<(char, String)>,
}

mod keyword {
    syn::custom_keyword!(by);
}

impl Parse for HtmlEscape {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let text_name = input.parse()?;

        input.parse::<keyword::by>()?;

        let pairs = {
            let mut pairs = vec![];
            let pairs_buf; bracketed!(pairs_buf in input);
            while !pairs_buf.is_empty() {
                let from = pairs_buf.parse::<LitChar>()?.value();
                pairs_buf.parse::<token::Colon>()?;
                let entity_name = pairs_buf.parse::<LitStr>()?.value();

                pairs.push((from, entity_name))
            }
            pairs
        };

        Ok(Self {text_name, pairs })
    }
}

impl Build for HtmlEscape {
    fn build(self) -> proc_macro2::TokenStream {
        let HtmlEscape { text_name, pairs } = self;
        let pairs = pairs.into_iter().fold(
            TokenStream::new(), |mut it, next| {
                let (from, entity_name) = next;
                let to = format!("&{entity_name};");
                let to_len = to.len();

                quote!{
                    #from => {text.replace_range(pos..=pos, #to); pos += #to_len},
                }

                .to_tokens(&mut it); it
            }
        );

        quote!{
            let mut pos = 0;
            for ch in &mut #text_name.clone().chars() {
                match ch {
                    #pairs
                    _ => pos += 1
                }
            }
        }
    }
}
