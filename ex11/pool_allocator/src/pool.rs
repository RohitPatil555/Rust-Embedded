use crate::block::Block;

pub struct Pool<const SIZE: usize, const COUNT: usize> {
    free: Option<*mut Block<SIZE>>,
    base: Option<*mut Block<SIZE>>,
    cur_count: usize,
    block_size: usize,
}

impl<const SIZE: usize, const COUNT: usize> Pool<SIZE, COUNT> {
    pub unsafe fn new(start: *mut u8, mem_size: usize) -> Option<Self> {
        let mut boff: usize = 0;
        let block_size: usize = size_of::<Block<SIZE>>();
        let mut pool = Pool::<SIZE, COUNT> {
            free: None,
            base: None,
            cur_count: 0,
            block_size: 0,
        };
        let mut prev_block: &mut Block<SIZE>;
        let mut temp_block: &mut Block<SIZE>;
        let mut next: *mut u8;

        unsafe {
            next = start.offset(boff.try_into().unwrap());
        }

        let block_start = Block::<SIZE>::new(next).unwrap();
        prev_block = block_start;

        for idx in 1..COUNT {
            boff = idx * block_size;
            if (boff + block_size) > mem_size {
                return None;
            }

            unsafe {
                next = start.offset(boff.try_into().unwrap());
            }
            temp_block = Block::<SIZE>::new(next).unwrap();
            prev_block.add_next(temp_block);
            prev_block = temp_block;
        }

        pool.base = Some(block_start);
        pool.free = Some(block_start);
        pool.cur_count = 0;
        pool.block_size = block_size;

        Some(pool)
    }

    pub fn alloc(&mut self) -> Option<*mut u8> {
        self.free?;

        let alloc_block = unsafe { self.free.unwrap().as_mut().unwrap() };
        self.free = alloc_block.get_next();
        alloc_block.remove_next();
        self.cur_count += 1;
        Some(alloc_block.get_value())
    }

    pub fn free(&mut self, val_ptr: *mut u8) {
        let block_to_free = Block::<SIZE>::get_block_from_value(val_ptr).unwrap();

        if self.cur_count == 0 {
            return;
        }

        if self.free.is_none() {
            self.free = Some(block_to_free);
        } else {
            let next_block = self.free.unwrap();
            block_to_free.add_next(next_block);
            self.free = Some(block_to_free);
        }

        self.cur_count -= 1;
    }

    #[cfg(feature = "test-utils")]
    pub fn get_count(&self) -> usize {
        self.cur_count
    }

    #[cfg(feature = "test-utils")]
    pub fn get_block_size(&self) -> usize {
        self.block_size
    }
}
