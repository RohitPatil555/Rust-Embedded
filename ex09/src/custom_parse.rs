use crate::custom_token::{context, default, event, proc, sm_name, state};
use proc_macro2::Span;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated, Block,
    Field, Result, Stmt, Token, Ident, Error
};

#[derive(Default)]
pub(crate) struct StateMachine {
    pub name: Option<Ident>,
    pub context_fields: Punctuated<Field, Token![,]>,
    pub event_list: Vec<SmacEvent>,
    pub state_list: Vec<Ident>,
    pub state_default: Option<Ident>,
    pub proc_list: Vec<SmacEventProc>,
}

#[derive(Default)]
pub(crate) struct SmacEventProc {
    pub state_name: Option<Ident>,
    pub event_name: Option<Ident>,
    pub event_proc_list: Vec<Stmt>,
    state_span: Option<Span>,
    event_span: Option<Span>,
}

#[derive(Default,PartialEq)]
pub(crate) struct SmacEvent {
    pub name: Option<Ident>,
    pub evt_fields: Punctuated<Field, Token![,]>,
}

impl Parse for StateMachine {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut smac: StateMachine = StateMachine::default();

        if input.peek(sm_name) {
            let _ = input.parse::<sm_name>()?;
            let _: Token![=] = input.parse()?;
            let st_name : Ident = input.parse()?;

            smac.name = Some(st_name);
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
            let state_name: Ident = input.parse()?;
            smac.state_list.push(state_name);
        }

        if input.peek(default) {
            let _ = input.parse::<default>()?;
            let span = input.span();
            let state_name: Ident = input.parse()?;

            if smac.state_list.is_empty() {
                return Err(Error::new(span, "States are not define before this lines."));
            }

            if smac.state_list.iter().find(|&state| state.to_string() == state_name.to_string()) == None {
                return Err(Error::new(span, "State not defined"));
            }

            smac.state_default = Some(state_name);
        }

        while input.peek(proc) {
            let _ = input.parse::<proc>()?;
            let proc_event: SmacEventProc = input.parse::<SmacEventProc>()?;

            if smac.state_list.iter().find(|&st| st.to_string() == proc_event.state_name.as_ref().unwrap().to_string()) == None {
                return Err(Error::new(proc_event.state_span.unwrap(), "State not defined or wrong"));
            }

            if smac.event_list.iter().find(|&evt| evt.name.as_ref().unwrap().to_string() == proc_event.event_name.as_ref().unwrap().to_string()) == None {
                return Err(Error::new(proc_event.event_span.unwrap(), "Event not defined or wrong"));
            }

            smac.proc_list.push(proc_event);
        }

        Ok(smac)
    }
}

impl Parse for SmacEvent {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut evt: SmacEvent = SmacEvent::default();

        if input.peek(event) {
            let _ = input.parse::<event>()?;
            let name: Ident = input.parse()?;
            let content;
            let _ = braced!(content in input);

            evt.name = Some(name);
            evt.evt_fields = content.parse_terminated(Field::parse_named, Token![,])?;
        }

        Ok(evt)
    }
}

impl Parse for SmacEventProc {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut event_proc: SmacEventProc = SmacEventProc::default();

        event_proc.state_span = Some(input.span());
        let state_name: Ident = input.parse()?;

        let _: Token![:] = input.parse()?;

        event_proc.event_span = Some(input.span());
        let event_name: Ident = input.parse()?;

        let content;
        let _ = braced!(content in input);

        event_proc.state_name = Some(state_name);
        event_proc.event_name = Some(event_name);
        event_proc.event_proc_list = content.call(Block::parse_within)?;

        Ok(event_proc)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_sm_name() {
        let input = "sm_name = testName";
        let smac = syn::parse_str::<StateMachine>(input).unwrap();
        assert_eq!("testName", smac.name.unwrap().to_string().as_str());
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
        let input = "sm_name = test 
        context {
            dd: u8,
        }";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        assert_eq!("test", smac.name.unwrap().to_string().as_str());
        for field in smac.context_fields {
            println!("{:?}", field);
        }
    }

    #[test]
    fn parse_event_1() {
        let input = "event TestEvent { dd: u8, }";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for evt in &smac.event_list {
            assert_eq!("TestEvent", evt.name.as_ref().unwrap().to_string().as_str());
        }

        assert_eq!(1, smac.event_list.len(), "Single Event not found");
    }

    #[test]
    fn parse_event_2() {
        let input = "event TestEvent1 { dd: u8, } 
                    event TestEvent2 { tt: u8, }";
        let mut idx: u8 = 1;

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for evt in &smac.event_list {
            assert_eq!(format!("TestEvent{}", idx), evt.name.as_ref().unwrap().to_string().as_str());
            idx += 1;
        }

        assert_eq!(2, smac.event_list.len(), "Single Event not found");
    }

    #[test]
    fn parse_state_1() {
        let input = "state TestState1";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for name in &smac.state_list {
            assert_eq!("TestState1", name.to_string().as_str());
        }

        assert_eq!(1, smac.state_list.len());
    }

    #[test]
    fn parse_state_2() {
        let input = "state TestState1
                     state TestState2";
        let mut idx: u8 = 1;

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for name in &smac.state_list {
            assert_eq!(format!("TestState{}", idx), name.to_string().as_str());
            idx += 1;
        }

        assert_eq!(2, smac.state_list.len());
    }

    #[test]
    fn parse_default() {
        let input = " state TestStateDef
                    default TestStateDef";
        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        assert_eq!("TestStateDef", smac.state_default.unwrap().to_string().as_str());
    }

    #[test]
    fn parse_proc_1() {
        let input = "
            event e0 {}
            state S0
            default S0
            proc S0 : e0 {
                let x = 20;
            }
            ";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        println!("Hay this is working");

        assert_eq!(smac.proc_list.is_empty(), false);
        for proc in smac.proc_list {
            assert_eq!(proc.state_name.unwrap().to_string().as_str(), "S0");
            assert_eq!(proc.event_name.unwrap().to_string().as_str(), "e0");
            assert_eq!(proc.event_proc_list.len(), 1);
        }
    }
}
