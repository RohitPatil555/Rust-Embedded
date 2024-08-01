use crate::uart_log::log_uart;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use pool_allocator::pool::Pool;

static mut POOL_1: Pool<12, 100> = Pool::<12, 100>::init();
static mut POOL_2: Pool<40, 20> = Pool::<40, 20>::init();

struct PoolAllocator {}

#[global_allocator]
static POOL_ALLOC: PoolAllocator = PoolAllocator {};

unsafe impl Sync for PoolAllocator {}

unsafe impl GlobalAlloc for PoolAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let _align = layout.align();

        if POOL_1.get_block_size() >= size {
            let buf = POOL_1.alloc();

            if buf.is_none() {
                return null_mut::<u8>();
            }

            return buf.unwrap();
        }

        if POOL_2.get_block_size() >= size {
            let buf = POOL_2.alloc();

            if buf.is_none() {
                return null_mut::<u8>();
            }

            return buf.unwrap();
        }

        log_uart("Fail allocator ?");
        null_mut::<u8>()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let _align = layout.align();

        if POOL_1.get_block_size() >= size {
            POOL_1.free(ptr);
            return;
        }

        if POOL_2.get_block_size() >= size {
            POOL_2.free(ptr);
            return;
        }
    }
}

pub(crate) unsafe fn create_pool(start: *mut u8, mem_size: usize) {
    let mut mem_addr = start;
    let mut free_mem_size = mem_size;

    let res = POOL_1.create(mem_addr, free_mem_size);

    if res.is_err() {
        log_uart("Pool1 Create Failed !");
    }

    free_mem_size -= POOL_1.get_pool_size();
    mem_addr = mem_addr.wrapping_add(POOL_1.get_pool_size());

    let res = POOL_2.create(mem_addr, free_mem_size);

    if res.is_err() {
        log_uart("Pool2 Create Failed !");
    }
}
