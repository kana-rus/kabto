#![allow(non_camel_case_types)]
mod components;

use crate::library::{Cows, IntoCows};
use components::*;


pub(crate) struct DOM {
    root: Node,
}

enum Node {
    a(a),
    div(div),
    h1(h1),
    text(Cows),
}

pub(crate) struct BaseElement {
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

struct a {
    base: BaseElement,

    href:     Option<Cows>,
    download: Option<Cows>,
    target:   Option<AnkerTarget>,
    rel:      Vec<AnkerRel>,
}

struct div {
    base: BaseElement,
}

struct h1 {
    base: BaseElement,
}
