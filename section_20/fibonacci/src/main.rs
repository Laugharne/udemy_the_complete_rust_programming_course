fn fact( num: i32) -> i32 {
	if num > 1 {
		return num * fact(num-1);
	} else {
		return 1;
	}
}


fn fib( num: i32) -> i32 {
	if num == 0 {
		0
	} else if num == 1 {
		1
	} else {
		let n1 = fib(num-1);
		let n2 = fib(num-2);
		n1+n2
	}

}


fn main() {
	let f = fact(5);
	println!("{}", f);	// -> 120
	println!("{}", fib(5));	// -> 5
	println!("{}", fib(15));	// -> 610
}
