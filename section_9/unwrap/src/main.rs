use std::fs::File;
use::std::io::ErrorKind;

fn main() {

	// let file = File::open("error.txt").unwrap();

	// What appen in unwrap() is if Ok() is returned from the method, then will return the value inside of Ok() for us.


	// BUT if the result is an error, unwrap will call panic for us.
	// So since we no longer have Err; i except this to panic force, which it does.


	let file = File::open("error.txt").expect("Error opening the file!");


}
