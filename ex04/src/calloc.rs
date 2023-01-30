
use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;

//use uart::log_str;

/*extern {
	static heap_start_addr: u8;
	static heap_end_addr: u8;
}*/

struct BumpPointerAlloc {
	head: UnsafeCell<usize>,
	end: usize,
}

unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let head = self.head.get();
		let size = layout.size();
		let align = layout.align();
		let align_mask = !(align - 1);

		let start = (*head + align - 1) & align_mask;

		if start + size > self.end {
			ptr::null_mut()
		} else {
			*head = start + size;
			start as *mut u8
		}
	}

	unsafe fn dealloc(&self, _:*mut u8, _: Layout) {
	}
}

#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc {
	head: UnsafeCell::new(0x2000_0400),
	end: 0x2000_2000,
};

#[alloc_error_handler]
pub unsafe fn calloc_error(_: Layout) -> ! {
	loop {}
} 
