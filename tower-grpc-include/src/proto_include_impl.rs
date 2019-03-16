use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemMod};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    path: String,
}

pub fn implement(attr_args: AttributeArgs, input: ItemMod) -> TokenStream {
    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors();
        }
    };

    let mod_name = input.ident;
    let vis = input.vis;
    quote! {#vis mod #mod_name {}}
}
