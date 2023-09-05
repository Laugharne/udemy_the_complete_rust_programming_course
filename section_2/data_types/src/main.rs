fn main() {
	let x: i8 = 10;
	println!("{}", x);

	let _y: u8 = 10;
	//println!("{}", y);

	let decimal = 02_55;
	let hex     = 0xff;
	let octal   = 0o377;
	let binary  = 0b1111_1111;

	println!("{}", decimal);
	println!("{}", hex);
	println!("{}", octal);
	println!("{}", binary);

	let byte = b'A';
	println!("{}", byte);

	let _f = 2.0;	// f64 default because on modern CPUs
	let _s: f32 = 1.0;

	let _on  = true;
	let _off = false;

	let c = 'c';
	println!("{}", c);

	// + - * %

	let a = 10;
	let b = 4;

	let remainder = a % b;
	println!("{}", remainder);

	let tup = (500, "hi", true);
	println!("{}", tup.0);
	println!("{}", tup.1);
	println!("{}", tup.2);

	let (x, y, z) = tup;
	println!("{}", x);
	println!("{}", y);
	println!("{}", z);

	let array = [1,2,3];
	println!("{}", array[0]);
	println!("{}", array[1]);
	println!("{}", array[2]);

	let mut array2: [i32; 3] = [4,5,6];
	println!("{}", array2[0]);
	array2[0] = 10;
	println!("{}", array2[0]);

	let mut nums = vec![1,2,3];
	nums.push(4);

	//	println!("{}", nums);	// `Vec<{integer}>` cannot be formatted with the default formatter
	println!("{:?}", nums);	// let nums = vec![1,2,3]	consider changing this to be mutable: `mut nums`
	nums.pop();
	println!("{:?}", nums);

	let mut vec = Vec::new();	// 2nd way to create Vector
											// we create empty Vector
	vec.push("Test");
	vec.push("String");
	println!("{:?}", vec);

	vec.reverse();
	println!("{:?}", vec);

	let mut vect = Vec::<i32>::with_capacity(2);
	println!("{}", vect.capacity());

	let v: Vec<i32> = (0..5).collect();
	println!("{:?}", v);

	// A slice is a region of an array or vector that can be any link.
	//Slices cannot be stored directly into variables or passed as function arguments.
	
	let sv: &[i32] = &v;

	// Reference to a slice is a fat pointer, wich is a two word value compising a pointer
	// of the slices first element and the number of elements in the slice
	println!("{:?}", sv);

	let sv: &[i32] = &v[2..4];
	println!("{:?}", sv);

	// References are an ordinary reference is a non owning pointer to a signle value,
	// but a reference to a slice is a non owning pointer to a range of consecutives values.

	// so what are sliced references good for ?
	// well, when you want to write a function that operate on either an array or a vector, that is when you want to use a slice.

	let name = String::from("Tyler");
	let course = "Rust".to_string();

	// just like Vectors, we can modify Strings...
	let new_name = name.replace("Tyler", "Ty");
	println!("{}", name);
	println!("{}", course);
	println!("{}", new_name);

	// &str = "string slice" or "stir"
	// You cannot modify a string slice
	let str1 = "hello";	// &str
	println!("{}", str1);
	// A string slice is more appropriate for function arguments when the caller should be allowed to pass either kind of string.
	// And remember a string slice does not allocate memory on the heap where a string do

	let str2 = str1.to_string();
	let str3 = &str2;
	println!("{}", str1);
	println!("{}", str2);
	println!("{}", str3);

	// compare strings == != 
	println!("{}", "ONE".to_lowercase() == "one");

	let rust = "\x52\x75\x73\x74";
	println!("{}", rust);

}
