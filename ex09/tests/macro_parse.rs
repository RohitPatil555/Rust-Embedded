extern crate ex09;

use ex09::state_mac;

state_mac!(
    sm_name = "WellPump"
    context {
        bPumpStarted: bool,
    }
);
