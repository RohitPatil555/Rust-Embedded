#![doc = include_str!("../docs/state_machine.md")]

extern crate proc_macro;
use crate::custom_parse::StateMachine;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod code_gen;
mod custom_parse;
mod custom_token;

/// This is procedure macro to generate state machine.
#[proc_macro]
pub fn state_mac(input_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_stream as StateMachine);
    let mut tk = TokenStream::new();

    let event_enum = input.create_event_enum();
    let state_enum = input.create_state_enum();
    let state_enum_default = input.create_state_default();

    let smac_struct = input.create_smac_struct();
    let smac_impl = input.create_smac_impl();

    let out = quote!(
        #event_enum

        #state_enum

        #state_enum_default

        #smac_struct

        #smac_impl
    );

    tk.extend(TokenStream::from(out));
    tk
}
