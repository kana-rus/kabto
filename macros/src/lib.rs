use proc_macro::TokenStream;
mod internals;

// privates
#[proc_macro]
pub fn html_escape(input: TokenStream) -> TokenStream {
    internals::html_escape(input.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
#[proc_macro_derive(ParentTag)]
pub fn derive_parent_tag(tag_def: TokenStream) -> TokenStream {
    internals::derive_parent_tag(tag_def.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

// publics
#[proc_macro_derive(Parent)]
pub fn derive_parent(input: TokenStream) -> TokenStream {
    internals::derive_parent(input.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    internals::css(input.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
