mod tag;
mod html_macro;

use std::{fs::File, io::Write, ops::Sub};
use crate::{config::Config, Result};

pub struct HTML(
    pub(crate) String
);
pub trait IntoHTML {
    fn into_html(self) -> HTML;
}

pub use crate::macros::Parent;
pub trait Parent<Children: IntoHTML>: Sub<Children, Output = Self> {
    fn set_children(&mut self, children: Children);
}

pub(crate) fn escaped(mut text: String) -> String {
    crate::internal_macros::html_escape!(text by [
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


const _: (/* HTML impls */) = {
    impl HTML {
        pub fn build(self) -> Result<()> {
            let config = Config::get()?;

            let out_file_path = format!("{}/index.html", config.build.out_dir);
            let mut dist_file =
                if let Ok(file) = File::open(&out_file_path) {
                    file
                } else {
                    File::create(&out_file_path)?
                };
            
            dist_file.write_all(self.0.as_bytes())?;
            Ok(())
        }
    }
};
