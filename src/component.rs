use std::ops::Sub;
use crate::html::{IntoHTML, HTML};

pub trait Component {
    fn render(self) -> impl IntoHTML;
}
pub trait ParentCopmonent<Children: IntoHTML> = Sub<Children, Output = HTML>;

