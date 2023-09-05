fn fib(val: i32) -> i32 {
	let mut a = 0;
	let mut b = 1;
	let mut c: i32;

	for _ in 2..=val {
		c = a + b;
		a = b;
		b = c;
	}

	return b
}

pub fn lcs(a: &str, b: &str) -> String {
	let a: Vec<char> = a.chars().collect();	// store all chars into vector
	let b: Vec<char> = b.chars().collect();

	let (len_a, len_b) = (a.len(), b.len());

	let mut solution = vec![vec![0;len_b+1]; len_a+1];

	for (i, mi) in a.iter().enumerate() {
		for (j, mj) in b.iter().enumerate() {
			// if mi == mj, there is a new common char
			// otherwise, take the best of the 2 solutions
			// at (i-1, j) and (i, j-1)

			solution[i+1][j+1] = if mi == mj {
				solution[i][j]
			} else {
				solution[i][j+1].max( solution[i+1][j])
			}
		}
	}

	let mut result: Vec<char> = Vec::new();
	let (mut i, mut j) = (len_a, len_b);

	while i > 0 && j > 0 {
		if a[i-1] == b[j-1] {
			result.push(a[i-1]);
			i -= 1;
			j -= 1;
		} else if solution[i-1][j] > solution[i][j-1] {
			i -= 1;
		} else {
			j -= 1;
		}
	}

	result.reverse();
	result.iter().collect()

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fib() {
		assert_eq!( fib(9), 34);
	}

	#[test]
	fn test_lcs() {
		assert_eq!( lcs("", ""), "");
		assert_eq!( lcs("", "acbd"), "");
		assert_eq!( lcs("asdf", ""), "");

//		assert_eq!( lcs("abcd", "a"), "a");
		assert_eq!( lcs("adbc", "b"), "b");
//		assert_eq!( lcs("qwerty", "rty"), "rty");

//		assert_eq!( lcs("abcdgh", "aedfhr"), "adh");
//		assert_eq!( lcs("aggtab", "gxtxayb"), "gtab");

	}
}