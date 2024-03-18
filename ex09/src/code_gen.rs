use crate::custom_parse::SmacEvent;
use proc_macro2::Ident;
use quote::format_ident;
use syn::{parse_quote, ItemEnum, ItemStruct};

pub fn gen_event_struct(evt: &SmacEvent) -> Option<ItemStruct> {
    let name = format_ident!("Param{}", evt.name);
    let fields = &evt.evt_fields;

    if !fields.is_empty() {
        let ost: ItemStruct = parse_quote!(
            struct #name {
                #fields
            }
        );

        return Some(ost);
    }

    None
}

pub fn gen_event_enum(evts: &Vec<SmacEvent>) -> Option<ItemEnum> {
    let mut name_list: Vec<Ident> = vec![];
    let mut name_list_arg: Vec<Ident> = vec![];
    let mut unit_name_list: Vec<Ident> = vec![];
    let ie: ItemEnum;

    for e in evts {
        if !e.evt_fields.is_empty() {
            name_list.push(format_ident!("{}", e.name));
            name_list_arg.push(format_ident!("Param{}", e.name));
        } else {
            unit_name_list.push(format_ident!("{}", e.name));
        }
    }

    if name_list.is_empty() && unit_name_list.is_empty() {
        return None;
    } else if name_list.is_empty() {
        ie = parse_quote!(
            enum SmacEvent {
                #(#unit_name_list),*
            }
        );
    } else if unit_name_list.is_empty() {
        ie = parse_quote!(
            enum SmacEvent {
                #(#name_list(#name_list_arg)),*
            }
        );
    } else {
        ie = parse_quote!(
            enum SmacEvent {
                #(#unit_name_list),* ,
                #(#name_list(#name_list_arg)),*
            }
        );
    }

    Some(ie)
}

pub fn get_state_enum(state_list: &Vec<String>) -> Option<ItemEnum> {
    let mut state_idents: Vec<Ident> = vec![];

    for st in state_list {
        state_idents.push(format_ident!("{}", st));
    }

    let ie: ItemEnum = parse_quote!(
        enum SmacState {
            #(#state_idents),*
        }
    );

    Some(ie)
}

#[cfg(test)]
mod tests {
    use crate::custom_parse::StateMachine;

    use super::{gen_event_enum, gen_event_struct, get_state_enum};

    #[test]
    fn event_test_1() {
        let input = "event \"TestE1\" { dd: u8, }";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        let st = gen_event_struct(&smac.event_list[0]);
        assert_ne!(st, None, "Struct is not exepected to be None type");
    }

    #[test]
    fn event_test_2() {
        let input = "event \"TestE1\" {}";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        let st = gen_event_struct(&smac.event_list[0]);
        assert_eq!(st, None, "Fail to get None");
    }

    #[test]
    fn event_test_3() {
        let input = "
            event \"TestE1\" {}
            event \"TestE2\" {}
            ";
        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for e in &smac.event_list {
            let st = gen_event_struct(&e);
            assert_eq!(st, None, "Expected to get argument structure as none");
        }

        let out = gen_event_enum(&smac.event_list).unwrap();
        println!("{:?}", out);
    }

    #[test]
    fn event_test_4() {
        let input = "
            event \"TestE1\" { k : u8 }
            event \"TestE2\" { t : u8 }
            ";
        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for e in &smac.event_list {
            let st = gen_event_struct(&e);
            assert_ne!(st, None, "Created Parameter Structure");
        }

        let out = gen_event_enum(&smac.event_list).unwrap();
        println!("{:?}", out);
    }

    #[test]
    fn event_test_5() {
        let input = "
            event \"TestE1\" { k : u8 }
            event \"TestE2\" {}
            ";
        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        for e in &smac.event_list {
            let _ = gen_event_struct(&e);
        }

        let out = gen_event_enum(&smac.event_list).unwrap();
        println!("{:?}", out);
    }

    #[test]
    fn state_test_1() {
        let input = "
                state  \"S1\"
                state  \"S2\"
                state  \"S3\"
                state  \"S4\"
            ";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        let _ = get_state_enum(&smac.state_list);
    }
}
