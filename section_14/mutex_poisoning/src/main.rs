// use std::thread;
// use std::sync::mpsc;	// Multi Procedure Single Consumer
// use std::sync::Arc;
// use std::rc:Rc;
use std::sync::{Arc, Mutex};


fn main() {

	let lock  = Arc::new(Mutex::new(0));
	let lock2 = Arc::clone(&lock);
	// Our clone produces a new Arc instance which points to
	// the same allocation on the heap s the source Arc.

	let _ = std::thread::spawn(move || -> () {
		let _guard = lock2.lock().unwrap();
		// We acquire the lock here !
		panic!();	// -> mutex is now poisoned !!
	}).join();

	let mut guard = match lock.lock() {
		Ok(guard) => guard,
		Err(poisoned) => poisoned.into_inner(),
	};

	*guard += 1;
	println!("{:?}", guard);	// -> 1

	// thread '<unnamed>' panicked at 'explicit panic', src/main.rs:18:9
	// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

	// So it's important to have this match statement running,
	// in the eventuality that you think one of your threads
	// might panic while holding the lock

}
