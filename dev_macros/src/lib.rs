mod define_tags;

#[proc_macro]
pub fn define_tags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    define_tags::define_tags(input.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
