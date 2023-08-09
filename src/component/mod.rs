mod tag;

use crate::{dom::{Node, Element}, library::IntoCows};


pub trait HTML {
    fn render_to(self, buf: &mut String);
}

pub trait IntoNode {
    fn into_node(self) -> Node;
} const _: () = {
    impl IntoNode for Element {
        fn into_node(self) -> Node {
            Node::Element(self)
        }
    }

    impl<IC: IntoCows> IntoNode for IC {
        fn into_node(self) -> Node {
            Node::Text(self.into_cows())
        }
    }
};
