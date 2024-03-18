#![doc = include_str!("../docs/state_machine.md")]

extern crate proc_macro;
use crate::custom_parse::StateMachine;
use code_gen::{gen_event_enum, gen_event_struct, get_state_enum};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod code_gen;
mod custom_parse;
mod custom_token;

/// This is procedure macro to generate state machine.
#[proc_macro]
pub fn state_mac(input_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_stream as StateMachine);
    let stname = format_ident!("{}", input.name);
    let pun = input.context_fields;
    let mut tk = TokenStream::new();
    let mut out = quote!(
        struct #stname {
            #pun
        }
    );
    tk.extend(TokenStream::from(out));

    for evt in &input.event_list {
        let evt_struct = gen_event_struct(&evt);
        if let Some(evt_param) = evt_struct {
            out = quote!( #evt_param );
            tk.extend(TokenStream::from(out));
        }
    }

    let evt_enum = gen_event_enum(&input.event_list);
    out = quote!( #evt_enum );
    tk.extend(TokenStream::from(out));

    let st_enum = get_state_enum(&input.state_list);
    out = quote!(#st_enum);
    tk.extend(TokenStream::from(out));

    let st_default = format_ident!("{}", input.state_default);
    out = quote!(static STATE: SmacState = #st_default;);
    tk.extend(TokenStream::from(out));

    tk
}
