pub(crate) struct Block<const SIZE: usize> {
    next: Option<*mut Block<SIZE>>,
    val: [u8; SIZE],
}

impl<const SIZE: usize> Block<SIZE> {
    pub(crate) fn new(start: *mut u8) -> Option<&'static mut Self> {
        let ret: *mut Block<SIZE>;
        unsafe {
            ret = start as *mut Block<SIZE>;
            ret.as_mut().unwrap().next = None;
            ret.as_mut()
        }
    }

    pub(crate) fn add_next(&mut self, next: *mut Self) {
        if self.next.is_none() {
            self.next = Some(next);
        }
    }

    pub(crate) fn remove_next(&mut self) {
        if self.next.is_none() {
            self.next = None;
        }
    }

    pub(crate) fn get_value(&mut self) -> *mut u8 {
        self.val.as_mut_ptr()
    }

    pub(crate) fn get_next(&mut self) -> Option<*mut Block<SIZE>> {
        self.next
    }

    pub(crate) fn get_block_from_value(val_ptr: *mut u8) -> Option<&'static mut Block<SIZE>> {
        let start_addr: *mut u8;
        unsafe {
            let block_start = 0x1000 as *mut Block<SIZE>;
            let block_val = block_start.as_ref().unwrap().val.as_ptr();
            let offset = block_val.offset_from(block_start as *const u8);

            start_addr = val_ptr.sub(offset.try_into().unwrap());
        }

        Block::<SIZE>::new(start_addr)
    }
}
