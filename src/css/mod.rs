pub mod property;

pub struct CSS {
    pub align_items:       property::AlignItems,
    pub back_ground_color: Option<&'static str>,
    pub display:           property::Display,
}
impl Default for CSS {
    fn default() -> Self {
        Self {
            align_items:       property::AlignItems::default(),
            back_ground_color: None,
            display:           property::Display::default(),
        }
    }
}

fn _sample() {
    let css = CSS {
        align_items: crate::css::property::AlignItems::center,
        display: crate::css::property::Display::block,
        ..Default::default()
    };
}
