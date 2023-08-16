mod define_tags;

#[proc_macro]
/// <br/>
/// 
/// ```ignore
/// define_tags! {
///     html @children [lang];
///     head @children [];
///     meta [charset, content, http_equiv, name];
///     title @children [];
///     style @children [media, nonce, title];
///     a @global @children [href, download, target, rel];
///     p @global [];
///     span @global [];
/// 
///     // ...
/// }
/// ```
pub fn define_tags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    define_tags::define_tags(input.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
