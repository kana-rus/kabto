mod internal;

#[proc_macro]
pub fn dom_tags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::dom_tags(input.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro]
pub fn component_tags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::component_tags(input.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
