use std::marker::Tuple;
use crate::{library::{Cows, IntoCows}, component::{NodeCollection, IntoNode}};
use super::{components::{AnkerTarget, AnkerRel}, Node, Element, BaseAttributes};


pub(crate) enum Tag {
    html(html),
    head(head),
    link(link),
    meta(meta),
    style(style),
    body(body),

    a(a),
    p(p),
    span(span),

    div(div),
    header(header),
    h1(h1),
    h2(h2),
    h3(h3),
    h4(h4),
} impl Tag {
    pub(crate) fn render_with_children(self, children: Vec<Node>, buf: &mut String) {
        match self {
            Self::html(html) => {
                html.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</html>".render_to(buf)
            }
            Self::head(head) => {
                head.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</head>".render_to(buf)
            }
            Self::link(link) => {
                link.render_self_closing_to(buf);
            }
            Self::meta(meta) => {
                meta.render_self_closing_to(buf);
            }
            Self::style(style) => {
                style.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</style>".render_to(buf)
            }
            Self::body(body) => {
                body.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</body>".render_to(buf)
            }

            Self::a(a) => {
                a.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</a>".render_to(buf)
            }
            Self::p(p) => {
                p.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</p>".render_to(buf)
            }
            Self::span(span) => {
                span.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</span>".render_to(buf)
            }

            Self::div(div) => {
                div.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</div>".render_to(buf)
            }
            Self::header(header) => {
                header.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</header>".render_to(buf)
            }
            Self::h1(h1) => {
                h1.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</h1>".render_to(buf)
            }
            Self::h2(h2) => {
                h2.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</h2>".render_to(buf)
            }
            Self::h3(h3) => {
                h3.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</h3>".render_to(buf)
            }
            Self::h4(h4) => {
                h4.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</h4>".render_to(buf)
            }
        }
    }
}


pub struct html {
    lang: Option<Cows>
} impl html {
    pub fn lang(mut self, lang: impl IntoCows) -> Self {
        self.lang.replace(lang.into_cows());
        self
    }
} impl html {
    pub(crate) fn new() -> Self {
        Self { lang: None }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { lang } = self;
        "<html".render_to(buf);

        if let Some(lang) = lang {
            " lang=".render_to(buf);
            lang.render_quoted_to(buf)
        }

        buf.push('>')
    }
} impl IntoNode for html {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::html(self),
            children: vec![],
        })
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for html {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::html(self),
            children: children.collect(),
        })
    }
}

pub struct head {}
impl head {
    pub(crate) fn new() -> Self {
        Self {}
    }
    fn render_opening_to(self, buf: &mut String) {
        "<head>".render_to(buf)
    }
}
impl IntoNode for head {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::head(self),
            children: vec![],
        })
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for head {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::head(self),
            children: children.collect(),
        })
    }
}

pub struct link {
    as_:         Option<Cows>,
    corsorigin:  Option<Cows>,
    href:        Option<Cows>,
    hreflang:    Option<Cows>,
    imagesizes:  Option<Cows>,
    imagesrcset: Option<Cows>,
    media:       Option<Cows>,
    rel:         Option<Cows>,
    title:       Option<Cows>,
    type_:       Option<Cows>,
} impl link {
    pub fn as_(mut self, as_: impl IntoCows) -> Self {
        self.as_.replace(as_.into_cows()); self
    }
    pub fn corsorigin(mut self, corsorigin: impl IntoCows) -> Self {
        self.corsorigin.replace(corsorigin.into_cows()); self
    }
    pub fn href(mut self, href: impl IntoCows) -> Self {
        self.href.replace(href.into_cows()); self
    }
    pub fn hreflang(mut self, hreflang: impl IntoCows) -> Self {
        self.hreflang.replace(hreflang.into_cows()); self
    }
    pub fn imagesizes(mut self, imagesizes: impl IntoCows) -> Self {
        self.imagesizes.replace(imagesizes.into_cows()); self
    }
    pub fn imagesrcset(mut self, imagesrcset: impl IntoCows) -> Self {
        self.imagesrcset.replace(imagesrcset.into_cows()); self
    }
    pub fn media(mut self, media: impl IntoCows) -> Self {
        self.media.replace(media.into_cows()); self
    }
    pub fn rel(mut self, rel: impl IntoCows) -> Self {
        self.rel.replace(rel.into_cows()); self
    }
    pub fn title(mut self, title: impl IntoCows) -> Self {
        self.title.replace(title.into_cows()); self
    }
    pub fn type_(mut self, type_: impl IntoCows) -> Self {
        self.type_.replace(type_.into_cows()); self
    }
} impl link {
    pub(crate) fn new() -> Self {
        Self { as_: None, corsorigin: None, href: None, hreflang: None, imagesizes: None, imagesrcset: None, media: None, rel: None, title: None, type_: None }
    }
    fn render_self_closing_to(self, buf: &mut String) {
        let Self { as_, corsorigin, href, hreflang, imagesizes, imagesrcset, media, rel, title, type_ } = self;
        "<link".render_to(buf);

        if let Some(as_) = as_ {
            " as=".render_to(buf); as_.render_quoted_to(buf)
        }
        if let Some(corsorigin) = corsorigin {
            " corsorigin=".render_to(buf); corsorigin.render_quoted_to(buf)
        }
        if let Some(href) = href {
            " href=".render_to(buf); href.render_quoted_to(buf)
        }
        if let Some(hreflang) = hreflang {
            " hreflang=".render_to(buf); hreflang.render_quoted_to(buf)
        }
        if let Some(imagesizes) = imagesizes {
            " imagesizes=".render_to(buf); imagesizes.render_quoted_to(buf)
        }
        if let Some(imagesrcset) = imagesrcset {
            " imagesrcset=".render_to(buf); imagesrcset.render_quoted_to(buf)
        }
        if let Some(media) = media {
            " media=".render_to(buf); media.render_quoted_to(buf)
        }
        if let Some(rel) = rel {
            " rel=".render_to(buf); rel.render_quoted_to(buf)
        }
        if let Some(title) = title {
            " title=".render_to(buf); title.render_quoted_to(buf)
        }
        if let Some(type_) = type_ {
            " type=".render_to(buf); type_.render_quoted_to(buf)
        }

        " />".render_to(buf)
    }
} impl IntoNode for link {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag:      Tag::link(self),
            children: vec![],
        })
    }
} // `link` DOESN'T implement `FnOnce<Children>`

