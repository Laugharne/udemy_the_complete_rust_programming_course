macro_rules! gcd {
	($a: expr, $b: expr) => {
		{
			let mut m = $b;
			let mut n = $a;

			while m != 0 {
				if m < n {
					let t = m;
					m = n;
					n = t;
				}
				m = m % n;
			}
			n
		}
	};
}
fn main() {
	println!("{}", gcd!(14,15));	// -> 1
	println!("{}", gcd!(14,16));	// -> 2
	println!("{}", gcd!(14,28));	// -> 14
}

/*

So when we call gcd or compile this macro, i should say,
and then run it when we compile it, it's going to write
this code into the binary for us. 
Which is, as i said, meta programming

*/
