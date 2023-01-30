

use core::ptr;

enum UartRegOffset {
    Status = 0x00,
    Value = 0x04,
    ControlReg1 = 0x0c,
}

fn get_uart1() -> * mut u8 {
    0x4001_3800 as * mut u8
}

pub fn init_uart_tx() {
    let uart1_ptr = get_uart1();

    unsafe {
        ptr::write_volatile(uart1_ptr.offset(UartRegOffset::ControlReg1 as isize) as *mut u32, 0x2004 as u32);
    }
}

fn send_uart(ch : char) {
    let uart1_ptr = get_uart1();

    unsafe {
        ptr::write_volatile(uart1_ptr.offset(UartRegOffset::Value as isize) as *mut u32, ch as u32);
    }

    loop {
        let st: u32= unsafe { ptr::read_volatile(uart1_ptr.offset(UartRegOffset::Status as isize) as *mut u32) };

        if st & 0x80 == 0x80 {
            break;
        }
    }
}

pub fn log_str(s: &str) {
    for ch in s.chars() {
        send_uart(ch);
    }
}


