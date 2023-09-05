use std::fs::File;
use std::io::ErrorKind;
use std::fs::rename;
use std::io::Error;

fn main() {

//	let test = open_file();
//	test.unwrap();

	rename_file().unwrap();

}


fn open_file() -> Result<File, Error> {
	let file = File::open("error.txt")?;
	Ok(file)
}


fn rename_file() ->Result<(), Error> {
	let file = rename("error.txt", "renamed.txt")?;
	Ok(file)
}