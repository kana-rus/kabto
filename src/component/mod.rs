use crate::{HTML};


pub trait Component {
    fn render(self) -> impl HTML;
}
