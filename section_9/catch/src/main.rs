use std::fs::File;
use::std::io::ErrorKind;

fn main() {

	let file = File::open("error.txt");
	// it return -> io::Result<File>

	// let file = match file {
	// 	Ok(file) => file,
	// 	Err(error) => panic!("Error: {:?}", error),
	// };

//	warning: unused variable: `file`
//	--> src/main.rs:8:6
//	  |
//	8 |     let file = match file {
//	  |         ^^^^ help: if this is intentional, prefix it with an underscore: `_file`
//	  |
//	  = note: `#[warn(unused_variables)]` on by default
//
//	warning: `catch` (bin "catch") generated 1 warning
//		Finished dev [unoptimized + debuginfo] target(s) in 0.20s
//		 Running `target/debug/catch`
//	thread 'main' panicked at 'Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:10:23

	let file = match file {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("error.txt") {
				Ok(file_created) => file_created,
				Err(err) => panic!("Cannot create the file !"),
			}
			_ => panic!("It was some other error kind !"),
		},
	};


}

/*
enum Result<T, E> {
	Ok(T),
	Err(E),
}
*/