pub struct meta {
    charset:    Option<Cows>,
    content:    Option<Cows>,
    http_equiv: Option<Cows>,
    name:       Option<Cows>,
} impl meta {
    pub fn charset(mut self, charset: impl IntoCows) -> Self {
        self.charset.replace(charset.into_cows()); self
    }
    pub fn content(mut self, content: impl IntoCows) -> Self {
        self.content.replace(content.into_cows()); self
    }
    pub fn http_equiv(mut self, http_equiv: impl IntoCows) -> Self {
        self.http_equiv.replace(http_equiv.into_cows()); self
    }
    pub fn name(mut self, name: impl IntoCows) -> Self {
        self.name.replace(name.into_cows()); self
    }
} impl meta {
    pub(crate) fn new() -> Self {
        Self { charset: None, content: None, http_equiv: None, name: None }
    }
    fn render_self_closing_to(self, buf: &mut String) {
        let Self { charset, content, http_equiv, name } = self;
        "<meta".render_to(buf);

        if let Some(charset) = charset {
            " charset=".render_to(buf); charset.render_quoted_to(buf)
        }
        if let Some(content) = content {
            " content=".render_to(buf); content.render_quoted_to(buf)
        }
        if let Some(http_equiv) = http_equiv {
            " http-equiv=".render_to(buf); http_equiv.render_quoted_to(buf)
        }
        if let Some(name) = name {
            " name=".render_to(buf); name.render_quoted_to(buf)
        }

        " />".render_to(buf)
    }
} impl IntoNode for meta {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag:      Tag::meta(self),
            children: vec![]
        })
    }
} // `link` DOESN'T implement `FnOnce<Children>`
 
pub struct style {
    media: Option<Cows>,
    nonce: Option<Cows>,
    title: Option<Cows>,
} impl style {
    pub fn media(mut self, media: impl IntoCows) -> Self {
        self.media.replace(media.into_cows());
        self
    }
    pub fn nonce(mut self, nonce: impl IntoCows) -> Self {
        self.nonce.replace(nonce.into_cows());
        self
    }
    pub fn title(mut self, title: impl IntoCows) -> Self {
        self.title.replace(title.into_cows());
        self
    }
} impl style {
    pub(crate) fn new() -> Self {
        Self { media: None, nonce: None, title: None }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { media, nonce, title } = self;
        "<style".render_to(buf);

        if let Some(media) = media {
            " media=".render_to(buf);
            media.render_quoted_to(buf);
        }
        if let Some(nonce) = nonce {
            " nonce=".render_to(buf);
            nonce.render_quoted_to(buf);
        }
        if let Some(title) = title {
            " title=".render_to(buf);
            title.render_quoted_to(buf);
        }

        buf.push('>')
    }
} impl IntoNode for style {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::style(self),
            children: vec![],
        })
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for style {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::style(self),
            children: children.collect(),
        })
    }
}


