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

/* ref: https://developer.mozilla.org/en-US/docs/Web/HTML/Element */
dev_macros::define_tags! {
    // main root
    html @children [lang];

    // document metadata
    head [];
    link [as_, corsorigin, href, hreflang, imagesize, imagesrcset, media, rel, title(alternative_stylesheet), type_];
    meta [charset, content, http_equiv, name];
    style @children [media, nonce, title(alternative_stylesheet)];
    title @children [];

    // sectioning root
    body @global @children [];

    // content sectioning
    article @global @children [];
    aside @global @children [];
    footer @global @children [];
    header @global @children [];
    h1 @global @children [];
    h2 @global @children [];
    h3 @global @children [];
    h4 @global @children [];
    h5 @global @children [];
    h6 @global @children [];
    main @global @children [];
    nav @global @children [];
    section @global @children [];

    // text content
    blockquote @global @children [];
    div @global @children [];
    li @global @children [];
    menu @global @children [];
    ol @global @children [];
    p @global @children [];
    pre @global @children [];
    ul @global @children [];

    // inline text semantics
    a @global @children [href, download, target, rel];
    code @global @children [];
    span @global @children [];
    strong @global @children [];

    // image and multimedia
    audio @global @children [autoplay, controls, crossorigin, loop_, muted, preload, src];
    img @global @children [alt, crossorigin, decoding, elementtiming, height, ismap, loading, referrerpolicy, sizes, src, srcset, width, usemap];
    video @global @children [autoplay, controls, crossorigin, height, loop_, muted, playsinline, poster, preload, src, width];

    // embedded content
    iframe @global @children [allow, allowfullscreen, height, loading, name, referrerpolicy, sandbox, src, srcdoc, width];

    // svg
    svg @global @children [height, viewBox, width, x, y];
    path [d, fill, stroke];

    // scripting
    canvas @global @children [];
    script @global @children [crossorigin, integrity, nomodule, nonce, referrerpolicy, src, type_];

    // forms
    button @global @children [form(ancestor_form), formaction, formenctype, formmethod, formnovalidate, formtarget, name, popovertarget, popovertargetaction, type_, value];
    form @global @children [autocomplete, name, rel, action, enctype, method, novalidate, target];
    input @global @children [accept, alt, autocomplete, capture, dirname, form(ancestor_form), formaction, formenctype, formmethod, formnovalidate, formtarget, height, inputmode, list, max, maxlength, min, minlength, name, pattern, placeholder, popovertarget, popovertargetaction, size, src, step, type_, value, width];
    label @global @children [for_];
    textarea @global @children [autocomplete, cols, dirname, form(ancestor_form), maxlength, minlength, name, placeholder, rows, spellcheck, wrap];
}
