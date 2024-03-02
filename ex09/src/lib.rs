#![doc = include_str!("../docs/state_machine.md")]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr, Result, Token,
};

mod kw {
    syn::custom_keyword!(sm_name);
}

enum Item {
    StateMachine(ItemStateMachine),
}

struct ItemStateMachine {
    #[allow(dead_code)]
    sm_token: kw::sm_name,
    #[allow(dead_code)]
    eq_token: Token![=],
    sm_value: LitStr,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let look_ahead = input.lookahead1();

        if look_ahead.peek(kw::sm_name) {
            input.parse().map(Item::StateMachine)
        } else {
            Err(look_ahead.error())
        }
    }
}

impl Parse for ItemStateMachine {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ItemStateMachine {
            sm_token: input.parse::<kw::sm_name>()?,
            eq_token: input.parse()?,
            sm_value: input.parse()?,
        })
    }
}

/// This is procedure macro to generate state machine.
#[proc_macro]
pub fn state_mac(input_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_stream as Item);
    let Item::StateMachine(smac) = input;

    let smac_name = smac.sm_value.value();
    let output = quote!( println!("State Machine name {} ", #smac_name ); );
    TokenStream::from(output)
}
