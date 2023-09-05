enum Pet {
	Dog,
	Cat,
	Fish,
}


impl Pet {
	// we need to specify our lifetime annotation, static lifetime !
	fn what_am_i(self) -> &'static str {
		match self {
			Pet::Dog  => "I am a dog",
			Pet::Cat  => "I am a cat",
			Pet::Fish => "I am a fish",
		}
	}
}


enum IpAddrKind {
//	V4,
	V4(String),
	V6,
}


struct IpAddr {
	kind   : IpAddrKind,
	address: String,
}


fn main() {
	let dog = Pet::Dog;
	println!("{}", dog.what_am_i());


	// let home = IpAddr {
	// 	kind: IpAddrKind::V4,
	// 	address: String::from("127.0.0.1"),
	// };
	
	let home = IpAddrKind::V4(String::from("127.0.0.1"));

	
	let loopack = IpAddr {
		kind   : IpAddrKind::V6,
		address: String::from("::1"),
	};

	let some_number          = Some(5);
	let some_string          = Some("a string");
	let nothing: Option<i32> = None;              // Option<T>, let x = 5 == i32;

	let x: i32 = 5;
	let y: Option<i32> = Some(5);

	//	let sum = x + y;
	// not possible directly, does not work because we cannot had i32 to Option<i32>

	let five =Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	println!("{:?}", five);	// -> 5
	println!("{:?}", six);
	println!("{:?}", none);

	what_pet("Dog");
	what_pet("Cat");
	what_pet("Cow");

	let dog2 = Some(Pet::Dog);
	if let Some(Pet::Dog) = dog2 {
		println!("The animal is a dog!");
	} else {
		println!("Not a dog!");
	}

	let mut stack = Vec::new();
	stack.push(1);
	stack.push(2);
	stack.push(3);

	while let Some(top) = stack.pop() {
		println!("{}", top);
	}
	// 3 2 1

	let x = 1;

	match x {
		1 | 2 => println!("One or Twwo"),
		_ => println!("Not One or Two"),
	}

	match x {
	//	1..5 => println!("Matches"),
		1..=5 => println!("Matches"),
		   _  => println!("Not matching"),
	}

	let x = Some(5);
	let y = 5;

	match x {
		Some(10) => println!("10!"),
		Some(x) if x == y => println!("Matches!"),
		_ => println!("default!"),
	}

}


fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i+1),
	}

}


fn what_pet(input: &str) {
	match input {
		"Dog"  => println!("I have a dog!"),
		"Fish" => println!("I have a fish!"),
		"Cat"  => println!("I have a cat!"),
		_ => println!("I have no clue what pet you have"),
	}
}