use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use crate::define_tags::model::{Definition, Tag, Attribute, GlobalAttributes};


impl Definition {
    pub(super) fn expand_for_dom(&self) -> TokenStream {
        let Self { tags } = self;

        let definitions = tags.into_iter().map(Tag::expand_for_dom);

        let tag_declarations = tags.iter().map(|Tag { name, .. }| quote! {
            #name(#name),
        });

        let renderer = {
            let arms = tags.iter().map(|Tag { name, with_children, .. }| {
                if *with_children {quote! {
                    Self::#name(#name) => #name.render_with_children_to(children, buf),
                }} else {quote! {
                    Self::#name(#name) => #name.render_self_closing_to(buf),
                }}
            });

            quote! {
                pub(crate) fn render_with_children(self, children: Vec<Node>, buf: &mut String) {
                    match self {
                        #( #arms )*
                    }
                }
            }
        };

        quote! {
            pub(crate) enum Tag {
                #( #tag_declarations )*
            } impl Tag {
                #renderer
            }

            #( #definitions )*
        }
    }
}

impl Tag {
    fn expand_for_dom(&self) -> TokenStream {
        let Self { name, with_global, with_children, own_attributes } = self;

        let mut attributes = own_attributes.iter().map(|Attribute { name, .. }| quote! {
            pub(crate) #name: Option<Cows>,
        }).collect::<Vec<_>>(); if *with_global {attributes.push(quote! {
            pub(crate) __global: GlobalAttributes,
        })}

        let mut methods = own_attributes.iter().map(|Attribute { name, argument_name }| quote! [
                    pub fn #name(mut self, #argument_name: impl IntoCows) -> Self {
                        self.#name.replace(#argument_name.into_cows());
                        self
                    }
        ]).collect::<Vec<_>>(); if *with_global {
            methods.append(&mut GlobalAttributes().iter().map(|Attribute { name, argument_name }| {
                let name = format_ident!("{name}"); quote! [
                    pub fn #name(mut self, #argument_name: impl IntoCows) -> Self {
                        self.__global.#name.replace(#argument_name.into_cows());
                        self
                    }
                ]
            }).collect::<Vec<_>>())
        }

        let mut new_attributes = own_attributes.iter().map(|Attribute { name, .. }| quote!{
            #name: None,
        }).collect::<Vec<_>>(); if *with_global {new_attributes.push(quote!{
            __global: GlobalAttributes::new(),
        })}

        let mut renderer = TokenStream::new();
        renderer.extend({
            let mut attributes = own_attributes.iter().map(|Attribute { name, .. }| quote!{
                #name,
            }).collect::<Vec<_>>(); if *with_global {attributes.push(quote! {
                __global
            })}
            quote!{
                let Self { #( #attributes )* } = self;
            }
        });
        renderer.extend({
            let opening = format!("<{}", name);
            quote! {
                #opening.render_to(buf);
            }
        });
        renderer.extend({
            with_global.then_some(quote! {
                __global.render_to(buf);
            })
        });
        renderer.extend({
            let render_attrs = own_attributes.iter().map(|Attribute { name, .. }| {
                let key = format!(" {}=", name.to_string().replace('_', "-"));
                quote! {
                    if let Some(value) = #name {
                        #key.render_to(buf);
                        value.render_quoted_to(buf);
                    }
                }
            });
            render_attrs
        });
        if *with_children {
            let closing = format!("</{}>", name);

            renderer.extend(quote! {
                buf.push('>');

                for c in children {
                    c.render_to(buf)
                }
        
                #closing.render_to(buf)
            });

            renderer = quote! {
                fn render_with_children_to(self, children: Vec<Node>, buf: &mut String) {
                    #renderer
                }
            }

        } else {
            renderer.extend(quote! {
                " />".render_to(buf)
            });

            renderer = quote! {
                fn render_self_closing_to(self, buf: &mut String) {
                    #renderer
                }
            }
        }

        let into_node = quote! {
            impl IntoNode for #name {
                fn into_node(self) -> Node {
                    Node::Element(Element {
                        tag: Tag::#name(self),
                        children: vec![]
                    })
                }
            }
        };
        let set_children = with_children.then(|| quote! {
            impl<Children: NodeCollection + Tuple> FnOnce<Children> for #name {
                type Output = Node;
                extern "rust-call" fn call_once(self, children: Children) -> Self::Output {
                    Node::Element(Element {
                        tag: Tag::#name(self),
                        children: children.collect(),
                    })
                }
            }
        });

        quote! {
            pub struct #name {
                #( #attributes )*
            }

            #into_node
            #set_children
            
            impl #name {
                #( #methods )*
            }
            
            impl #name {
                pub(crate) fn new() -> Self {
                    Self {
                        #( #new_attributes )*
                    }
                }
                #renderer
            }
        }
    }
}
