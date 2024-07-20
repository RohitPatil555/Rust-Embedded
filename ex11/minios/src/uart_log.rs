use core::ptr;
use logger::MyLogger;

enum UartRegOffset {
    Status = 0x00,
    Value = 0x04,
    ControlReg1 = 0x0c,
}

pub(crate) struct UartLog {
    base: *mut u8,
}

impl UartLog {
    pub(crate) const fn new(uart_base_addr: *mut u8) -> Self {
        UartLog {
            base: uart_base_addr,
        }
    }
}

impl MyLogger for UartLog {
    fn init_logger(&mut self) -> bool {
        unsafe {
            ptr::write_volatile(
                self.base.offset(UartRegOffset::ControlReg1 as isize) as *mut u32,
                0x2004_u32,
            );
        }

        true
    }

    fn write_byte(&mut self, ch: char) -> bool {
        unsafe {
            ptr::write_volatile(
                self.base.offset(UartRegOffset::Value as isize) as *mut u32,
                ch as u32,
            );
        }

        loop {
            let st: u32 = unsafe {
                ptr::read_volatile(self.base.offset(UartRegOffset::Status as isize) as *mut u32)
            };

            if st & 0x80 == 0x80 {
                break;
            }
        }

        true
    }
}

unsafe impl Sync for UartLog {}
