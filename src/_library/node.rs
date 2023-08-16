use crate::{Node, Element};
use super::{IntoCows};


pub trait IntoNode {
    fn into_node(self) -> Node;
} const _: () = {
    impl IntoNode for Node {
        fn into_node(self) -> Node {
            self
        }
    }

    impl<IN: IntoNode> IntoNode for Option<IN> {
        fn into_node(self) -> Node {
            match self {
                None      => Node::None,
                Some(i_n) => i_n.into_node(),
            }
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
};

pub trait NodeCollection {
    fn collect(self) -> Vec<Node>;
} const _: () = {
    impl NodeCollection for Node {
        fn collect(self) -> Vec<Node> {
            vec![self]
        }
    }
}; macro_rules! impl_children {
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
}; const _: () = {
    impl NodeCollection for (Vec<Node>,) {
        fn collect(self) -> Vec<Node> {
            self.0
        }
    }
};
