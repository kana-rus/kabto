use std::marker::Tuple;

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
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for a {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::a(dom::a::new()),
            children: children.collect(),
        })
    }
}

pub struct p;
impl IntoNode for p {
    fn into_node(self) -> Node {
        dom::p::new().into_node()
    }
} impl p {
    pub fn class(self, class: impl IntoCows) -> dom::p {
        dom::p::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::p {
        dom::p::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::p {
        dom::p::new()
            .style(style)
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for p {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::p(dom::p::new()),
            children: children.collect(),
        })
    }
}

pub struct span;
impl IntoNode for span {
    fn into_node(self) -> Node {
        dom::span::new().into_node()
    }
} impl span {
    pub fn class(self, class: impl IntoCows) -> dom::span {
        dom::span::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::span {
        dom::span::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::span {
        dom::span::new()
            .style(style)
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for span {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::span(dom::span::new()),
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
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for div {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::div(dom::div::new()),
            children: children.collect(),
        })
    }
}

pub struct body;
impl IntoNode for body {
    fn into_node(self) -> Node {
        dom::body::new().into_node()
    }
} impl body {
    pub fn class(self, class: impl IntoCows) -> dom::body {
        dom::body::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::body {
        dom::body::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::body {
        dom::body::new()
            .style(style)
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for body {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::body(dom::body::new()),
            children: children.collect(),
        })
    }
}

pub struct header;
impl IntoNode for header {
    fn into_node(self) -> Node {
        dom::header::new().into_node()
    }
} impl header {
    pub fn class(self, class: impl IntoCows) -> dom::header {
        dom::header::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::header {
        dom::header::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::header {
        dom::header::new()
            .style(style)
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for header {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::header(dom::header::new()),
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
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for h1 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h1(dom::h1::new()),
            children: children.collect(),
        })
    }
}

pub struct h2;
impl IntoNode for h2 {
    fn into_node(self) -> Node {
        dom::h2::new().into_node()
    }
} impl h2 {
    pub fn class(self, class: impl IntoCows) -> dom::h2 {
        dom::h2::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::h2 {
        dom::h2::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::h2 {
        dom::h2::new()
            .style(style)
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for h2 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h2(dom::h2::new()),
            children: children.collect(),
        })
    }
}

pub struct h3;
impl IntoNode for h3 {
    fn into_node(self) -> Node {
        dom::h3::new().into_node()
    }
} impl h3 {
    pub fn class(self, class: impl IntoCows) -> dom::h3 {
        dom::h3::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::h3 {
        dom::h3::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::h3 {
        dom::h3::new()
            .style(style)
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for h3 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h3(dom::h3::new()),
            children: children.collect(),
        })
    }
}

pub struct h4;
impl IntoNode for h4 {
    fn into_node(self) -> Node {
        dom::h4::new().into_node()
    }
} impl h4 {
    pub fn class(self, class: impl IntoCows) -> dom::h4 {
        dom::h4::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::h4 {
        dom::h4::new()
            .id(id)
    }
    pub fn style(self, style: impl IntoCows) -> dom::h4 {
        dom::h4::new()
            .style(style)
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for h4 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h4(dom::h4::new()),
            children: children.collect(),
        })
    }
}
