#![allow(non_camel_case_types)]
mod components;

use crate::library::{Cows, IntoCows};
use components::*;


pub(crate) struct DOM {
    root: Node,
}

enum Node {
    Element(Element),
    Text(Cows),
}

/// ```ignore
/// txtxtxtxtxt<strong>STRONGSTRONGS!!!</strong>xtxtxtxt...
/// ```
/// みたいなものもあるので、`children` に `Element` と `Text` が
/// 混ざって並ぶ可能性もある \therefore `Vec<Node>`
struct Element {
    tag:      Tag,
    base:     BaseElement,
    children: Vec<Node>,
}

enum Tag {
    a {
        href:     Option<Cows>,
        download: Option<Cows>,
        target:   Option<AnkerTarget>,
        rel:      Vec<AnkerRel>,
    },
    div,
    h1,
}

struct BaseElement {
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
            c.render_to(buf)
        }
        if let Some(i) = id {
            buf.push_str(" id=");
            i.render_to(buf)
        }
        if let Some(s) = style {
            buf.push_str(" style=");
            s.render_to(buf)
        }
    }
}
