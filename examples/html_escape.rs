use kabto_macros::html_escape;

fn main() {
    let mut text = String::from("");
    html_escape!{text by [
        '\t': 9
        '\n': 10
    ]}
}

fn _expanded() {
    let mut text = String::from("");
    let mut pos = 0;
    for ch in &mut text.clone().chars() {
        match ch {
            '\t' => {
                text.replace_range(pos..=pos, "&#9;");
                pos += 4usize;
            }
            '\n' => {
                text.replace_range(pos..=pos, "&#10;");
                pos += 5usize;
            }
            _ => {}
        }
    }
}
