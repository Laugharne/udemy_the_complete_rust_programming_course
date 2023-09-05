use std::rc::Rc;

fn main() {
	let t = (12, "eggs");
	let b = Box::new(t);
	println!("{:?}", b);	// -> (12, "eggs")

	// Pointer also come with a deference operator

	let x = 5;
	let y = &x;

	assert_eq!(5, x);
	assert_eq!(5, *y);

	// Now with a Box<T>
	let x = 5;
	let y = Box::new(x);

	assert_eq!(5, x);
	assert_eq!(5, *y);

	println!("{:?}", y);
	println!("{:?}", *y);

	let s1 = Rc::new(String::from("Pointer"));
	let s2 = s1.clone();
	let s3 = s2.clone();

	println!("{} {} {}", s1, s2, s3);	// -> Pointer Pointer Pointer
	// Cloning an Rc does not copy the value
	// In this example, the string pointer has a reference count of three
	// and s1, s2 and s3 which are all on the stack, all point to the
	// string that is located on the heap.

	println!("{} {} {}", s1.contains("Point"), s2, s3.contains("ter"));	// -> true Pointer true
	// Our data being a string, we are still able to use
	// String methods as usual directly.

}
