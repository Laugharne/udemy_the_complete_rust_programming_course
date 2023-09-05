use std::collections::HashMap;

fn main() {
	let mut hm = HashMap::new();

	hm.insert(1,1);
	hm.insert(5,2);
	hm.insert(30,3);
	println!("{:?}", hm);	// -> {1: 1, 5: 2, 30: 3}

	let old = hm.insert(30,4);
	println!("{:?}", old);	// -> Some(3)
	println!("{:?}", hm);	// -> {1: 1, 5: 2, 30: 4}

	println!("{}", hm.contains_key(&5));	// -> true
	println!("{}", hm.contains_key(&8));	// -> false

	println!("{:?}", hm);	// -> {1: 1, 5: 2, 30: 4}

	println!("{:?}", hm.get(&5));	// -> Some(2)

	let one = hm.remove(&1);
	println!("{:?}", one);	// -> Some(1)
	println!("{:?}", hm);	// -> {5: 2, 30: 4}

	let remove = hm.remove_entry(&5);
	println!("{:?}", remove);	// -> Some(5,2)
	println!("{:?}", hm);		// -> {30: 4}

	hm.clear();
	println!("{}", hm.is_empty());		// true
	println!("{:?}", hm);		// -> {}

}
