// fn main() {
//     let _manual_css = kabto::css::CSS {
//         display: kabto::css::property::display::flex,
//         ..Default::default()
//     };
//     
//     macro_rules! display_sytle {
//         { display: $value:ident ; } => {
//             kabto::css::property::display::$value
//         };
//     }
//     let d = display_sytle!{
//         display: none;
//     };
// }
// 

use kabto_macros::css;

fn main() {
    let c = css!{
        display: flex;
    };
}
