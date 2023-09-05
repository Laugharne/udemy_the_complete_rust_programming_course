// use std::thread;
// use std::sync::mpsc;	// Multi Procedure Single Consumer
// use std::sync::Arc;
// use std::rc:Rc;
use std::sync::{Arc, Mutex};


fn main() {
	// let rc1 = Arc::new(String::from("Test"));
	// let rc2 = rc1.clone();

	// std::thread::spawn(move || {
	// 	rc2;
	// });

// IF we use Rc instead of Arc
//	Rc<String>` cannot be sent between threads safely


//     |
// 12  |       std::thread::spawn(move || {
//     |       ------------------ ^------
//     |       |                  |
//     |  _____|__________________within this `[closure@src/main.rs:12:21: 12:28]`
//     | |     |
//     | |     required by a bound introduced by this call
// 13  | |         rc2;
// 14  | |     });
//     | |_____^ `Rc<String>` cannot be sent between threads safely
//     |
//     = help: within `[closure@src/main.rs:12:21: 12:28]`, the trait `Send` is not implemented for `Rc<String>`
// note: required because it's used within this closure
//    --> src/main.rs:12:21
//     |
// 12  |     std::thread::spawn(move || {
//     |                        ^^^^^^^
// note: required by a bound in `spawn`
//    --> /home/franck/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:704:8
//     |
// 704 |     F: Send + 'static,
//     |        ^^^^ required by this bound in `spawn`


// WITH Arc, we are now able to safely use a reference counter
// smart pointer inside a thread.


	let counter = Arc::new(Mutex::new(0));
	let mut handles = vec![];

	// '_' cause xwe don't care about the values
	for _ in 0..8 {
		let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
		let handle = std::thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num += 1;
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}

	println!("{}", counter.lock().unwrap());	// -> 8

}
