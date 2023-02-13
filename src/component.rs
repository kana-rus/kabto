use std::{fs::File, io::Write, ops::Sub};
use crate::{config::Config, Result};

pub trait Component {
    fn render(self) -> impl IntoHTML;
}
pub trait ParentCopmonent<Children: IntoHTML> = Sub<Children, Output = HTML>;

pub trait IntoHTML {
    fn into_html(self) -> HTML;
}
pub struct HTML(
    pub(crate) String
);

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
