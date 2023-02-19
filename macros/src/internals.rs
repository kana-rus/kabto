use proc_macro2::TokenStream;
use quote::quote;
use syn::{Result, parse2, ItemStruct};

trait Interpret<As> {
    fn interpret(self) -> As;
}
trait Build {
    fn build(self) -> TokenStream;
}

mod html_escape;
pub(super) fn html_escape(input: TokenStream) -> Result<TokenStream> {
    Ok(parse2::<html_escape::HtmlEscape>(input)?.build())
}

mod css;
pub(super) fn css(input: TokenStream) -> Result<TokenStream> {
    Ok(parse2::<>(input)?)
}

/// image:
/// ```no_run
/// impl<Children: IntoHTML> Parent<Children> for html {
///     fn set_children(&mut self, children: Children) {
///         self.children = children.into_html()
///     }
/// }
/// impl<Children: IntoHTML> std::ops::Sub<Children> for html {
///     type Output = Self;
///     fn sub(mut self, children: Children) -> Self::Output {
///         self.set_children(children);
///         self
///     }
/// }
/// ```
pub(super) fn derive_parent(input: TokenStream) -> Result<TokenStream> {
    let ItemStruct {ident, .. } = parse2::<ItemStruct>(input)?;
    Ok(quote!{
        const _: () = {
            impl<Children: kabto::html::IntoHTML> kabto::html::Parent<Children> for #ident {
                fn set_children(&mut self, children: Children) {
                    self.children = children.into_html()
                }
            }
            impl<Children: kabto::html::IntoHTML> std::ops::Sub<Children> for #ident {
                type Output = Self;
                fn sub(mut self, children: Children) -> Self {
                    self.set_children(children);
                    self
                }
            }
        };
    })
}
pub(super) fn derive_parent_tag(tag_def: TokenStream) -> Result<TokenStream> {
    let ItemStruct {ident, .. } = parse2::<ItemStruct>(tag_def)?;
    Ok(quote!{
        const _: () = {
            impl<Children: crate::html::IntoHTML> crate::html::Parent<Children> for #ident {
                fn set_children(&mut self, children: Children) {
                    self.children = children.into_html()
                }
            }
            impl<Children: crate::html::IntoHTML> std::ops::Sub<Children> for #ident {
                type Output = Self;
                fn sub(mut self, children: Children) -> Self {
                    self.children = children.into_html();
                    self
                }
            }
        };
    })
}
