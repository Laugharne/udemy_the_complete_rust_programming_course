fn main() {
	let mut num = 5;
	let r1 = &num as *const i32;	
	let r2 = &mut num as *mut i32;

	println!("r1 is {:?}", r1);	// -> r1 is 0x7ffe9217c6e4
	println!("r2 is {:?}", r2);	// -> r2 is 0x7ffe9217c6e4

	// BUT if we try to deference this,
	// then it become an unsafe operation
//	println!("r1 is {:?}", *r1);
//	println!("r2 is {:?}", *r2);
/*
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
  --> src/main.rs:11:25
   |
11 |     println!("r1 is {:?}", *r1);
   |                            ^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

*/

	unsafe {
		println!("r1 is {:?}", *r1);	// -> r1 is 5
		println!("r2 is {:?}", *r2);	// -> r2 is 5
	}


}
