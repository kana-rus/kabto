use proc_macro2::TokenStream;
use syn::{Result, parse2};

mod model;
mod expand;

pub fn define_tags(input: TokenStream) -> Result<TokenStream> {
    let definition = parse2::<model::Definition>(input)?;
    Ok(definition.expand())
}
