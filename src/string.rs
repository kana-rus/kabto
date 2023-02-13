use crate::internal_macros::html_escape;

pub(crate) fn escaped(mut text: String) -> String {
    html_escape!(text by [
        '\t': "Tab"
        '\n': "NewLine"
        ' ': "nbsp"
        '"': "quot"
        '&': "amp"
        '<': "lt"
        '>': "gt"
    ]);
    text
}
