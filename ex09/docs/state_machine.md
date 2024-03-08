
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
        // temp: u8;
    }
    
    state <state_name>
    
    default <state_name>
    
    proc <state_name> {
        <event_name> => {
            ...
            next <= <state_name>
            result <= {error,success}
        }
        all_events => {
            ...
        }
    }
}

```

## Generated Structure

Above code generate below structure.

```ignore
pub struct <event_name>_io {
    pub #<event_name context>
}

pub enum <sm_name>_events {
    pub <event_name>(&<event_name_io>),
}

enum <sm_name>_states {
    <state_name>
}

pub struct <sm_name> {
    #context
}

impl <sm_name> {
    pub fn process(self, <sm_name_events>) -> Option(u8) {
        // add match statement for state
        // and call corresponding state processing function.
    } 
    
    fn process_<state_name>(self, <sm_name_events>) -> Option(u8) {
         // add event match case 
         // handle processing of events.
    }    
}

```

## Code Observation

Please execute below command to view generated code by macro.

```ignore
cargo expand --test macro_parse
```




