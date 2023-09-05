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


fn palindrom(array: &Vec<i32>, start: usize, end: usize) -> bool {
	if start >= end {
		return true
	}

	if array[start] == array[end] {
		return palindrom(array, start+1, end-1)
	} else {
		return false
	}
}


fn toh(n: i32) -> i32 {
	if n == 0 {
		return 0
	}

	return 2*toh(n-1) + 1
//	return toh(n-1) + 1 + toh(n-1)
}


fn main() {
	let f = fact(5);
	println!("{}", f);	// -> 120
	println!("{}", fib(5));	// -> 5
	println!("{}", fib(15));	// -> 610

	let array = vec![1,2,3,4];
	println!("{:?}", palindrom(&array, 0, array.len()-1));	// -> false

	let array = vec![1,2,3,4,3,2,1];
	println!("{:?}", palindrom(&array, 0, array.len()-1));	// -> true

	println!("{}", toh(0));	// -> 0
	println!("{}", toh(1));	// -> 1
	println!("{}", toh(2));	// -> 3
	println!("{}", toh(3));	// -> 7
	println!("{}", toh(4));	// -> 15
}
