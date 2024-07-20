pub trait MyLoogger {
    fn init_logger(&self) -> bool;
    fn write_byte(&self, ch: char) -> bool;
}

pub fn log<T>(hdlr: T, s: &str)
where
    T: MyLoogger,
{
    for ch in s.chars() {
        if !hdlr.write_byte(ch) {
            return;
        }
    }

    let _ = hdlr.write_byte('\n');
}

#[cfg(test)]
mod tests {
    use super::*;

    struct UartLog {}

    impl MyLoogger for UartLog {
        fn init_logger(&self) -> bool {
            return true;
        }

        fn write_byte(&self, ch: char) -> bool {
            print!("{}", ch);
            return true;
        }
    }

    #[test]
    fn test_log() {
        let hdlr = UartLog {};
        let _ = hdlr.init_logger();

        let _ = log::<UartLog>(hdlr, "Hello World !");
    }
}
