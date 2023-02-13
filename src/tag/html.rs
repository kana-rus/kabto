use crate::{html::{HTML, IntoHTML}, internal_macros::ParentTag};

#[allow(non_camel_case_types)]
#[derive(ParentTag)]
pub struct html {
    pub(crate) children: HTML,

    pub(crate) lang: Option<&'static str>,
}

/* derive(ParentTag):

    const _: () = {
        impl<Children: IntoHTML> Parent<Children> for html {
                fn set_children(&mut self, children: Children) {
                    self.children = children.into_html()
            }
        }
        impl<Children: IntoHTML> std::ops::Sub<Children> for html {
                type Output = Self;
                fn sub(mut self, children: Children) -> Self::Output {
                    self.set_children(children);
                    self
            }
        }
    };

*/

const _: (/* html tag impls */) = {
    impl IntoHTML for html {
        fn into_html(self) -> HTML {
            HTML(format!(
                "<html{}>{}</html>",
                if let Some(lang) = self.lang {format!(r#" lang="{lang}""#)} else {String::new()},
                self.children.0
            ))
        }
    }
};
