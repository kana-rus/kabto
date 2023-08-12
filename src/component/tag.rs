use crate::{dom, library::IntoCows};
use super::IntoNode;


pub struct a;
impl IntoNode for a {
    fn into_node(self) -> dom::Node {
        dom::a::new().into_node()
    }
}
impl a {
    pub fn class(self, class: impl IntoCows) -> dom::a {
        dom::a::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::a {
        dom::a::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::a {
        dom::a::new()
            .style(style)
    }
}
impl a {
    pub fn href(self, href: impl IntoCows) -> dom::a {
        dom::a::new()
            .href(href)
    }
    pub fn download(self, download: impl IntoCows) -> dom::a {
        dom::a::new()
            .download(download)
    }
    pub fn target(self, target: impl IntoCows) -> dom::a {
        dom::a::new()
            .target(target)
    }
    pub fn rel(self, rel: impl IntoCows) -> dom::a {
        dom::a::new()
            .rel(rel)
    }
}

pub struct div;
impl IntoNode for div {
    fn into_node(self) -> dom::Node {
        dom::div::new().into_node()
    }
}
impl div {
    pub fn class(self, class: impl IntoCows) -> dom::div {
        dom::div::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::div {
        dom::div::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::div {
        dom::div::new()
            .style(style)
    }
}

pub struct h1;
impl IntoNode for h1 {
    fn into_node(self) -> dom::Node {
        dom::h1::new().into_node()
    }
}
impl h1 {
    pub fn class(self, class: impl IntoCows) -> dom::h1 {
        dom::h1::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::h1 {
        dom::h1::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::h1 {
        dom::h1::new()
            .style(style)
    }
}