pub struct a {
    pub(crate) __base:   BaseAttributes,    
    pub(crate) href:     Option<Cows>,
    pub(crate) download: Option<Cows>,
    pub(crate) target:   Option<AnkerTarget>,
    pub(crate) rel:      Option<AnkerRel>,
} impl a {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl a {
    pub fn href(mut self, href: impl IntoCows) -> Self {
        self.href.replace(href.into_cows());
        self
    }
    pub fn download(mut self, download: impl IntoCows) -> Self {
        self.download.replace(download.into_cows());
        self
    }
    pub fn target(mut self, target: impl IntoCows) -> Self {
        self.target.replace(AnkerTarget::from_str(&target.into_cows()));
        self
    }
    pub fn rel(mut self, rel: impl IntoCows) -> Self {
        self.rel.replace(AnkerRel::from_str(&rel.into_cows()));
        self
    }
} impl a {
    pub(crate) fn new() -> Self {
        Self {
            __base:   BaseAttributes::new(),
            href:     None,
            download: None,
            target:   None,
            rel:      None,
        }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base, href, download, target, rel } = self;
        "<a".render_to(buf);
        __base.render_to(buf);

        if let Some(h) = href {
            " href=".render_to(buf);
            h.render_quoted_to(buf)
        }
        if let Some(d) = download {
            " download=".render_to(buf);
            d.render_quoted_to(buf)
        }
        if let Some(t) = target {
            " target=".render_to(buf);
            t.as_str().render_quoted_to(buf)
        }
        if let Some(r) = rel {
            " rel=".render_to(buf);
            r.as_str().render_quoted_to(buf)
        }

        buf.push('>')
    }
} impl IntoNode for a {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::a(self),
            children: vec![],
        })
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for a {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::a(self),
            children: children.collect(),
        })
    }
}

pub struct p {
    pub(crate) __base: BaseAttributes,
} impl p {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl p {
    pub(crate) fn new() -> Self {
        Self { __base: BaseAttributes::new() }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<p".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for p {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::p(self),
            children: children.collect(),
        })
    }
} impl IntoNode for p {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::p(self),
            children: vec![],
        })
    }
}

pub struct span {
    pub(crate) __base: BaseAttributes,
} impl span {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl span {
    pub(crate) fn new() -> Self {
        Self { __base: BaseAttributes::new() }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<span".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for span {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::span(self),
            children: children.collect(),
        })
    }
} impl IntoNode for span {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::span(self),
            children: vec![],
        })
    }
}



pub struct div {
    pub(crate) __base: BaseAttributes,
} impl div {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl div {
    pub(crate) fn new() -> Self {
        Self { __base: BaseAttributes::new() }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<div".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for div {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::div(self),
            children: children.collect(),
        })
    }
} impl IntoNode for div {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::div(self),
            children: vec![],
        })
    }
}

pub struct header {
    pub(crate) __base: BaseAttributes,
} impl header {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl header {
    pub(crate) fn new() -> Self {
        Self { __base: BaseAttributes::new() }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<header".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for header {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::header(self),
            children: children.collect(),
        })
    }
} impl IntoNode for header {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::header(self),
            children: vec![],
        })
    }
}

pub struct body {
    pub(crate) __base: BaseAttributes,
} impl body {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl body {
    pub(crate) fn new() -> Self {
        Self { __base: BaseAttributes::new() }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<body".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for body {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::body(self),
            children: children.collect(),
        })
    }
} impl IntoNode for body {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::body(self),
            children: vec![],
        })
    }
}

pub struct h1 {
    pub(crate) __base: BaseAttributes,
} impl h1 {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl h1 {
    pub(crate) fn new() -> Self {
        Self { __base: BaseAttributes::new() }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<h1".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for h1 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h1(self),
            children: children.collect(),
        })
    }
} impl IntoNode for h1 {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::h1(self),
            children: vec![],
        })
    }
}

pub struct h2 {
    pub(crate) __base: BaseAttributes,
} impl h2 {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl h2 {
    pub(crate) fn new() -> Self {
        Self { __base: BaseAttributes::new() }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<h2".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for h2 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h2(self),
            children: children.collect(),
        })
    }
} impl IntoNode for h2 {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::h2(self),
            children: vec![],
        })
    }
}

pub struct h3 {
    pub(crate) __base: BaseAttributes,
} impl h3 {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl h3 {
    pub(crate) fn new() -> Self {
        Self { __base: BaseAttributes::new() }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<h3".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for h3 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h3(self),
            children: children.collect(),
        })
    }
} impl IntoNode for h3 {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::h3(self),
            children: vec![],
        })
    }
}

pub struct h4 {
    pub(crate) __base: BaseAttributes,
} impl h4 {
    pub fn class(mut self, class: impl IntoCows) -> Self {
        self.__base.class.replace(class.into_cows());
        self
    }
    pub fn id(mut self, id: impl IntoCows) -> Self {
        self.__base.id.replace(id.into_cows());
        self
    }
    pub fn style(mut self, css: impl IntoCows) -> Self {
        self.__base.style.replace(css.into_cows());
        self
    }
} impl h4 {
    pub(crate) fn new() -> Self {
        Self { __base: BaseAttributes::new() }
    }
    fn render_opening_to(self, buf: &mut String) {
        let Self { __base } = self;
        "<h4".render_to(buf);
        __base.render_to(buf);
        buf.push('>')
    }
} impl<Children: NodeCollection + Tuple> FnOnce<Children> for h4 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h4(self),
            children: children.collect(),
        })
    }
} impl IntoNode for h4 {
    fn into_node(self) -> Node {
        Node::Element(Element {
            tag: Tag::h4(self),
            children: vec![],
        })
    }
}
