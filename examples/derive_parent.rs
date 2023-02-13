use kabto::{html::HTML, component::Parent};

fn main() {
    #[derive(Parent)]
    struct Tag {
        children: HTML
    }
}

fn _expanded() {
    struct Tag {
        children: HTML,
    }
    const _: () = {
        impl<Children: kabto::html::IntoHTML> kabto::component::Parent<Children>
        for Tag {
            fn set_children(&mut self, children: Children) {
                self.children = children.into_html();
            }
        }
        impl<Children: kabto::html::IntoHTML> std::ops::Sub<Children> for Tag {
            type Output = Self;
            fn sub(mut self, children: Children) -> Self {
                self.set_children(children);
                self
            }
        }
    };
}
