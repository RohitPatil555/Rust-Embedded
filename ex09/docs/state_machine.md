
# Domain Specific Language

In this tutorial, we are exploring DSL using rust procedure macro.

## Language Structure

Let consider below Language procedure for our DSL.

```ignore
state_mac!{

    sm_name = <state machine name>

    context {
        // rust struct define style here e.g.
        // temp: u8;       
    }

    event <event_name> {
        // rust struct define style here e.g.
    }
    
    state <state_name>
    
    default <state_name>
    
    proc <state_name>:<event_name> {
      ...
    }
}

```

## Generated Code Structure

Above code generate below structure.

```ignore

pub enum <sm_name>_events {
    <event_name>
}

enum <sm_name>_states {
    <state_name>
}

pub struct <sm_name> {
    state: <sm_name>_states,
}

impl <sm_name> {
    pub fn new() -> Self {
        Box::new(<sm_name>::default())
    }

    pub fn process(self, <sm_name_events>) -> Option(u8) {
        // add match statement for state
        // and call corresponding state processing function.
    } 
    
    fn proc_<state_name>(self, <sm_name_event>) -> <sm_name>_states {
         // add event match case 
         // handle processing of events.
    }    

    fn proc_<state_name>_<event_name>(self, <sm_name_event>) {
        // add processing code of event.
    }
}

```

## Interface Use

* First create state machine instance using below interface.
```ignore
    sm_inst = <sm_name>::new();
```

* Then process event using below API.
```ignore
    sm_inst.process(event);
```

## Code Observation

Please execute below command to view generated code by macro.

```ignore
cargo expand --test macro_parse
```




