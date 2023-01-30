#![no_std] 
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
use alloc::boxed::Box;

use core::panic::PanicInfo;

mod uart;
mod calloc;


#[no_mangle]
pub extern "C" fn Reset() -> ! {
    uart::init_uart_tx();
    uart::log_str("Hello World !!\n");

	let _x = Box::new(2);

    loop {}
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
    
