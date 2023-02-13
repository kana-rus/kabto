use std::ops::Sub;
use crate::html::IntoHTML;

pub use kabto_macros::Parent;

pub trait Component {
    fn render(self) -> impl IntoHTML;
}
pub trait Parent<Children: IntoHTML>: Sub<Children, Output = Self> {
    fn set_children(&mut self, children: Children);
}
