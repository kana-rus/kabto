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
            mod dom {
                #expanded_for_dom
            }

            mod component {
                #expanded_for_component
            }
        }
    }
}
