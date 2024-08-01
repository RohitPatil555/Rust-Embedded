use pool_allocator::pool::Pool;

static mut POOL_1: Pool<40, 10> = Pool::<40, 10>::init();
static mut POOL_2: Pool<100, 10> = Pool::<100, 10>::init();

static mut MEM_BUF: [u8; 4112] = [0; 4112];

#[test]
fn create_pooling() {
    let mut mem_addr: *mut u8 = unsafe { MEM_BUF.as_mut_ptr() };
    let mut free_memory: usize = 4112;

    unsafe {
        println!(
            "Memory address : {:p} and free memory : {}",
            mem_addr, free_memory
        );
        let _ = POOL_1.create(mem_addr, free_memory);
        free_memory -= POOL_1.get_pool_size();

        mem_addr = mem_addr.wrapping_add(POOL_1.get_pool_size());
        println!(
            "Memory address : {:p} and free memory : {}",
            mem_addr, free_memory
        );
        let _ = POOL_2.create(mem_addr, free_memory);

        let ptr1 = POOL_1.alloc().unwrap();
        let ptr2 = POOL_2.alloc().unwrap();
        let ptr3 = POOL_2.alloc().unwrap();

        POOL_2.free(ptr2);
        POOL_1.free(ptr1);
        POOL_2.free(ptr1);
    }
}
