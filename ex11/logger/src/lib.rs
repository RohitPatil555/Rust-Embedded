#![no_std]

pub trait MyLogger {
    fn init_logger(&mut self) -> bool;
    fn write_byte(&mut self, ch: char) -> bool;
}

pub fn log<T>(hdlr: &mut T, s: &str)
where
    T: MyLogger,
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
    extern crate std;

    use super::*;
    use std::string::String;

    #[test]
    fn test_log() {
        let mut hdlr = UartLog {
            st: String::default(),
        };
        let _ = hdlr.init_logger();

        let _ = log::<UartLog>(&mut hdlr, "Hello World !");
        assert_eq!(hdlr.get_output(), "Hello World !\n");
    }

    struct UartLog {
        st: String,
    }

    impl MyLogger for UartLog {
        fn init_logger(&mut self) -> bool {
            return true;
        }

        fn write_byte(&mut self, ch: char) -> bool {
            //print!("{}", ch);
            self.st.push(ch);
            return true;
        }
    }

    impl UartLog {
        fn get_output(&self) -> String {
            self.st.clone()
        }
    }
}
