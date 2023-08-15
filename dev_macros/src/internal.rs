use proc_macro2::TokenStream;
use syn::{Result, parse2};


trait Expand {
    fn expand(&self) -> TokenStream;
}
const BASE_ATTRIBUTES: &[&str] = &[
    "class",
    "id",
    "style",
];

mod dom_tags;
/// ```ignore
/// dom_tags! {
///     html @children [lang];
///     head @children [];
///     link [
///         as_, corsorigin, href, hreflang,
///         imagesizes, imagecharset, media,
///         rel, title, type_,
///     ];
///     meta [
///         charset, content, http_equiv, name,
///     ];
///     title  @children [];
///     style [media, nonce, title];
///     a @base @children [href, download, target, rel];
/// }
/// ```
pub(super) fn dom_tags(input: TokenStream) -> Result<TokenStream> {
    let tags = parse2::<dom_tags::Tags>(input)?;
    Ok(tags.expand())
}

mod component_tags;
pub(super) fn component_tags(input: TokenStream) -> Result<TokenStream> {
    let tags = parse2::<component_tags::Tags>(input)?;
    Ok(tags.expand())
}
