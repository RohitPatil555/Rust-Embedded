use crate::uart_log::log_uart;
use alloc::format;

async fn async_number() -> u32 {
    log_uart("async number\n");
    42
}

pub async fn example_task() {
    let number = async_number().await;
    log_uart(format!("example_task: {}", number).as_str());
}
