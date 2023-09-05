fn fact( num: i32) -> i32 {
	if num > 1 {
		return num * fact(num-1);
	} else {
		return 1;
	}
}

fn main() {
	let f = fact(5);
	println!("{}", f);	// -> 120
}
