use kabto_macros::html_escape;

fn main() {
    let mut text = String::from("");
    html_escape!{text by [
        '\t': "Tab"
        '\n': "NewLine"
        ' ': "nbsp"
        '"': "quot"
        '&': "amp"
        '<': "lt"
        '>': "gt"
    ]}
}

fn _expanded() {
    let mut text = String::from("");
    let mut pos = 0;
    for ch in &mut text.clone().chars() {
        match ch {
            '\t' => {
                text.replace_range(pos..=pos, "&Tab;");
                pos += 5usize;
            }
            '\n' => {
                text.replace_range(pos..=pos, "&NewLine;");
                pos += 9usize;
            }
            ' ' => {
                text.replace_range(pos..=pos, "&nbsp;");
                pos += 6usize;
            }
            '"' => {
                text.replace_range(pos..=pos, "&quot;");
                pos += 6usize;
            }
            '&' => {
                text.replace_range(pos..=pos, "&amp;");
                pos += 5usize;
            }
            '<' => {
                text.replace_range(pos..=pos, "&lt;");
                pos += 4usize;
            }
            '>' => {
                text.replace_range(pos..=pos, "&gt;");
                pos += 4usize;
            }
            _ => pos += 1,
        }
    }
}
