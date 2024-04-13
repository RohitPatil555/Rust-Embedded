#![doc = include_str!("../docs/state_machine.md")]

extern crate proc_macro;
use crate::custom_parse::StateMachine;
use code_gen::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,Stmt, parse_quote};

mod code_gen;
mod custom_parse;
mod custom_token;

/// This is procedure macro to generate state machine.
#[proc_macro]
pub fn state_mac(input_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_stream as StateMachine);
    let stname = input.name;
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

    let st_default = input.state_default.unwrap();
    out = quote!(static STATE: SmacState = SmacState::#st_default;);
    tk.extend(TokenStream::from(out));

    for pse in input.proc_list {
        let proc_func = get_proc_function(&pse);
        out = quote!( #proc_func );
        tk.extend(TokenStream::from(out));
    }

    for _st in &input.state_list {
        let st_proc_func = get_state_function(&input.event_list, _st);
        out = quote!( #st_proc_func );
        tk.extend(TokenStream::from(out));
    }

    let global_state_define: Stmt = parse_quote!{
        static mut G_STATE : SmacState = SmacState::#st_default;
    };
    out = quote!( #global_state_define );
    tk.extend(TokenStream::from(out));

    let st_intf_func = get_interface_function(&input.state_list, stname.as_ref().unwrap());
    out = quote!( #st_intf_func );
    tk.extend(TokenStream::from(out));

    tk
}
