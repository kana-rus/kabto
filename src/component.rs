use std::fs::File;

pub trait Component {
    fn render(self) -> impl IntoHTML;
}
pub trait IntoHTML {
    fn into(self) -> HTML;
}
pub struct HTML(
    pub(crate) String
);

const _: (/* HTML impls */) = {
    impl HTML {
        pub fn build(self) -> Result<(), String> {


            let dist_file =
                if let Ok(file) = File::open() {

                };
            
            // self.0

            Ok(())
        }
    }

    impl HTML {
        fn 
    }

};
