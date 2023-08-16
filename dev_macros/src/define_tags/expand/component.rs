use quote::quote;
use proc_macro2::TokenStream;
use crate::define_tags::model::{Tags, Tag, Attribute, GlobalAttributes};


impl Tags {
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
        let Self { name:tag_name, with_global, with_children, own_attributes } = self;

        let mut methods = own_attributes.iter().map(|Attribute { name, argument_name }| quote! {
                pub fn #name(self, #argument_name: impl IntoCows) -> super::dom::#tag_name {
                    super::dom::#tag_name::new().#name(#argument_name)
                }
        }).collect::<Vec<_>>(); if *with_global {
            methods.append(&mut GlobalAttributes().iter().map(|Attribute { name, argument_name }| quote! {
                pub fn #name(self, #argument_name: impl IntoCows) -> super::dom::#tag_name {
                    super::dom::#tag_name::new().#name(#argument_name)
                }
            }).collect::<Vec<_>>())
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
            #into_node
            #set_children
            impl #tag_name {
                #( #methods )*
            }
        }
    }
}
