use crate::custom_parse::{SmacEvent,SmacEventProc};
use proc_macro2::Ident;
use quote::format_ident;
use syn::{parse_quote, ItemEnum, ItemStruct, ItemFn, Arm};

pub fn gen_event_struct(evt: &SmacEvent) -> Option<ItemStruct> {
    let name = format_ident!("Param{}", evt.name.as_ref().unwrap().to_string());
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
            name_list.push(format_ident!("{}", e.name.as_ref().unwrap().to_string()));
            name_list_arg.push(format_ident!("Param{}", e.name.as_ref().unwrap().to_string()));
        } else {
            unit_name_list.push(format_ident!("{}", e.name.as_ref().unwrap().to_string()));
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

pub fn get_state_enum(state_list: &Vec<Ident>) -> Option<ItemEnum> {
    let ie: ItemEnum = parse_quote!(
        enum SmacState {
            #(#state_list),*
        }
    );

    Some(ie)
}

pub fn get_proc_function(st_evt_proc: &SmacEventProc) -> Option<ItemFn> {
    let st_ident = st_evt_proc.state_name.as_ref().unwrap();
    let st_name : String = st_evt_proc.state_name.as_ref().unwrap().to_string();
    let evt_name : String = st_evt_proc.event_name.as_ref().unwrap().to_string();
    let func_name = format_ident!("proc_{}_{}", st_name, evt_name);
    let proc_func : ItemFn = parse_quote! (
        fn  #func_name() -> SmacState {
            SmacState::#st_ident
        }
    );

    Some(proc_func)
}

pub fn get_state_function(evts: &Vec<SmacEvent>, state_name: &Ident) -> ItemFn {
    let mut evt_list: Vec<Arm> = vec![];

     for evt in evts {
         let evt_ident = evt.name.as_ref().unwrap();
         let func_name = format_ident!("proc_{}_{}", state_name.to_string(), evt_ident.to_string());
         let evt_stmt : Arm = parse_quote! {
             SmacEvent::#evt_ident => #func_name()
         };
         evt_list.push(evt_stmt);
     }

    let func_name = format_ident!("proc_{}", state_name.to_string());
    let state_proc : ItemFn = parse_quote!(
        fn #func_name(evt : SmacEvent) -> SmacState {
            let state = match evt {
                #(#evt_list),*
            };

            state
        }
    );

    state_proc
}

pub fn get_interface_function(state_list: &Vec<Ident>, smac_name: &Ident) -> ItemFn {
    let mut arm_list: Vec<Arm> = vec![];
    for st in state_list {
        let st_proc = format_ident!("proc_{}",st.to_string());
        let st_stmt : Arm = parse_quote!{
            SmacState::#st => #st_proc(evt)
        };
        arm_list.push(st_stmt);
    }

    let func_name = format_ident!("proc_events_{}", smac_name.to_string());
    let intf_func : ItemFn = parse_quote!(
        fn #func_name(evt: SmacEvent) {
            unsafe {
                G_STATE = match G_STATE {
                    #(#arm_list),*
                }
            }
        }
    );

    intf_func
}

#[cfg(test)]
mod tests {

    use crate::custom_parse::StateMachine;

    use super::{gen_event_enum, gen_event_struct, get_state_enum};

    #[test]
    fn event_test_1() {
        let input = "event TestE1 { dd: u8, }";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        let st = gen_event_struct(&smac.event_list[0]);
        assert_ne!(st, None, "Struct is not exepected to be None type");
    }

    #[test]
    fn event_test_2() {
        let input = "event TestE1 {}";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        let st = gen_event_struct(&smac.event_list[0]);
        assert_eq!(st, None, "Fail to get None");
    }

    #[test]
    fn event_test_3() {
        let input = "
            event TestE1 {}
            event TestE2 {}
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
            event TestE1 { k : u8 }
            event TestE2 { t : u8 }
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
            event TestE1 { k : u8 }
            event TestE2 {}
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
                state  S1
                state  S2
                state  S3
                state  S4
            ";

        let smac = syn::parse_str::<StateMachine>(input).unwrap();

        let _ = get_state_enum(&smac.state_list);
    }

}
