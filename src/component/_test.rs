use crate::{Component, HTML, a, div, h1};

struct UserInfo {
    name:         String,
    age:          usize,
    homepage_url: String,
} impl Component for UserInfo {
    fn render(self) -> impl HTML {
        let Self { name, age, homepage_url } = self;

        div.id("user-info-box").class("info-box")(
            h1.class("user-name-display")(
                name
            ),
            div.class("user-detail-info")(
                div(
                    "age: ", age
                ),
                div(
                    "homepage: ", a.href(homepage_url.clone())(homepage_url),
                )
            )
        )
    }
}
