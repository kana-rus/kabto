use std::format as f;
use crate::{Component, HTML, tag::*};

macro_rules! assert_component {
    ($component:expr, $expected:literal) => {
        {
            let mut rendered_html = String::with_capacity($expected.len());
            $component.render().render_to(&mut rendered_html);

            let mut expected_html = String::with_capacity($expected.len());
            for line in $expected.lines() {
                expected_html.push_str(line.trim())
            }
        
            assert_eq!(rendered_html, expected_html)
        }
    };
}


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

#[test] fn user_info() {
    let user_info = UserInfo {
        name:         f!("kanarus"),
        age:          20,
        homepage_url: f!("https://kana-rus.github.io/blog"),
    };

    assert_component!(user_info, r#"
        <div class="info-box" id="user-info-box">
            <h1 class="user-name-display">kanarus</h1>
            <div class="user-detail-info">
                <div>age:&#32;20</div>
                <div>homepage:&#32;<a href="https://kana-rus.github.io/blog">https://kana-rus.github.io/blog</a></div>
            </div>
        </div>
    "#)
}
