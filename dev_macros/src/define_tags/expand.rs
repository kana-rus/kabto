mod dom;
mod component;

use quote::quote;
use proc_macro2::TokenStream;
use super::model::Tags;


impl Tags {
    pub(super) fn expand(self) -> TokenStream {
        let expanded_for_dom = self.expand_for_dom();
        let expanded_for_component = self.expand_for_component();

        quote! {
            pub(crate) mod dom {
                use std::marker::Tuple;
                use super::{GlobalAttributes};
                use crate::{Element, Node, IntoNode};
                use crate::_library::{NodeCollection, Cows, IntoCows};

                #expanded_for_dom
            }

            pub(crate) mod component {
                use std::marker::Tuple;
                use crate::{Element, Node, IntoNode, Tag};
                use crate::_library::{NodeCollection, Cows, IntoCows};

                #expanded_for_component
            }
        }
    }
}
