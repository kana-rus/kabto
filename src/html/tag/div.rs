use super::{Dir, Hidden};
use crate::{
    library::{Cows, IntoCows},
    html::{HTML},
};


pub struct div {
    rendered_child:  Option<Cows>,
    class:           Option<Cows>,
    contenteditable: Option<bool>,
    // data_attributes: Vec<Cows>,
    dir:             Option<Dir>,
    draggable:       Option<bool>,
    hidden:          Option<Hidden>,
    id:              Option<Cows>,
    // role:            Option<AriaRole>,
    // slot:            Option<Cows>,
    spellcheck:      Option<bool>,
    style:           Option<Cows>,
    tabindex:        Option<usize>,
    title:           Option<Cows>,
    translate:       Option<bool>,
} const _: () = {
    impl HTML for div {
        fn render(self) -> Cows {
            let mut template = format!("<div");
            let Self {
                rendered_child,

                class,
                contenteditable,
                dir,
                draggable,
                hidden,
                id,
                spellcheck,
                style,
                tabindex,
                title,
                translate,
            } = self;

            if let Some(c) = class {
                template.push_str(" class=");
                c.render_to(&mut template);
            }
            if let Some(b) = contenteditable {
                template.push_str(" contenteditable=");
                b.render_to(&mut template)
            }
            if let Some(d) = dir {
                template.push_str(" dir=");
                template.push_str(d.render())
            }
            if let Some(b) = draggable {
                template.push_str(" draggable=");
                b.render_to(&mut template)
            }
            if let Some(h) = hidden {
                template.push_str(" hidden=");
                h.render_to(&mut template)
            }
            if let Some(id) = id {
                template.push_str(" id=");
                id.render_to(&mut template)
            }
            if let Some(b) = spellcheck {
                template.push_str(" spellcheck=");
                b.render_to(&mut template);
            }
            if let Some(s) = style {
                template.push_str(" style=");
                s.render_to(&mut template)
            }
            if let Some(ti) = tabindex {
                template.push_str(" tabindex=");
                ti.render_to(&mut template)
            }
            if let Some(t) = title {
                template.push_str(" title=");
                t.render_to(&mut template)
            }
            if let Some(yes) = translate {
                template.push_str(" translate=");
                template.push_str(if yes {"\"yes\""} else {"\"no\""})
            }
            template.push_str(">");

            if let Some(rendered_child) = rendered_child {
                template.push_str(&rendered_child)
            }
            template.push_str("</div>");

            Cows::Owned(template)
        }
    }
}; impl div {
    pub(crate) fn new() -> Self {
        Self {
            rendered_child:  None,
            id:              None,
            class:           None,
            contenteditable: None,
            dir:             None,
            draggable:       None,
            hidden:          None,
            spellcheck:      None,
            style:           None,
            tabindex:        None,
            title:           None,
            translate:       None,
        }
    }
}
