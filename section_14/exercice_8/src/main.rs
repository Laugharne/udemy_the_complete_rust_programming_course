use rayon::prelude::*;
use std::time::{Duration, Instant};

fn fib_recursive(n: u32) -> u32 {
	if n < 2 {
		return n;
	}

	fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn fibonacci_join(n: u32) -> u32 {
	if n < 2 {
		return n;
	}

	let (a, b) = rayon::join(
		|| fibonacci_join( n-1),
		|| fibonacci_join( n-2),
	);
	a + b
}

fn main() {
	let start = Instant::now();
	let x = fib_recursive(10);
	let duration = start.elapsed();
	println!("Recursive fibonacci answer: {}, time taken: {:?}", x, duration);

	println!("Now run with Rayon's join.");

	let start = Instant::now();
	let x = fibonacci_join(10);
	let duration = start.elapsed();
	println!("Rayon fibonacci answer: {}, time taken: {:?}", x, duration);
}
