use crate::_library::{Cows, IntoCows};


pub(crate) struct GlobalAttributes {
    pub(crate) class: Option<Cows>,
    pub(crate) id:    Option<Cows>,
    pub(crate) style: Option<Cows>,
} impl GlobalAttributes {
    pub(crate) fn new() -> Self {
        Self { class: None, id: None, style: None }
    }
    pub(crate) fn render_to(self, buf: &mut String) {
        let Self { class, id, style } = self;
        if let Some(c) = class {  
            " class=".render_to(buf);
            c.render_quoted_to(buf)
        }
        if let Some(i) = id {
            " id=".render_to(buf);
            i.render_quoted_to(buf)
        }
        if let Some(s) = style {
            " style=".render_to(buf);
            s.render_quoted_to(buf)
        }
    }
}

dev_macros::define_tags! {
    html @children [lang];
    head [];
    link [as_, corsorigin, href, hreflang, imagesize, imagesrcset, media, rel, title(alternative_stylesheet), type_];
    meta [charset, content, http_equiv, name];
    title @children [];
    style @children [media, nonce, title(alternative_stylesheet)];
    a @global @children [href, download, target, rel];
    p @global @children [];
    span @global @children [];
    div @global @children [];
    header @global @children [];
    body @global @children [];
    h1 @global @children [];
    h2 @global @children [];
    h3 @global @children [];
    h4 @global @children [];
    h5 @global @children [];
    h6 @global @children [];
}
