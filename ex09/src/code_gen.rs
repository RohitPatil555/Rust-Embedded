use crate::custom_parse::{StateMachine, SmacEvent};
use quote::format_ident;
use syn::{parse_quote, Ident, Arm, Expr, Stmt,
    ItemEnum, ItemStruct, ItemImpl, ItemFn};

impl StateMachine {
    pub fn create_event_enum(&self) -> ItemEnum {
        let smac_name = self.name.as_ref().unwrap();
        let mut event_types : Vec<&Ident> = vec![];
        for evt in &self.event_list {
            let evt_ident = evt.name.as_ref().unwrap();

            event_types.push(evt_ident);
        }
        let event_enum_name = format_ident!("{}Event", smac_name);

        let event_enum : ItemEnum = parse_quote!(
            enum #event_enum_name {
                #(#event_types),*
            }
        );

        event_enum
    }

    pub fn create_state_enum(&self) -> ItemEnum {
        let smac_name = self.name.as_ref().unwrap();
        let state_types = &self.state_list;

        let state_enum_name = format_ident!("{}State", smac_name);

        let state_enum : ItemEnum = parse_quote!(
            enum #state_enum_name {
                #(#state_types),*
            }
        );

        state_enum
    }

    pub fn create_state_default(&self) -> ItemImpl {
        let smac_name = self.name.as_ref().unwrap();
        let state_default = self.state_default.as_ref().unwrap();

        let state_enum_name = format_ident!("{}State", smac_name);
        
        let state_default_impl : ItemImpl = parse_quote!(
            impl Default for #state_enum_name {
                fn default() -> Self {
                    #state_enum_name::#state_default
                }
            }
        );

        state_default_impl
    }

    pub fn create_smac_struct(&self) -> ItemStruct {
        // generate structure for self
        let smac_name = self.name.as_ref().unwrap();
        let smac_context = &self.context_fields;
        let smac_state_name = format_ident!("{}State",smac_name.to_string());

        let smac_struct : ItemStruct = parse_quote!(
            #[derive(Default)]
            struct #smac_name {
                state : #smac_state_name,
                #smac_context
            }
        );

        smac_struct
    }

    pub fn create_smac_impl(&self) -> ItemImpl {
        let smac_name = self.name.as_ref().unwrap();
        let mut func_list : Vec<ItemFn> = vec![];

        func_list.push(self._create_process_func());

        for _st in &self.state_list {
            let _st_func = self._create_state_process_function(_st);
            func_list.push(_st_func);

            // Generate event related APIs.
            for _e in &self.event_list {
                let _evt_func = self._create_state_event_process_function(_st, &_e);
                func_list.push(_evt_func);
            }
        }

        let smac_impl : ItemImpl = parse_quote!(
            impl #smac_name {
                pub fn new() -> Self {
                    #smac_name::default()
                }

                #(#func_list)*
            }
        );

        smac_impl
    }

    fn _create_process_func(&self) -> ItemFn {
        let smac_name = self.name.as_ref().unwrap();
        let smac_event_name = format_ident!("{}Event", smac_name); 
        let smac_state_type = format_ident!("{}State", smac_name);    
        let mut arm_list : Vec<Arm> = vec![];

        for smac_st in &self.state_list {
            let _func_name = format_ident!("_proc_{}", smac_st.to_string());
            let _arm : Arm = parse_quote!(
                #smac_state_type::#smac_st => self.#_func_name(evt)
            );
            arm_list.push(_arm);
        }
        
        let proc_fn : ItemFn = parse_quote!(
            pub fn process(&mut self, evt: #smac_event_name) {
                self.state = match self.state {
                    #(#arm_list),*
                };
            }
        );

        proc_fn
    }

    fn _create_state_process_function(&self, _state: &Ident) -> ItemFn {
        let smac_name = self.name.as_ref().unwrap();
        let smac_event_type = format_ident!("{}Event", smac_name);    
        let smac_state_type = format_ident!("{}State", smac_name);
        let mut arm_list : Vec<Arm> = vec![];

        for evt in &self.event_list {
            let evt_name = evt.name.as_ref().unwrap();
            let _func_name = format_ident!("_proc_{}_{}",_state.to_string(), evt_name.to_string());
            let _arm : Arm = parse_quote!(
                #smac_event_type::#evt_name => self.#_func_name()
            );
            arm_list.push(_arm);
        }

        let _proc_func = format_ident!("_proc_{}",_state.to_string());
        let proc_state : ItemFn = parse_quote!(
            fn #_proc_func(&self, evt: #smac_event_type) -> #smac_state_type {
                let state = match evt {
                    #(#arm_list),*
                };

                state
            }
        );

        proc_state
    }

    fn _replace_next_state(&self, _proc_list: &Vec<Stmt>) -> Vec<Stmt> {
        let smac_name = self.name.as_ref().unwrap();
        let smac_state_type = format_ident!("{}State", smac_name);
        //let clone_data = self.proc_list.clone();
        let final_stmt = _proc_list.clone()
                        .into_iter()
                        .map(|stmt| match stmt {
                            Stmt::Expr(ref expr, _) => {
                                if let Expr::Assign(ref assign) = expr {
                                    if let Expr::Path(lpath) = &*assign.left {
                                        if let Some(lident) = lpath.path.get_ident() {
                                            if lident.to_string() == "next_state" {
                                                if let Expr::Path(rpath) = &*assign.right {
                                                    if let Some(rident) = rpath.path.get_ident() {
                                                        let stmt_change : Stmt = parse_quote!(
                                                            state = #smac_state_type::#rident;
                                                        );
                                                        return stmt_change;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                stmt
                            }
                            _ => stmt,
                        })
                        .collect::<Vec<_>>();

        final_stmt
    }

    fn _create_state_event_process_function(&self, _state: &Ident, _event: &SmacEvent) -> ItemFn {
        let smac_event_ident = _event.name.as_ref().unwrap();
        let smac_name = self.name.as_ref().unwrap();
        //let smac_event_type = format_ident!("{}Event", smac_name);    
        let smac_state_type = format_ident!("{}State", smac_name);
        let _smac_proc = self.proc_list.iter().find(|_proc| 
                (_proc.state_name.as_ref().unwrap().to_string() == _state.to_string() && 
                 _proc.event_name.as_ref().unwrap().to_string() == _event.name.as_ref().unwrap().to_string())
            );
        
        let _non_update_proc_contain = &_smac_proc.as_ref().unwrap().event_proc_list;
        let _proc_contain = self._replace_next_state(_non_update_proc_contain);

        let _proc_func = format_ident!("_proc_{}_{}",_state.to_string(), smac_event_ident.to_string());
        let proc_event : ItemFn = parse_quote!(
            fn #_proc_func(&self) -> #smac_state_type {
                let mut state = #smac_state_type::#_state;
                #(#_proc_contain)*
                state               
            }
        );

        proc_event
    }
}