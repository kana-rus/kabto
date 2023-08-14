use std::marker::Tuple;
use crate::{dom::{self, Node, Element, Tag}, library::IntoCows};
use super::{IntoNode, NodeCollection};


pub struct html;
impl html {
    pub fn lang(self, lang: impl IntoCows) -> dom::html {
        dom::html::new().lang(lang)
    }
} impl IntoNode for html {
    fn into_node(self) -> Node {
        dom::html::new().into_node()
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for html {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::html(dom::html::new()),
            children: children.collect(),
        })
    }
}

pub struct head;
impl IntoNode for head {
    fn into_node(self) -> Node {
        dom::head::new().into_node()
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for head {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::head(dom::head::new()),
            children: children.collect(),
        })
    }
}

pub struct link;
impl IntoNode for link {
    fn into_node(self) -> Node {
        dom::link::new().into_node()
    }
} // `link` DOESN'T implement `FnOnce<Children>`
impl link {
    pub fn as_(self, as_: impl IntoCows) -> dom::link {
        dom::link::new().as_(as_)
    }
    pub fn corsorigin(self, corsorigin: impl IntoCows) -> dom::link {
        dom::link::new().corsorigin(corsorigin)
    }
    pub fn href(self, href: impl IntoCows) -> dom::link {
        dom::link::new().href(href)
    }
    pub fn hreflang(self, hreflang: impl IntoCows) -> dom::link {
        dom::link::new().hreflang(hreflang)
    }
    pub fn imagesizes(self, imagesizes: impl IntoCows) -> dom::link {
        dom::link::new().imagesizes(imagesizes)
    }
    pub fn imagesrcset(self, imagesrcset: impl IntoCows) -> dom::link {
        dom::link::new().imagesrcset(imagesrcset)
    }
    pub fn media(self, media: impl IntoCows) -> dom::link {
        dom::link::new().media(media)
    }
    pub fn rel(self, rel: impl IntoCows) -> dom::link {
        dom::link::new().rel(rel)
    }
    pub fn title(self, title: impl IntoCows) -> dom::link {
        dom::link::new().title(title)
    }
    pub fn type_(self, type_: impl IntoCows) -> dom::link {
        dom::link::new().type_(type_)
    }
}

pub struct style;
impl style {
    pub fn media(self, media: impl IntoCows) -> dom::style {
        dom::style::new().media(media)
    }
    pub fn nonce(self, nonce: impl IntoCows) -> dom::style {
        dom::style::new().nonce(nonce)
    }
    pub fn title(self, title: impl IntoCows) -> dom::style {
        dom::style::new().title(title)
    }
} impl IntoNode for style {
    fn into_node(self) -> Node {
        dom::style::new().into_node()
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for style {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::style(dom::style::new()),
            children: children.collect(),
        })
    }
}


pub struct a;
impl a {
    pub fn class(self, class: impl IntoCows) -> dom::a {
        dom::a::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::a {
        dom::a::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::a {
        dom::a::new()
            .style(css)
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
} impl IntoNode for a {
    fn into_node(self) -> Node {
        dom::a::new().into_node()
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
impl p {
    pub fn class(self, class: impl IntoCows) -> dom::p {
        dom::p::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::p {
        dom::p::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::p {
        dom::p::new()
            .style(css)
    }
} impl IntoNode for p {
    fn into_node(self) -> Node {
        dom::p::new().into_node()
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
impl span {
    pub fn class(self, class: impl IntoCows) -> dom::span {
        dom::span::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::span {
        dom::span::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::span {
        dom::span::new()
            .style(css)
    }
} impl IntoNode for span {
    fn into_node(self) -> Node {
        dom::span::new().into_node()
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
impl div {
    pub fn class(self, class: impl IntoCows) -> dom::div {
        dom::div::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::div {
        dom::div::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::div {
        dom::div::new()
            .style(css)
    }
} impl IntoNode for div {
    fn into_node(self) -> Node {
        dom::div::new().into_node()
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
impl body {
    pub fn class(self, class: impl IntoCows) -> dom::body {
        dom::body::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::body {
        dom::body::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::body {
        dom::body::new()
            .style(css)
    }
} impl IntoNode for body {
    fn into_node(self) -> Node {
        dom::body::new().into_node()
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
impl header {
    pub fn class(self, class: impl IntoCows) -> dom::header {
        dom::header::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::header {
        dom::header::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::header {
        dom::header::new()
            .style(css)
    }
} impl IntoNode for header {
    fn into_node(self) -> Node {
        dom::header::new().into_node()
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
impl h1 {
    pub fn class(self, class: impl IntoCows) -> dom::h1 {
        dom::h1::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::h1 {
        dom::h1::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::h1 {
        dom::h1::new()
            .style(css)
    }
} impl IntoNode for h1 {
    fn into_node(self) -> Node {
        dom::h1::new().into_node()
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
impl h2 {
    pub fn class(self, class: impl IntoCows) -> dom::h2 {
        dom::h2::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::h2 {
        dom::h2::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::h2 {
        dom::h2::new()
            .style(css)
    }
} impl IntoNode for h2 {
    fn into_node(self) -> Node {
        dom::h2::new().into_node()
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
impl h3 {
    pub fn class(self, class: impl IntoCows) -> dom::h3 {
        dom::h3::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::h3 {
        dom::h3::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::h3 {
        dom::h3::new()
            .style(css)
    }
} impl IntoNode for h3 {
    fn into_node(self) -> Node {
        dom::h3::new().into_node()
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
impl h4 {
    pub fn class(self, class: impl IntoCows) -> dom::h4 {
        dom::h4::new()
            .class(class)
    }
    pub fn id(self, id: impl IntoCows) -> dom::h4 {
        dom::h4::new()
            .id(id)
    }
    pub fn style(self, css: impl IntoCows) -> dom::h4 {
        dom::h4::new()
            .style(css)
    }
} impl IntoNode for h4 {
    fn into_node(self) -> Node {
        dom::h4::new().into_node()
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
