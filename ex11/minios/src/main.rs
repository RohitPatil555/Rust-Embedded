#![no_std]
#![no_main]

mod uart_log;

use core::panic::PanicInfo;
use core::ptr::addr_of_mut;
use logger::{log, MyLogger};
use uart_log::UartLog;

static mut UART_LOG: UartLog = UartLog::new(0x4001_3800 as *mut u8);

#[no_mangle]
pub extern "C" fn Reset() -> ! {
    unsafe {
        UART_LOG.init_logger();
        log::<UartLog>(
            addr_of_mut!(UART_LOG).as_mut().unwrap(),
            "Hello Mini-OS !!\n",
        );
    }

    panic!();
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
