pub mod property;

pub struct CSS {
    pub align_items:       property::align_items,
    pub back_ground_color: Option<&'static str>,
    pub display:           property::display,
}
impl Default for CSS {
    fn default() -> Self {
        Self {
            align_items:       property::align_items::default(),
            back_ground_color: None,
            display:           property::display::default(),
        }
    }
}

fn _sample() {
    let css = CSS {
        align_items: crate::css::property::align_items::center,
        display: crate::css::property::display::block,
        ..Default::default()
    };
}

// macro_rules! css {
//     { $key1:ident: $value1:expr; } => {
//         crate::css::CSS {
//             css!{ $key1 $value1 }
//             ..Default::default()
//         }
//     };

//     (display $value:expr) => {
//         display: crate::css::property::display::$value,
//     };
//     (align_items $value:expr) => {
//         align_items: crate::css::property::align_items::$value,
//     };
// }

macro_rules! properties {
    { display: $display:ident; $( $key:ident : $value:tt ; )* } => {
        display: crate::css::property::display::$display,
        properties!{ $( $key: $value; )* }
    };
    { align_items $align_items:ident; $( $key:ident : $value:tt ; )* } => {
        align_items: crate::css::property::align_items::$align_items,
        properties!{ $( $key: $value; )* }
    };
}
macro_rules! _css {
    ( { display: $display:ident; $( $key:ident : $value:ident ; )* } ) => {
        // {
        //     #[allow(unused_mut)]
        //     let mut css = crate::css::CSS::default();
        //     _css!{ $( $key: $value; )* }
        //     css
        // }
        {
            crate::css::CSS {
                display: crate::css::property::display::$display,
                $(
                    $key: crate::css::property::$key::$value,
                )*
                ..Default::default()
            }
        }
    };
    {display: $display:ident; $( $key:ident: $value:ident; )*} => {
        css.display = crate::css::property::display::$display;
        _css!{ $( $key: $value; )* }
    };
    {align_items: $align_items:ident; $( $key:ident: $value:ident; )*} => {
        css.align_items = crate::css::property::align_items::$align_items;
        _css!{ $( $key: $value; )* }
    };
    {} => {};
}
fn _f() {
    let c =_css!({
        display: inherit;
    });
}

macro_rules! css {
    { $(display: $value:ident;)? $(align_items: $value2:ident;)? } => {
        crate::css::CSS {
            $(align_items: crate::css::property::align_items::$value2,)?
            $(display: crate::css::property::display::$value,)?
            ..Default::default()
        }
    };
}

fn _sample2() {
    let c = css!{
        display: inline;
        align_items: center;
    };
}
