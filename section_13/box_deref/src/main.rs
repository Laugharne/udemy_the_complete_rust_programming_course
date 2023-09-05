fn main() {
	// Tupple
	//	created on the stack !
	let t = (12, "eggs");

	// it create a reference aka a pointer to t
	//	Created on the heap, but b was stored on the stack !
	let b = Box::new(t);

	println!("{:?}", b);	// -> (12, "eggs")

	// Pointer also come with a deference operator

	let x = 5;
	let y = &x;

	assert_eq!(5, x);
	//assert_eq!(5, y);
	// doesn't work, cause we can't compare an integer
	// with a reference to an integer
	//    |
	// 18 |     assert_eq!(5, y);
	//    |     ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
	//    |
	//    = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
	//    = help: the following other types implement trait `PartialEq<Rhs>`:
	// 			 f32
	// 			 f64
	// 			 i128
	// 			 i16
	// 			 i32
	// 			 i64
	// 			 i8
	// 			 isize
	// 		   and 6 others
	//    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

	assert_eq!(5, *y);

	// Now with a Box<T>
	let x = 5;
	let y = Box::new(x);

	assert_eq!(5, x);
	assert_eq!(5, *y);

	println!("{:?}", y);
	println!("{:?}", *y);

}
