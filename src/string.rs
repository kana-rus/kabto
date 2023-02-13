use kabto_macros::html_escape;

pub(crate) fn escaped(mut text: String) -> String {
    html_escape!{text by [
        '\t': 9
        '\n': 10
        ' ': 32
        '!': 33
        '"': 34
        
    ]}
    text
}
