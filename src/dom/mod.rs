mod components;
#[cfg(test)] mod test;

use crate::library::{Cows, IntoCows};
use components::*;


pub enum Node {
    Element(Element),
    Text(Cows),
} impl Node {
    pub(crate) fn render_to(self, buf: &mut String) {
        match self {
            Self::Element(e) => e.render_to(buf),
            Self::Text(text) => buf.push_str(&text.html_escaped())
        }
    } 
}

/// ```ignore
/// txtxtxtxtxt<strong>STRONGSTRONGS!!!</strong>xtxtxtxt...
/// ```
/// みたいなものもあるので、`children` に `Element` と `Text` が
/// 混ざって並ぶ可能性もある \therefore `children: Vec<Node>`
pub struct Element {
    tag:      Tag,
    base:     BaseElement,
    children: Vec<Node>,
} impl Element {
    pub(crate) fn render_to(self, buf: &mut String) {
        let Self { tag, base, children } = self;
        match tag {
            Tag::a { href, download, target, rel } => {
                buf.push_str("<a");
                if let Some(h) = href {
                    buf.push_str(" href=");
                    h.render_quoted_to(buf)
                }
                if let Some(d) = download {
                    buf.push_str(" download=");
                    d.render_quoted_to(buf)
                }
                if let Some(t) = target {
                    buf.push_str(" target=");
                    t.as_str().render_quoted_to(buf)
                }
                if !rel.is_empty() {
                    buf.push_str(" rel=");
                    rel.into_iter().fold(String::new(), |mut s, rel| {
                        s.push_str(rel.as_str()); s}).render_quoted_to(buf)
                }
                base.render_to(buf);
                buf.push('>');
                for c in children {c.render_to(buf)}
                buf.push_str("</a>")
            }
            Tag::div => {
                buf.push_str("<div");
                base.render_to(buf);
                buf.push('>');
                for c in children {c.render_to(buf)}
                buf.push_str("</div>")
            }
            Tag::h1 => {
                buf.push_str("<h1");
                base.render_to(buf);
                buf.push('>');
                for c in children {c.render_to(buf)}
                buf.push_str("</h1>")
            }
        }
    }
}

pub enum Tag {
    a {
        href:     Option<Cows>,
        download: Option<Cows>,
        target:   Option<AnkerTarget>,
        rel:      Vec<AnkerRel>,
    },
    div,
    h1,
}

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
