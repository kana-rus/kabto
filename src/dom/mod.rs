mod components;
#[cfg(test)] mod test;

mod tag;


use crate::library::{Cows, IntoCows};


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
    pub(crate) tag:      Tag,
    pub(crate) children: Vec<Node>,
} impl Element {
    pub(crate) fn render_to(self, buf: &mut String) {
        let Self { tag, children } = self;
        match tag {
            Tag::a(tag::a { __base, href, download, target, rel }) => {
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
                __base.render_to(buf);
                buf.push('>');
                for c in children {c.render_to(buf)}
                buf.push_str("</a>")
            }
            Tag::div(tag::div { __base }) => {
                buf.push_str("<div");
                __base.render_to(buf);
                buf.push('>');
                for c in children {c.render_to(buf)}
                buf.push_str("</div>")
            }
            Tag::h1(tag::h1 { __base }) => {
                buf.push_str("<h1");
                __base.render_to(buf);
                buf.push('>');
                for c in children {c.render_to(buf)}
                buf.push_str("</h1>")
            }
        }
    }
}

pub enum Tag {
    a(tag::a),
    div(tag::div),
    h1(tag::h1),
}
