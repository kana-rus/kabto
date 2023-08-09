use crate::{library::Cows};
use super::{DOM, Node, Element, Tag, BaseElement};


#[test] fn render_dom() {
    let dom = DOM {root: Node::Element(
        Element {
            tag:      Tag::div,
            children: vec![],
            base:     BaseElement {
                class: Some(Cows::Borrowed("card")),
                ..BaseElement::new()
            },
        }
    )}; assert_eq!(dom.render(),
        r#"<div class="card"></div>"#
    );

    let dom = DOM {root: Node::Element(
        Element {
            tag:      Tag::div,
            base:     BaseElement {
                class: Some(Cows::Borrowed("card")),
                ..BaseElement::new()
            },
            children: vec![
                Node::Text(Cows::Borrowed("This is card!!!"))
            ],
        }
    )}; assert_eq!(dom.render(),
        r#"<div class="card">This&#32;is&#32;card!!!</div>"#
    );

    let dom = DOM {
        root: Node::Element(Element {
            tag:      Tag::div,
            base:     BaseElement {
                class: Some(Cows::Borrowed("card")),
                id:    Some(Cows::Borrowed("card")),
                ..BaseElement::new()
            },
            children: vec![
                Node::Element(Element {
                    tag: Tag::div,
                    base: BaseElement {
                        class: Some(Cows::Borrowed("card-inner")),
                        ..BaseElement::new()
                    },
                    children: vec![
                        Node::Text(Cows::Borrowed("This is card!!!"))
                    ]
                })
            ],
        }
    )}; assert_eq!(dom.render(),
        r#"<div class="card" id="card"><div class="card-inner">This&#32;is&#32;card!!!</div></div>"#
    );
}
