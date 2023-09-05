use std::thread;

fn main() {
	// let handle = std::thread::spawn(move || {
	// 	println!("Hello from a thread!");
	// });

	// handle.join().unwrap();

	// println!("Hello from main!");

	let v = vec![1, 2, 3];

	// let handle = std::thread::spawn(move || {
	// 	println!("{:?}", v);
	// });

	// IF WE DON'T USE move
	// 14 |           let handle = std::thread::spawn(|| {
	// 	  |  ______________________^
	// 15 | |             println!("{:?}", v);
	// 16 | |         });
	// 	  | |__________^
	//  help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
	// 	  |
	// 14 |         let handle = std::thread::spawn(move || {
	// 	  |                                         ++++

	// So now, let's look at how we can spawn multiple threads.

	let mut thread_handles = Vec::new();
	for e in v {
		println!("e : {}", e);
		thread_handles.push(thread::spawn(move || println!("Thread {}", e)));
	}

	println!("Main Thread !");

	for handle in thread_handles {
		handle.join().unwrap();
	}

}
