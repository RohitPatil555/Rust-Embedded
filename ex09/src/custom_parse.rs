use crate::custom_token::{context, default, event, sm_name, state};
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
    pub event_list: Vec<SmacEvent>,
    pub state_list: Vec<String>,
    pub state_default: String,
}

#[derive(Default)]
pub(crate) struct SmacEvent {
    pub name: String,
    pub evt_fields: Punctuated<Field, Token![,]>,
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

        while input.peek(event) {
            let evt = input.parse::<SmacEvent>()?;
            smac.event_list.push(evt);
        }

        while input.peek(state) {
            let _ = input.parse::<state>()?;
            let state_name: LitStr = input.parse()?;
            smac.state_list.push(state_name.value());
        }

        if input.peek(default) {
            let _ = input.parse::<default>()?;
            let state_name: LitStr = input.parse()?;

            smac.state_default = state_name.value();
        }

        Ok(smac)
    }
}

impl Parse for SmacEvent {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut evt: SmacEvent = SmacEvent::default();

        if input.peek(event) {
            let _ = input.parse::<event>()?;
            let name: LitStr = input.parse()?;
            let content;
            let _ = braced!(content in input);

            evt.name = name.value();
            evt.evt_fields = content.parse_terminated(Field::parse_named, Token![,])?;
        }

        Ok(evt)
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

    #[test]
    fn parse_event_1() {
        let input = "event \"TestEvent\" { dd: u8, }";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for evt in &smac.event_list {
            println!("Event name : {}", evt.name);
            assert_eq!("TestEvent", evt.name);
        }

        assert_eq!(1, smac.event_list.len(), "Single Event not found");
    }

    #[test]
    fn parse_event_2() {
        let input = "event \"TestEvent1\" { dd: u8, } 
                    event \"TestEvent2\" { tt: u8, }";
        let mut idx: u8 = 1;

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for evt in &smac.event_list {
            println!("Event name : {}", evt.name);
            assert_eq!(format!("TestEvent{}", idx), evt.name);
            idx += 1;
        }

        assert_eq!(2, smac.event_list.len(), "Single Event not found");
    }

    #[test]
    fn parse_state_1() {
        let input = "state \"TestState1\"";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for name in &smac.state_list {
            assert_eq!("TestState1", name);
        }

        assert_eq!(1, smac.state_list.len());
    }

    #[test]
    fn parse_state_2() {
        let input = "state \"TestState1\"
                     state \"TestState2\"";
        let mut idx: u8 = 1;

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for name in &smac.state_list {
            assert_eq!(format!("TestState{}", idx), *name);
            idx += 1;
        }

        assert_eq!(2, smac.state_list.len());
    }

    #[test]
    fn parse_default() {
        let input = "default \"TestStateDef\"";
        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        assert_eq!("TestStateDef", smac.state_default);
    }
}
