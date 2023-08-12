use crate::{library::{Cows, IntoCows}, component::Children};
use super::{components::{AnkerTarget, AnkerRel}, Node, Element};


pub enum Tag {
    a(a),
    div(div),
    h1(h1),
} impl Tag {
    pub(crate) fn render_with_children(self, children: Vec<Node>, buf: &mut String) {
        match self {
            Self::a(a) => {
                a.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</a>".render_to(buf)
            }
            Self::div(div) => {
                div.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</div>".render_to(buf)
            }
            Self::h1(h1) => {
                h1.render_opening_to(buf);
                for c in children {c.render_to(buf)}
                "</h1>".render_to(buf)
            }
        }
    }
}

pub(crate) struct BaseAttributes {
    pub(crate) class: Option<Cows>,
    pub(crate) id:    Option<Cows>,
    pub(crate) style: Option<Cows>,
} impl BaseAttributes {
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
} impl BaseAttributes {
    pub(crate) fn class(mut self, class: impl IntoCows) -> Self {
        self.class.replace(class.into_cows());
        self
    }
    pub(crate) fn id(mut self, id: impl IntoCows) -> Self {
        self.id.replace(id.into_cows());
        self
    }
    pub(crate) fn style(mut self, style: impl IntoCows) -> Self {
        self.style.replace(style.into_cows());
        self
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
    pub fn style(mut self, style: impl IntoCows) -> Self {
        self.__base.style.replace(style.into_cows());
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
} impl<C: Children> FnOnce<C> for a {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: C) -> Self::Output {
        Node::Element(Element {
            tag: Tag::a(self),
            children: children.collect(),
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
    pub fn style(mut self, style: impl IntoCows) -> Self {
        self.__base.style.replace(style.into_cows());
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
} impl<C: Children> FnOnce<C> for div {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: C) -> Self::Output {
        Node::Element(Element {
            tag: Tag::div(self),
            children: children.collect(),
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
    pub fn style(mut self, style: impl IntoCows) -> Self {
        self.__base.style.replace(style.into_cows());
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
} impl<C: Children> FnOnce<C> for h1 {
    type Output = Node;
    extern "rust-call" fn call_once(self, children: C) -> Self::Output {
        Node::Element(Element {
            tag: Tag::h1(self),
            children: children.collect(),
        })
    }
}
