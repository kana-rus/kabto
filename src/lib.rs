#![allow(
    non_snake_case,
    non_camel_case_types,
)]

#![feature(
    unboxed_closures, fn_traits,
    tuple_trait
)]

#![allow(incomplete_features)]
#![feature(
    return_position_impl_trait_in_trait,
)]

mod _library;
mod tags;




use _library::*;
use tags::dom::Tag;


pub trait Component {
    fn render(self) -> impl HTML;
}

pub trait HTML {
    fn render_to(self, buf: &mut String);
} const _: () = {
    impl<NC: NodeCollection> HTML for NC {
        fn render_to(self, buf: &mut String) {
            for node in self.collect() {
                node.render_to(buf)
            }
        }
    }
}; const _: () = {
    impl HTML for &str {
        fn render_to(self, buf: &mut String) {
            buf.push_str(self)
        }
    }
    impl HTML for String {
        fn render_to(self, buf: &mut String) {
            buf.push_str(&self)
        }
    }
};

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

pub struct Element {
    pub(crate) tag:      Tag,
    pub(crate) children: Vec<Node>,
} impl Element {
    pub(crate) fn render_to(self, buf: &mut String) {
        let Self { tag, children } = self;
        tag.render_with_children(children, buf)
    }
}
