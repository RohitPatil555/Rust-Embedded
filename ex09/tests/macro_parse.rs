extern crate ex09;

use ex09::state_mac;

state_mac!(
    sm_name = "Smac"
    context {
        s: bool,
    }

    event "testE1" {
        j: u8,
    }

    event "testE2" {
    }
);
