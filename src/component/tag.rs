use crate::{dom::{self, Node, Element, Tag}, library::IntoCows};
use super::{IntoNode, NodeCollection};


pub struct a;
impl IntoNode for a {
    fn into_node(self) -> Node {
        dom::a::new().into_node()
    }
} impl a {
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
} impl a {
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
} impl<Children: NodeCollection> FnOnce<Children> for a {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::a(dom::a::new()),
            children: children.collect(),
        })
    }
}

pub struct div;
impl IntoNode for div {
    fn into_node(self) -> Node {
        dom::div::new().into_node()
    }
} impl div {
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
} impl<Children: NodeCollection> FnOnce<Children> for div {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::div(dom::div::new()),
            children: children.collect(),
        })
    }
}

pub struct h1;
impl IntoNode for h1 {
    fn into_node(self) -> Node {
        dom::h1::new().into_node()
    }
} impl h1 {
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
} impl<Children: NodeCollection> FnOnce<Children> for h1 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h1(dom::h1::new()),
            children: children.collect(),
        })
    }
}
