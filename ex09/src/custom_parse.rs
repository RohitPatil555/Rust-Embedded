use crate::custom_token::{context, sm_name};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Field, LitStr, Result, Token,
};

#[derive(Default)]
pub(crate) struct StateMachine {
    pub name: String,
    pub context_fields: Punctuated<Field, Token![,]>,
}

impl Parse for StateMachine {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut smac: StateMachine = StateMachine::default();

        if input.peek(sm_name) {
            let _ = input.parse::<sm_name>()?;
            let _: Token![=] = input.parse()?;
            let sm_name_val: LitStr = input.parse()?;

            smac.name = sm_name_val.value();
        }

        if input.peek(context) {
            let _ = input.parse::<context>()?;
            let content;
            let _ = braced!(content in input);

            smac.context_fields = content.parse_terminated(Field::parse_named, Token![,])?;
        }

        Ok(smac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sm_name() {
        let input = "sm_name = \"testName\"";
        let smac = syn::parse_str::<StateMachine>(input).unwrap();
        assert_eq!("testName", smac.name);
    }

    #[test]
    fn parse_smac_context() {
        let input = "context { dd: u8 }";
        let smac = syn::parse_str::<StateMachine>(input).unwrap();
        for field in smac.context_fields {
            println!("{:?}", field);
        }
    }

    #[test]
    fn parse_smac() {
        let input = "sm_name = \"test\" 
        context {
            dd: u8,
        }";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        println!("State Machine Name : {} ", smac.name);
        for field in smac.context_fields {
            println!("{:?}", field);
        }
    }
}
