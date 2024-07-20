use pool_allocator::pool::Pool;

static mut MEM_BUF: [u8; 1028] = [0; 1028];

#[test]
fn test_initialize() {
    let mut pool = Pool::<100, 2>::init();
    let _ = unsafe { pool.create(MEM_BUF.as_mut_ptr(), 1028) };
    assert_eq!(pool.get_count(), 0);
}

#[test]
fn test_print_blocks() {
    let mut pool = Pool::<100, 2>::init();
    let ptr_prev: *mut u8;
    let ptr_next: *mut u8;

    let _ = unsafe { pool.create(MEM_BUF.as_mut_ptr(), 1028) };
    assert_eq!(pool.get_count(), 0);

    ptr_prev = pool.alloc().unwrap();
    assert_eq!(pool.get_count(), 1);

    ptr_next = pool.alloc().unwrap();
    assert_eq!(
        unsafe { ptr_next.offset_from(ptr_prev) },
        pool.get_block_size().try_into().unwrap()
    );
    assert_eq!(pool.get_count(), 2);

    assert_eq!(pool.alloc(), None);
    assert_eq!(pool.get_count(), 2);

    // let start free the blocks
    pool.free(ptr_next);
    assert_eq!(pool.get_count(), 1);

    pool.free(ptr_prev);
    assert_eq!(pool.get_count(), 0);

    pool.free(ptr_prev);
    assert_eq!(pool.get_count(), 0);
}
