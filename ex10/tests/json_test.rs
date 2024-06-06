extern crate ex10;

use ex10::{JsonEncoder, create_json_trait};

create_json_trait!();

#[derive(JsonEncoder)]
struct HelloMe {
    #[json(tag = "Index")]
    a: u8, // tag=Index ; This indicate json query Index
}

mod tests {
    use super::*;

    #[test]
    fn json_encoder_print() {
        let h : HelloMe = HelloMe{a:1};

        assert_eq!(h.json_get(), "{\"Index\" : 1 }");
    }
}
