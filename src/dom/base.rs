use crate::library::{Cows, IntoCows};


pub(crate) struct BaseAttributes {
    pub(crate) class: Option<Cows>,
    pub(crate) id:    Option<Cows>,
    pub(crate) style: Option<Cows>,
} impl BaseAttributes {
    pub(crate) fn new() -> Self {
        Self { class: None, id: None, style: None }
    }
    pub(crate) fn render_to(self, buf: &mut String) {
        let Self { class, id, style } = self;
        if let Some(c) = class {  
            " class=".render_to(buf);
            c.render_quoted_to(buf)
        }
        if let Some(i) = id {
            " id=".render_to(buf);
            i.render_quoted_to(buf)
        }
        if let Some(s) = style {
            " style=".render_to(buf);
            s.render_quoted_to(buf)
        }
    }
} #[allow(unused)] impl BaseAttributes {
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
