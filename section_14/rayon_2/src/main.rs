use rayon::prelude::*;
use num::{BigUint, One};
use std::time::Instant;	// top measure the time of functions execution

fn factorial(num: u32) -> BigUint {
	if num == 0 || num == 1 {
		return BigUint::one()
	} else {
		(1..=num).map(BigUint::from).reduce(|acc, x| acc*x).unwrap()
		// 1 to num
		// inclusive -> '='
		// the reduce() function is a closur with 2 args
		//		arg : accumulator
		//		x   : is the element we're passing in (1..=num).map(BigUint::from)
		//	it takes accumulator and multiply it by x, then now is our new accumulator

	}
}


fn multi_fact(num: u32) -> BigUint {
	if num == 0 || num == 1 {
		return BigUint::one()
	} else {
		(1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(), |acc, x| acc*x)
	}
}

fn main() {
	let now = Instant::now();
	factorial(50000);
	println!("{:.2?}", now.elapsed());	// -> 6.35s

	let now = Instant::now();
	multi_fact(4);
	println!("{:.2?}", now.elapsed());	// -> 1.30ms
}
