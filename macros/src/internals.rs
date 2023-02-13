use proc_macro2::TokenStream;
use syn::{Result, parse2};

trait Interpret<As> {
    fn interpret(self) -> As;
}
trait Build {
    fn build(self) -> TokenStream;
}

mod escape;
pub(super) fn html_escape(input: TokenStream) -> Result<TokenStream> {
    Ok(parse2::<escape::HtmlEscape>(input)?.build())
}
