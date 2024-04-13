extern crate ex09;

use ex09::state_mac;

state_mac!(
    sm_name = Smac
    context {
        s: bool,
    }

    event testE1 {
    }

    event testE2 {
    }
    
    state S0
    state S1
    state S2
    state S3

    default S0

    proc S0:testE1 {
        let _ = 20;
    }

    proc S0:testE2 {
        let _ = 20;
    }

    proc S1:testE1 {
        let _ = 20;
    }

    proc S1:testE2 {
        let _ = 20;
    }

    proc S2:testE1 {
        let _ = 20;
    }

    proc S2:testE2 {
        let _ = 20;
    }

    proc S3:testE1 {
        let _ = 20;
    }

    proc S3:testE2 {
        let _ = 20;
    }
);

