mod components;
mod base;
mod tag;
#[cfg(test)] mod _test;

pub(crate) use base::BaseAttributes;
pub(crate) use tag::*;
use crate::library::{Cows, IntoCows};


pub enum Node {
    Element(Element),
    Text(Cows),
    None,
} impl Node {
    pub(crate) fn render_to(self, buf: &mut String) {
        match self {
            Self::Element(e) => e.render_to(buf),
            Self::Text(text) => buf.push_str(&text.html_escaped()),
            Self::None       => (/* render nothing */),
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
        tag.render_with_children(children, buf)
    }
}
