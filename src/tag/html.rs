use std::ops::Sub;
use crate::component::{HTML, IntoHTML};

#[allow(non_camel_case_types)]
pub struct html { 
    pub(crate) children: HTML,

    pub(crate) lang: Option<&'static str>,
}

const _: (/* html tag impls */) = {
    impl<Children: IntoHTML> Sub<Children> for html {
        type Output = HTML;
        fn sub(mut self, children: Children) -> Self::Output {
            self.children = children.into_html();
            self.into_html()
        }
    }

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
