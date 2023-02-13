use crate::html::{HTML, IntoHTML};



#[allow(non_camel_case_types)]
pub struct head { 
    pub(crate) children: HTML,
}

const _: (/* head tag impls */) = {
    impl IntoHTML for head {
        fn into_html(self) -> HTML {
            HTML(format!(
                "<head>{}</head>",
                self.children.0
            ))
        }
    }
};
