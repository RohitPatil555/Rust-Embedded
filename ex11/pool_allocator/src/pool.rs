use crate::block::Block;
use core::mem::size_of;

pub struct Pool<const SIZE: usize, const COUNT: usize> {
    free: Option<*mut Block<SIZE>>,
    base: Option<*mut Block<SIZE>>,
    cur_count: usize,
    block_size: usize,
}

impl<const SIZE: usize, const COUNT: usize> Pool<SIZE, COUNT> {
    pub const fn init() -> Self {
        let mut pool = Pool::<SIZE, COUNT> {
            free: None,
            base: None,
            cur_count: 0,
            block_size: 0,
        };

        pool.block_size = size_of::<Block<SIZE>>();
        pool
    }

    /// # Safety
    ///
    /// This API create bucket on raw memory.
    pub unsafe fn create(&mut self, start: *mut u8, mem_size: usize) -> Result<(), &str> {
        // SAFETY:  This API modify raw pointer and consume ti to create pool.
        let mut boff: usize = 0;
        let mut prev_block: &mut Block<SIZE>;
        let mut temp_block: &mut Block<SIZE>;
        let mut next: *mut u8;

        if self.get_pool_size() > mem_size {
            return Err("Memory is not enough");
        }

        unsafe {
            next = start.offset(boff.try_into().unwrap());
        }

        let block_start = Block::<SIZE>::new(next).unwrap();
        prev_block = block_start;

        for idx in 1..COUNT {
            boff = idx * self.block_size;
            if (boff + self.block_size) > mem_size {
                panic!("Error : Unsufficent memory");
            }

            unsafe {
                next = start.offset(boff.try_into().unwrap());
            }
            temp_block = Block::<SIZE>::new(next).unwrap();
            prev_block.add_next(temp_block);
            prev_block = temp_block;
        }

        self.base = Some(block_start);
        self.free = Some(block_start);
        self.cur_count = 0;

        Ok(())
    }

    fn is_addr_in_range(&mut self, addr: *mut u8) -> bool {
        let base_ptr = self.base.unwrap() as *mut u8;
        let offset = unsafe { addr.offset_from(base_ptr).try_into().unwrap() };
        let total_size = self.block_size * COUNT;

        if total_size > offset {
            return true;
        }

        false
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

        assert!(self.is_addr_in_range(val_ptr), "Error: Not valid pointer");

        if self.free.is_none() {
            self.free = Some(block_to_free);
        } else {
            let next_block = self.free.unwrap();
            block_to_free.add_next(next_block);
            self.free = Some(block_to_free);
        }

        self.cur_count -= 1;
    }

    pub fn get_block_size(&self) -> usize {
        self.block_size
    }

    pub fn get_pool_size(&self) -> usize {
        self.block_size * COUNT
    }

    #[cfg(feature = "test-utils")]
    pub fn get_count(&self) -> usize {
        self.cur_count
    }
}
