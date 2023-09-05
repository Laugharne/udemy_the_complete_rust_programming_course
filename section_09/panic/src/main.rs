fn main() {
	//	panic!("panicked here !");
	//	thread 'main' panicked at 'panicked here !', src/main.rs:2:5
	//	note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

	let vec = vec![];
	vec[10];
// 	error[E0282]: type annotations needed for `Vec<T>`
// 	--> src/main.rs:6:6
// 	 |
//    6 |     let vec = vec![];
// 	 |         ^^^
// 	 |
//    help: consider giving `vec` an explicit type, where the type for type parameter `T` is specified
// 	 |
//    6 |     let vec: Vec<T> = vec![];
// 	 |            ++++++++
   
//    For more information about this error, try `rustc --explain E0282`.
   
	// we see that it says "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"

	// There is another behavior that panicking can have and
	// that is called an abort in the event of an abort.
	// Unwinding does not happen.

}
