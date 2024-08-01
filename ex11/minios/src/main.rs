#![no_std]
#![no_main]

extern crate alloc;

mod example;
mod executor;
mod pool_alloc;
mod task;
mod uart_log;

use crate::task::Task;
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut};
use example::example_task;
use executor::Executor;
use pool_alloc::create_pool;
use uart_log::{log_uart, log_uart_init};

extern "C" {
    static mut heap_start: u8;
    static mut heap_end: u8;
}

#[no_mangle]
pub extern "C" fn Reset() -> ! {
    unsafe {
        let heap_size = addr_of!(heap_end) as usize - addr_of!(heap_start) as usize;
        create_pool(addr_of_mut!(heap_start), heap_size);

        log_uart_init();
    }

    log_uart("Hello Mini-OS !!\n");

    scheduler_loop();

    log_uart("Main completed !!\n");

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

fn scheduler_loop() {
    let mut sched = Executor::new();
    sched.spawn(Task::new(example_task()));
    sched.run();
}
