use crate::html::{HTML, IntoHTML};

#[allow(non_camel_case_types)]
pub struct html { 
    pub(crate) children: HTML,

    pub(crate) lang: Option<&'static str>,
}

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
