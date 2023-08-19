/* ref: https://developer.mozilla.org/en-US/docs/Web/HTML/Element */
dev_macros::define_tags! {
    /* main root */
    html @children [lang];

    /* document metadata */
    head            ;
    link            [as_, crossorigin, href, hreflang, imagesize, imagesrcset, media, rel, title(alternative_stylesheet), type_];
    meta            [charset, content, http_equiv, name];
    style @children [media, nonce, title(alternative_stylesheet)];
    title @children ;

    /* sectioning root */
    body @global @children;

    /* content sectioning */
    article @global @children;
    aside   @global @children;
    footer  @global @children;
    header  @global @children;
    h1      @global @children;
    h2      @global @children;
    h3      @global @children;
    h4      @global @children;
    h5      @global @children;
    h6      @global @children;
    main    @global @children;
    nav     @global @children;
    section @global @children;

    /* text content */
    blockquote @global @children;
    div        @global @children;
    li         @global @children;
    menu       @global @children;
    ol         @global @children;
    p          @global @children;
    pre        @global @children;
    ul         @global @children;

    /* inline text semantics */
    a      @global @children [download, href, hreflang, ping, referrerpolicy, rel, target, type_];
    code   @global @children;
    span   @global @children;
    strong @global @children;

    /* image and multimedia */
    audio @global @children (autoplay, loop_, muted) [controls, crossorigin, preload, src];
    img   @global @children (ismap)                  [alt, crossorigin, decoding, elementtiming, height, loading, referrerpolicy, sizes, src, srcset, width, usemap];
    video @global @children (autoplay, loop_, muted) [controls, crossorigin, height, playsinline, poster, preload, src, width];

    /* embedded content */
    iframe @global @children [allow, allowfullscreen, height, loading, name, referrerpolicy, sandbox, src, srcdoc, width];

    /* svg */
    svg    @global @children [height, viewBox, width, x, y];
    path                     [d, fill, stroke];
    circle                   [d, fill, stroke];

    /* scripting */
    canvas @global @children                 [height, width];
    script @global @children (async_, defer) [crossorigin, integrity, nomodule, nonce, referrerpolicy, src, type_];

    /* table content */
    caption  @global @children;
    col      @global           [span(number_of_columns)];
    colgroup @global @children [span(number_of_columns)];
    table    @global @children;
    tbody    @global @children;
    td       @global @children [colspan, headers, rowspan];
    tfroot   @global @children [abbr, colspan, headers, rowspan, scope];
    th       @global @children [abbr, colspan, headers, rowspan, scope];
    thread   @global @children;
    tr       @global @children;

    /* forms */
    button   @global @children (autofocus, disabled)                     [form(ancestor_form), formaction, formenctype, formmethod, formnovalidate, formtarget, name, popovertarget, popovertargetaction, type_, value];
    form     @global @children                                           [autocomplete, name, rel, action, enctype, method, novalidate, target];
    input    @global @children (autofocus, disabled, readonly, required) [accept, alt, autocomplete, capture, dirname, form(ancestor_form), formaction, formenctype, formmethod, formnovalidate, formtarget, height, inputmode, list, max, maxlength, min, minlength, name, pattern, placeholder, popovertarget, popovertargetaction, size, src, step, type_, value, width];
    label    @global @children                                           [for_];
    textarea @global @children                                           [autocomplete, cols, dirname, form(ancestor_form), maxlength, minlength, name, placeholder, rows, spellcheck, wrap];
}
