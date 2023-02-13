use proc_macro::TokenStream;
mod internals;

#[proc_macro]
pub fn html_escape(input: TokenStream) -> TokenStream {
    internals::html_escape(input.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
