use kabto::macros::css;

fn main() {
    let _manual_css = kabto::css::CSS {
        display: kabto::css::property::Display::flex,
        ..Default::default()
    };

}
