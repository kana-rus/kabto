mod tag;

use std::marker::Tuple;

use crate::{
    library::{IntoCows},
    dom::{Node, Element, a, Tag, div, h1},
};


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
};const _: () = {
    impl IntoNode for a {
        fn into_node(self) -> Node {
            Node::Element(Element {
                tag: Tag::a(self),
                children: vec![],
            })
        }
    }
    impl IntoNode for div {
        fn into_node(self) -> Node {
            Node::Element(Element {
                tag: Tag::div(self),
                children: vec![],
            })
        }
    }
    impl IntoNode for h1 {
        fn into_node(self) -> Node {
            Node::Element(Element {
                tag: Tag::h1(self),
                children: vec![],
            })
        }
    }
};

pub trait Children: Tuple {
    fn collect(self) -> Vec<Node>;
} macro_rules! impl_children {
    ( $( $name:ident ),* ) => {
        #[allow(non_snake_case)]
        impl<$( $name:IntoNode ),*> Children for ( $( $name,)* ) {
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
