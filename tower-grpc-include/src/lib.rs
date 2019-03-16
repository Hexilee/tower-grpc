extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemMod};

#[proc_macro_attribute]
pub fn proto_include(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemMod);
    proto_include_impl::implement(attr_args, input).into()
}

mod proto_include_impl;
