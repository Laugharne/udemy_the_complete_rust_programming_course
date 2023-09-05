use std::fs::File;

fn main() {

	let file = File::open("error.txt");
	// it return -> io::Result<File>

}

/*
enum Result<T, E> {
	Ok(T),
	Err(E),
}
*/