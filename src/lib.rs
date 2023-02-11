#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]

#![allow(non_camel_case_types)]

pub mod html {
    pub struct HTML;

    pub trait Component {
        fn render(self) -> impl IntoHTML;
    }
    
    pub trait IntoHTML {
        fn into(self) -> HTML;
    }
}

pub mod tag { #![allow(non_camel_case_types, non_upper_case_globals)]
    use crate::html::HTML;

    mod tag {
        use crate::html::HTML;

        pub struct html {
            pub(crate) children: HTML
        }
        pub struct head {pub(crate) children: HTML}
        pub struct body {pub(crate) children: HTML}
        pub struct header {pub(crate) children: HTML}
        pub struct h1 {pub(crate) children: HTML}
        pub struct h2 {pub(crate) children: HTML}
        pub struct h3 {pub(crate) children: HTML}
        pub struct h4 {pub(crate) children: HTML}
        pub struct div {pub(crate) children: HTML}
        pub struct article {pub(crate) children: HTML}
        pub struct nav {pub(crate) children: HTML}
        pub struct svg {pub(crate) children: HTML}
        pub struct span {pub(crate) children: HTML}
        pub struct p {pub(crate) children: HTML}
    }

    pub const html:    tag::html = tag::html {children: HTML};
    pub const head:    tag::head = tag::head {children: HTML};
    pub const body:    tag::body = tag::body {children: HTML};
    pub const header:  tag::header = tag::header {children: HTML};
    pub const h1:      tag::h1 = tag::h1 {children: HTML};
    pub const h2:      tag::h2 = tag::h2 {children: HTML};
    pub const h3:      tag::h3 = tag::h3 {children: HTML};
    pub const h4:      tag::h4 = tag::h4 {children: HTML};
    pub const div:     tag::div = tag::div {children: HTML};
    pub const article: tag::article = tag::article {children: HTML};
    pub const nav:     tag::nav = tag::nav {children: HTML};
    pub const svg:     tag::svg = tag::svg {children: HTML};
    pub const span:    tag::span = tag::span {children: HTML};
    pub const p:       tag::p = tag::p {children: HTML};

}
