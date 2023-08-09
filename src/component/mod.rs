use crate::dom::Node;


pub struct a;
pub struct div;
pub struct h1;

pub trait HTML {
    fn render_to(self, buf: &mut String);
}

pub trait IntoNode {
    fn into_node(self) -> Node;
}
