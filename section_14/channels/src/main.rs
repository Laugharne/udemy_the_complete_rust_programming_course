//	use std::thread;
use std::sync::mpsc;	// Multi Procedure Single Consumer
						// FIFO queue communication primitives.

fn main() {

	let (transmitter, receiver) = mpsc::channel();

	let val = String::from("Transmitting!");
	std::thread::spawn(move || {
		transmitter.send(val).unwrap();
	});

	let msg = receiver.recv().unwrap();
	println!("{}", msg);	// -> "Transmitting!"

	//println!("{}", val);	// -> "Transmitting!"
	//    |
	// 9  |     let val = String::from("Transmitting!");
	//    |         --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
	// 10 |     std::thread::spawn(move || {
	//    |                        ------- value moved into closure here
	// 11 |         transmitter.send(val).unwrap();
	//    |                          --- variable moved due to use in closure
	// ...
	// 16 |     println!("{}", val);    // -> "Transmitting!"
	//    |                    ^^^ value borrowed here after move
	//    |
	//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)



	// So now, let's look at how we can create multi producers

}
