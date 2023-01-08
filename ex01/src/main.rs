#![no_std] 
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn Reset() -> ! {
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
    
