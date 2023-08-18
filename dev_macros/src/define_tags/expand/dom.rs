use quote::quote;
use proc_macro2::TokenStream;
use crate::define_tags::model::{Definition, Tag, NormalAttribute, BooleanAttribute};


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
        let Self { name, with_children, boolean_attributes, normal_attributes, .. } = self;
        let n_attributes = boolean_attributes.len() + normal_attributes.len();

        let mut attributes = Vec::with_capacity(n_attributes);
        for BooleanAttribute { name } in boolean_attributes {attributes.push(quote! {
            pub(crate) #name: boolean,
        })}
        for NormalAttribute { name, .. } in normal_attributes {attributes.push(quote! {
            pub(crate) #name: Option<Cows>,
        })}

        let mut mutations = Vec::with_capacity(n_attributes);
        for BooleanAttribute { name } in boolean_attributes {mutations.push(quote! {
            pub fn #name(mut self) -> Self {
                self.#name = true;
                self
            }
        })}
        for NormalAttribute { name, argument_name } in normal_attributes {attributes.push(quote! {
            pub fn #name(mut self, #argument_name: impl IntoCows) -> Self {
                self.#name.replace(#argument_name.into_cows());
                self
            }
        })}

        let mut constructer = TokenStream::new();
        for BooleanAttribute { name } in boolean_attributes {constructer.extend(quote! {
            #name: false,
        })}
        for NormalAttribute { name, .. } in normal_attributes {constructer.extend(quote! {
            #name: None,
        })}
        constructer = quote! {
            pub(crate) fn new() -> Self {
                #constructer
            }
        };

        let mut renderer = TokenStream::new();
        renderer.extend({
            let booleans = boolean_attributes.iter().map(|BooleanAttribute { name }| quote! { #name });
            let normals  = normal_attributes .iter().map(|NormalAttribute  { name, .. }| quote! { #name });
            quote!{
                let Self { #( #booleans, )* #( #normals, )* } = self;
            }
        });
        renderer.extend({
            let opening = format!("<{}", name);
            quote! {
                #opening.render_to(buf);
            }
        });
        renderer.extend({
            let render_booleans = boolean_attributes.iter().map(|BooleanAttribute { name }| {
                let key = format!(" {}", name.to_string().replace('_', "-"));
                quote! {
                    if #name {
                        #key.render_to(buf);
                    }
                }
            });
            let render_normals = normal_attributes.iter().map(|NormalAttribute { name, .. }| {
                let key = format!(" {}=", name.to_string().replace('_', "-"));
                quote! {
                    if let Some(value) = #name {
                        #key.render_to(buf);
                        value.render_quoted_to(buf);
                    }
                }
            });
            quote! {
                #( #render_booleans )*
                #( #render_normals  )*
            }
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

            impl #name {
                #constructer
                #renderer
            }

            impl #name {
                #( #mutations )*
            }

            #into_node
            #set_children
        }
    }
}
