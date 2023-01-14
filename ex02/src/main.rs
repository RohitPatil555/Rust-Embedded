use core::ffi::*;

#[repr(C)]
pub struct test_t {
	a : c_int,
	b : c_int,
}

#[repr(C)]
pub enum test_e {
	Test0,
	Test1,
	Test2,
	Test88 = 88,
	Test89,
}

extern "C" {
    pub fn hello_to(name: c_int) -> c_int;
	pub fn hello_name(name: *const c_char);
	pub fn print_array(p : *mut c_int, c: c_int);
	pub fn print_struct(p : *mut test_t);
	pub fn print_enum(val : test_e);
}


fn main() {
	// Generic type
    let x = unsafe{ hello_to(444) };
    println!("Return code : {}", x);

	// String type
	unsafe { hello_name("cool\0".as_ptr() as *const i8) }; 

	// Array type
	let mut arr: [i32;5] = [11, 22, 33, 44, 55];
	unsafe { print_array(arr.as_mut_ptr(), arr.len() as i32) };

	// Structure type
	let mut t = test_t{ a: 66, b: 77};
	unsafe { print_struct(&mut t as *mut test_t) };

	// Enum type
	unsafe { print_enum(test_e::Test0) };
	unsafe { print_enum(test_e::Test1) };
	unsafe { print_enum(test_e::Test2) };
	unsafe { print_enum(test_e::Test88) };
	unsafe { print_enum(test_e::Test89) };
}
