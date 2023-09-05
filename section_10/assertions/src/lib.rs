pub fn add(left: usize, right: usize) -> usize {
	left + right
}



#[cfg(test)]
mod tests {
	use super::*;	// simple_add() is out of scope;
					// so we need to include this line
					// to proceed testing

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}

	#[test]
	//#[ignore]
	#[should_panic]
	fn it_fails() {
		panic!("Test failed !")
	}

	#[test]
	fn call_simple_add() {
		assert!( simple_add());
	}

}


fn simple_add() -> bool {
	if 2+2 == 4 {
		true
	} else {
		false
	}
}