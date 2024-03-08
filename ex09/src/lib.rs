#![doc = include_str!("../docs/state_machine.md")]

extern crate proc_macro;
use crate::custom_parse::StateMachine;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod custom_parse;
mod custom_token;

/// This is procedure macro to generate state machine.
#[proc_macro]
pub fn state_mac(input_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_stream as StateMachine);
    let stname = format_ident!("{}", input.name);
    let pun = input.context_fields;
    let out = quote!(
        struct #stname {
            #pun
        }
    );

    TokenStream::from(out)
}
