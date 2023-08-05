use super::{AriaRole, Dir};


pub struct div {
    id: Option<String>,
    class: Vec<String>,
    contenteditable: bool,
    data_attributes: Vec<String>,
    dir: Dir,
    draggable: bool,
    hidden: bool,
    role: Option<AriaRole>,
    slot: Option<String>,
    spellcheck: bool,
    style: Option<String>,
    tabindex: Option<usize>,
    title: Option<String>,
    translate: bool,
} impl div {
    pub(crate) fn new() -> div {
        div {

        }
    }
}
