use quote::quote;
use proc_macro2::TokenStream;
use crate::define_tags::model::{Definition, Tag, BooleanAttribute, NormalAttribute};


impl Definition {
    pub(super) fn expand_for_component(&self) -> TokenStream {
        let Self { tags } = self;
        let definitions = tags.into_iter().map(Tag::expand_for_component);

        quote! {
            #( #definitions )*
        }
    }
}

impl Tag {
    fn expand_for_component(&self) -> TokenStream {
        let Self { name:tag_name, with_children, boolean_attributes, normal_attributes, .. } = self;

        let mut mutations = TokenStream::new();
        for BooleanAttribute { name } in boolean_attributes {
            mutations.extend(quote! {
                pub fn #name(mut self) -> super::dom::#tag_name {
                    super::dom::#tag_name::new().#name()
                }
            })
        }
        for NormalAttribute { name, argument_name } in normal_attributes {
            mutations.extend(quote! {
                pub fn #name(mut self, #argument_name: impl IntoCows) -> super::dom::#tag_name {
                    super::dom::#tag_name::new().#name(#argument_name)
                }
            })
        }

        let into_node = quote! {
            impl IntoNode for #tag_name {
                fn into_node(self) -> Node {
                    super::dom::#tag_name::new().into_node()
                }
            }
        };

        let set_children = with_children.then(|| quote! {
            impl<Children: NodeCollection + Tuple> FnOnce<Children> for #tag_name {
                type Output = Node;
                extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
                    Node::Element(Element {
                        tag: Tag::#tag_name(super::dom::#tag_name::new()),
                        children: children.collect(),
                    })
                }
            }
        });

        quote! {
            pub struct #tag_name;

            impl #tag_name {
                #mutations
            }

            #into_node
            #set_children
        }
    }
}
