use crate::{library::Cows, dom::tag::{self, BaseAttributes}};
use super::{Node, Element, Tag};


fn render(node: Node) -> String {
    let mut buf = String::new();
    node.render_to(&mut buf);
    buf
}

#[test] fn render_dom() {
    let dom = render(Node::Element(Element {
        tag: Tag::div(tag::div {
            __base: BaseAttributes::new().class("card")
        }),
        children: vec![],
    })); assert_eq!(dom,
        r#"<div class="card"></div>"#
    );

    let dom = render(Node::Element(Element {
        tag: Tag::div(tag::div {
            __base: BaseAttributes::new().class("card")
        }),
        children: vec![
            Node::Text(Cows::Borrowed("This is card!!!"))
        ],
    })); assert_eq!(dom,
        r#"<div class="card">This&#32;is&#32;card!!!</div>"#
    );

    let dom = render(Node::Element(Element {
        tag: Tag::div(tag::div {
            __base: BaseAttributes::new().class("card").id("card")
        }),
        children: vec![
            Node::Element(Element {
                tag: Tag::div(tag::div {
                    __base: BaseAttributes::new().class("card-inner")
                }),
                children: vec![
                    Node::Text(Cows::Borrowed("This is card!!!"))
                ]
            })
        ],
    })); assert_eq!(dom,
        r#"<div class="card" id="card"><div class="card-inner">This&#32;is&#32;card!!!</div></div>"#
    );
}
