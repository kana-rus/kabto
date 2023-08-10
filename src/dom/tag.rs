use crate::library::{Cows, IntoCows};
use super::{components::{AnkerTarget, AnkerRel}, Node};


pub enum Tag {
    a(a),
    div(div),
    h1(h1),
} impl Tag {
    pub(crate) fn render_with_children(self, children: Vec<Node>, buf: &mut String) {
        match self {
            Self::a(a) => {
                a.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</a>".render_to(buf)
            }
            Self::div(div) => {
                div.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</div>".render_to(buf)
            }
            Self::h1(h1) => {
                h1.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</h1>".render_to(buf)
            }
        }
    }
}

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
} impl BaseAttributes {
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
    pub(crate) __base:   BaseAttributes,    
    pub(crate) href:     Option<Cows>,
    pub(crate) download: Option<Cows>,
    pub(crate) target:   Option<AnkerTarget>,
    pub(crate) rel:      Vec<AnkerRel>,
} impl a {
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base, href, download, target, mut rel } = self;
        "<a".render_to(buf);
        __base.render_to(buf);

        if let Some(h) = href {
            " href=".render_to(buf);
            h.render_quoted_to(buf)
        }
        if let Some(d) = download {
            " download=".render_to(buf);
            d.render_quoted_to(buf)
        }
        if let Some(t) = target {
            " target=".render_to(buf);
            t.as_str().render_quoted_to(buf)
        }
        if !rel.is_empty() {
            " rel=".render_to(buf);
            buf.push('"');
            rel.pop().unwrap().as_str().render_to(buf);
            for r in rel {buf.push(' '); r.as_str().render_to(buf)}
            buf.push('"')
        }

        buf.push('>')
    }
}

pub struct div {
    pub(crate) __base: BaseAttributes,
} impl div {
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<div".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
}

pub struct h1 {
    pub(crate) __base: BaseAttributes,
} impl h1 {
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<h1".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
}
