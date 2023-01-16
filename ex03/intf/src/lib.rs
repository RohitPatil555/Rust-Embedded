use core::slice::from_raw_parts;

#[no_mangle]
pub extern "C" fn rust_hello() {
	println!("Hello World from Rust !!");
}

#[no_mangle]
pub extern "C" fn rust_add(a: u32, b: u32) -> u32 {
	a + b
}

#[no_mangle]
pub extern "C" fn rust_array(a:*mut u32, c: u32) {
	let a = unsafe{ from_raw_parts( a, c as usize) };
	for (i, x) in a.iter().enumerate() {
		println!("Array {} => {:x}", i, x);
	}
}
