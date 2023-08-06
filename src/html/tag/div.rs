// use super::{Dir, Hidden};
use crate::{
    library::{Cows, IntoCows},
    html::{HTML},
};


pub struct div {
    rendered_child:  Option<Cows>,
    class:           Option<Cows>,
    // contenteditable: Option<bool>,
    // data_attributes: Vec<Cows>,
    // dir:             Option<Dir>,
    // draggable:       Option<bool>,
    // hidden:          Option<Hidden>,
    id:              Option<Cows>,
    // role:            Option<AriaRole>,
    // slot:            Option<Cows>,
    // spellcheck:      Option<bool>,
    style:           Option<Cows>,
    // tabindex:        Option<usize>,
    // title:           Option<Cows>,
    // translate:       Option<bool>,
} const _: () = {
    impl HTML for div {
        fn render(self) -> Cows {
            let mut template = format!("<div");
            let Self {
                rendered_child,

                class,
                id,
                style,
            } = self;

            if let Some(c) = class {
                template.push_str(" class=");
                c.render_to(&mut template);
            }
            if let Some(id) = id {
                template.push_str(" id=");
                id.render_to(&mut template)
            }
            if let Some(s) = style {
                template.push_str(" style=");
                s.render_to(&mut template)
            }
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
            id:              None,
            class:           None,
            style:           None,
        }
    }
} impl div {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, style: impl IntoCows) -> Self {
        self.style.replace(style.into_cows());
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
