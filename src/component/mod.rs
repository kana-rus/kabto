pub(crate) mod tag;
#[cfg(test)] mod _test;

use std::marker::Tuple;
use crate::{
    library::{IntoCows},
    dom::{self, Node, Element, Tag},
};


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
};const _: () = {
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

    impl HTML for Node {
        fn render_to(self, buf: &mut String) {
            self.render_to(buf)
        }
    }
};const _: () = {
    impl HTML for crate::a {
        fn render_to(self, buf: &mut String) {
            self.into_node().render_to(buf)
        }
    }
    impl HTML for crate::div {
        fn render_to(self, buf: &mut String) {
            self.into_node().render_to(buf)
        }
    }
    impl HTML for crate::h1 {
        fn render_to(self, buf: &mut String) {
            self.into_node().render_to(buf)
        }
    }

    impl HTML for crate::dom::a {
        fn render_to(self, buf: &mut String) {
            self.into_node().render_to(buf)
        }
    }
    impl HTML for crate::dom::div {
        fn render_to(self, buf: &mut String) {
            self.into_node().render_to(buf)
        }
    }
    impl HTML for crate::dom::h1 {
        fn render_to(self, buf: &mut String) {
            self.into_node().render_to(buf)
        }
    }
};


pub trait IntoNode {
    fn into_node(self) -> Node;
} const _: () = {
    impl IntoNode for Node {
        fn into_node(self) -> Node {
            self
        }
    }

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
};const _: () = {
    impl IntoNode for dom::a {
        fn into_node(self) -> Node {
            Node::Element(Element {
                tag: Tag::a(self),
                children: vec![],
            })
        }
    }
    impl IntoNode for dom::div {
        fn into_node(self) -> Node {
            Node::Element(Element {
                tag: Tag::div(self),
                children: vec![],
            })
        }
    }
    impl IntoNode for dom::h1 {
        fn into_node(self) -> Node {
            Node::Element(Element {
                tag: Tag::h1(self),
                children: vec![],
            })
        }
    }
};


pub trait NodeCollection: Tuple {
    fn collect(self) -> Vec<Node>;
} macro_rules! impl_children {
    ( $( $name:ident ),* ) => {
        #[allow(non_snake_case)]
        impl<$( $name:IntoNode ),*> NodeCollection for ( $( $name,)* ) {
            fn collect(self) -> Vec<Node> {
                let ( $( $name, )* ) = self;
                vec![ $( $name.into_node(), )* ]
            }
        }
    };
} const _: () = {
    impl_children!();
    impl_children!(IN1);
    impl_children!(IN1, IN2);
    impl_children!(IN1, IN2, IN3);
    impl_children!(IN1, IN2, IN3, IN4);
    impl_children!(IN1, IN2, IN3, IN4, IN5);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9, IN10);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9, IN10, IN11);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9, IN10, IN11, IN12);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9, IN10, IN11, IN12, IN13);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9, IN10, IN11, IN12, IN13, IN14);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9, IN10, IN11, IN12, IN13, IN14, IN15);
    impl_children!(IN1, IN2, IN3, IN4, IN5, IN6, IN7, IN8, IN9, IN10, IN11, IN12, IN13, IN14, IN15, IN16);
};
