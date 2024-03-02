use ex09::state_mac;

#[test]
fn parse_macro() {
    state_mac!(sm_name = "AirConditioner");
}
