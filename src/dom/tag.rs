use crate::library::{Cows, IntoCows};

use super::components::{AnkerTarget, AnkerRel};


pub struct BaseElement {
    pub(crate) class: Option<Cows>,
    pub(crate) id:    Option<Cows>,
    pub(crate) style: Option<Cows>,
} impl BaseElement {
    pub(crate) fn new() -> Self {
        Self { class: None, id: None, style: None }
    }
    pub(crate) fn render_to(self, buf: &mut String) {
        let Self { class, id, style } = self;
        if let Some(c) = class {  
            buf.push_str(" class=");
            c.render_quoted_to(buf)
        }
        if let Some(i) = id {
            buf.push_str(" id=");
            i.render_quoted_to(buf)
        }
        if let Some(s) = style {
            buf.push_str(" style=");
            s.render_quoted_to(buf)
        }
    }
} impl BaseElement {
    pub(crate) fn class(mut self, class: impl IntoCows) -> Self {
        self.class.replace(class.into_cows());
        self
    }
    pub(crate) fn id(mut self, id: impl IntoCows) -> Self {
        self.id.replace(id.into_cows());
        self
    }
    pub(crate) fn style(mut self, style: impl IntoCows) -> Self {
        self.style.replace(style.into_cows());
        self
    }
}


pub struct a {
    pub(crate) __base:   BaseElement,
    pub(crate) href:     Option<Cows>,
    pub(crate) download: Option<Cows>,
    pub(crate) target:   Option<AnkerTarget>,
    pub(crate) rel:      Vec<AnkerRel>,
}

pub struct div {
    pub(crate) __base: BaseElement,
}

pub struct h1 {
    pub(crate) __base: BaseElement,
}
