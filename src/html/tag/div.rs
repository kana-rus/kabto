use ::std::marker::Tuple;
use super::BaseElement;
use crate::{
    library::{Cows, IntoCows},
    html::{HTML},
};


pub struct div {
    rendered_children: Option<Cows>,
    base_attributes:   BaseElement,
} impl div {
    pub(crate) fn new() -> Self {
        Self {
            rendered_children: None,
            base_attributes:   BaseElement::new(),
        }
    }
    pub(crate) fn set_children(&mut self, rendered_children: Cows) {
        self.rendered_children.replace(rendered_children);
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
} const _: () = {
    impl HTML for div {
        fn render(self) -> Cows {
            let Self {
                rendered_children,
                base_attributes,
            } = self;

            let mut template = format!("<div");
            base_attributes.render_to(&mut template);
            template.push_str(">");

            if let Some(rendered_children) = rendered_children {
                template.push_str(&rendered_children)
            }
            template.push_str("</div>");

            Cows::Owned(template)
        }
    }

    impl<Children: HTML + Tuple> FnOnce<Children> for div {
        type Output = Cows;
        extern "rust-call" fn call_once(mut self, children: Children) -> Self::Output {
            self.set_children(children.render());
            self.render()
        }
    }
}; 

#[cfg(test)]
mod test {
    use std::marker::{Tuple};
    use crate::{HTML, library::{IntoCows, Cows}};

    struct div;
    #[allow(unused)]
    impl div {
        pub fn class(self, class: impl IntoCows) -> super::div {
            super::div::new().class(class)
        }
        pub fn id(self, id: impl IntoCows) -> super::div {
            super::div::new().id(id)
        }
        pub fn style(self, style: impl IntoCows) -> super::div {
            super::div::new().style(style)
        }
    }
    const _: () = {
        impl HTML for div {
            fn render(self) -> Cows {
                Cows::Borrowed("<div></div>")
            }
        }
        impl<Children: HTML + Tuple> FnOnce<Children> for div {
            type Output = Cows;
            extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
                let mut this = super::div::new();
                this.set_children(children.render());
                this.render()
            }
        }
    };

    #[test]
    fn render_div() {
        let div_0 = div;
        assert_eq!(&div_0.render(), r#"<div></div>"#);

        let div_1 = div.id("my-first-html-tag").style("margin: auto;");
        assert_eq!(&div_1.render(), r#"<div id="my-first-html-tag" style="margin: auto;"></div>"#);

        let div_2 = div.class("cards-box").id("game-cards-box");
        assert_eq!(&div_2.render(), r#"<div class="cards-box" id="game-cards-box"></div>"#);
    }

    #[test]
    fn render_div_with_children() {
        let div_0 =
            div.id("main")(
                div.class("card main-card"),
                div.class("card").style("margin-top: 8px;"),
            );
        assert_eq!(div_0,
            r#"<div id="main"><div class="card main-card"></div><div class="card" style="margin-top: 8px;"></div></div>"#
        );

        let div_1 =
            div.id("main")(
                div.class("card main-card")(
                    "This is a card!!!"
                )
            );
        assert_eq!(div_1,
            r#"<div id="main"><div class="card main-card">This&#32;is&#32;a&#32;card!!!</div></div>"#
        );
    }
}
