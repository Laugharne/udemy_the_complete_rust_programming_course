use std::collections::HashSet;

fn main() {
	let mut hs = HashSet::new();

	// .len() -> value
	// .is_empty() -> boolean

	hs.insert(1);
	hs.insert(2);
	hs.insert(3);
	hs.insert(4);

	println!("");
	for x in hs.iter()	 {
		println!("Iter : {}", x);
	}

	hs.remove(&2);

	println!("");
	for x in hs.iter()	 {
		println!("Iter : {}", x);
	}

	let mut hs2 = HashSet::new();

	// .len() -> value
	// .is_empty() -> boolean

	hs2.insert(1);
	hs2.insert(3);
	hs2.insert(5);
	hs2.insert(7);

	println!("");
	for x in hs.intersection(&hs2) {
		println!("Intersection : {}", x);
	}

	let intersection = &hs & &hs2;
	// shorthand intersection using the binary bitwise & operator

	println!("");
	for x in intersection {
		println!("Short hand way : {}", x);
	}

	let union = &hs | &hs2;
	// shorthand unioon using the binary bitwise | operator

	println!("");
	for x in union {
		println!("Union : {}", x);
	}

	let diff = &hs - &hs2;

	println!("");
	for x in diff {
		println!("Diff : {}", x);
	}

}

