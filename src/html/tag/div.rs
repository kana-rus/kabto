// use super::{Dir, Hidden};
use crate::{
    library::{Cows, IntoCows},
    html::{HTML},
};
use super::BaseAttributes;


pub struct div {
    rendered_child:  Option<Cows>,
    base_attributes: BaseAttributes,
} const _: () = {
    impl HTML for div {
        fn render(self) -> Cows {
            let Self {
                rendered_child,
                base_attributes,
            } = self;

            let mut template = format!("<div");
            base_attributes.render_to(&mut template);
            template.push_str(">");

            if let Some(rendered_child) = rendered_child {
                template.push_str(&rendered_child)
            }
            template.push_str("</div>");

            Cows::Owned(template)
        }
    }
}; impl div {
    pub(crate) fn new() -> Self {
        Self {
            rendered_child:  None,
            base_attributes: BaseAttributes::new(),
        }
    }
} impl div {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.base_attributes.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.base_attributes.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, style: impl IntoCows) -> Self {
        self.base_attributes.style.replace(style.into_cows());
        self
    }
}


#[cfg(test)]
mod test {
    use crate::{HTML};

    mod __ {
        use crate::{HTML, library::{IntoCows, Cows}};

        pub struct div;
        impl HTML for div {
            fn render(self) -> Cows {
                Cows::Borrowed("<div></div>")
            }
        }
        #[allow(unused)]
        impl div {
            pub fn class(self, class: impl IntoCows) -> super::super::div {
                super::super::div::new().class(class)
            }
            pub fn id(self, id: impl IntoCows) -> super::super::div {
                super::super::div::new().id(id)
            }
            pub fn style(self, style: impl IntoCows) -> super::super::div {
                super::super::div::new().style(style)
            }
        }
    }
    #[allow(non_upper_case_globals)]
    const div: __::div = __::div;

    #[test]
    fn render_div_tag() {
        let div_1 = div.id("my-first-html-tag").style("margin: auto;");
        assert_eq!(&div_1.render(), r#"<div id="my-first-html-tag" style="margin: auto;"></div>"#);

        let div_2 = div.class("cards-box").id("game-cards-box");
        assert_eq!(&div_2.render(), r#"<div class="cards-box" id="game-cards-box"></div>"#);
    }
}
