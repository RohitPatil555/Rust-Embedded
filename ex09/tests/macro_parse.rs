#![no_std]
extern crate ex09;

use ex09::state_mac;
use core::sync::atomic::{AtomicU8, Ordering};

static G_MOTORCOUNT : AtomicU8 = AtomicU8::new(0);

state_mac!(
    sm_name = Smac
    context {
        s: bool,
    }

    event PressStartButton {}
    event PressStopButton {}
    
    state MotorIdle
    state MotorRunning

    default MotorIdle

    proc MotorIdle:PressStartButton {
        G_MOTORCOUNT.fetch_add(1, Ordering::SeqCst);
        next_state = MotorRunning;
    }

    proc MotorIdle:PressStopButton {
    }

    proc MotorRunning:PressStartButton {
    }

    proc MotorRunning:PressStopButton {
        G_MOTORCOUNT.fetch_sub(1, Ordering::SeqCst);
    }
);

mod tests {
    use super::*;

    #[test]
    fn motor_success_control() {
        let mut smac = Smac::new();

        smac.process(SmacEvent::PressStartButton);
        assert_eq!(G_MOTORCOUNT.load(Ordering::SeqCst), 1);

        smac.process(SmacEvent::PressStopButton);
        assert_eq!(G_MOTORCOUNT.load(Ordering::SeqCst), 0);
    } 
}
