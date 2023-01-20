use core::slice::from_raw_parts;
use core::str::from_utf8;

#[repr(C)]
#[derive(Debug)]
pub struct MyInfo {
	a: u32,
	b: u32
}

#[repr(C)]
pub enum MyEnum {
	Hello = 1,
	Bye
}

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

#[no_mangle]
pub extern "C" fn rust_string(a:*mut char, c: u32) {
	let s = from_utf8(unsafe{from_raw_parts(a as *mut u8, c as usize)}).unwrap();
	println!("Got string {}", s);
}

#[no_mangle]
pub extern "C" fn rust_struct_print(p:*mut MyInfo) {
	println!("{:x?}", unsafe { p.as_ref() }.unwrap() );
}

#[no_mangle]
pub extern "C" fn rust_enum_print(v: MyEnum) {
	match v {
		MyEnum::Hello => println!("Hello from C"),
		MyEnum::Bye => println!("Bye from C"),
	}
}
