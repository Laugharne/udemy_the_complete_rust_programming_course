fn main() {
	
	let var   = 1;                    // on the stack
	let mut s = "Hello".to_string();  // created on the heap
	s.push_str(", world");	// created on the heap, so allowed to grow !

	// let x = vec!["tyler".to_string()];
	// let y = x;
	// let z = y;
	// //println!("{:?}", x);	// value borrowed here after move
	// println!("{:?}", z);

	// let x = vec!["tyler".to_string()];
	// let y = x.clone();
	// let z = y.clone();

	// println!("{:?}", x);
	// println!("{:?}", y);
	// println!("{:?}", z);

	// let x = 1;
	// let y = x;
	// println!("x = {}, y = {}", x, y);
	// why it is work ?
	// i32 is implemented only with copy and not move
	// And a copy is going to be implemented on types
	// that are already stored on the stack such
	// an integer, float,or a tuple can also have the copy trait
	// if every value it contains implements copy.

	// let s = String::from("takes");	// create a variable with a string "takes"
	// takes_ownership( s);	// we give ownership to the function 
	// //println!("{}", s);	// we loose the ownership with s !

	// let val = 1;
	// make_copy( val);

	// let str1: String = give_ownership();
	// println!("{}", str1);

	// let str3: String = take_and_give(str1);
	//println!("{}", str1);	// we loose the ownership with str1 !

	// if(true) {
	// 	let str4 = str3;
	// } else {
	// 	let str5 = str3;
	// }

	// println!("{}", str3);

	// let mut str1: String::from("Tyler");
	// let mut str2: String;

	// loop {
	// 	str2 = str1;	// this is an infinite loop
	// 					// ownership is loose from a previous itertation of the loop
	// }

	let mut s = String::from("hello");
	change_string( &mut s);
	println!("{}", s);	// -> "Hello world!"
	// So after running cargo run, we were able to see that
	// we change the variable without ever taking ownership
	// of the variable inside the function.
	// Because if we didn't pass it by reference,
	// then change string would have taken ownership of s

}


fn change_string(some_string: &mut String) {
	some_string.push_str(", world!");
}


fn takes_ownership( s: String) {
	let strin = s;
	println!("{}", strin);
}


// we make a ccopy here because i32 implements the copy trait.
fn make_copy( one: i32) {
	let val1 = one;
	println!("{}", val1);
}


fn give_ownership() -> String {
	"given".to_string()
}


fn take_and_give( str2: String) -> String {
	str2
}


// var is dropped, s is also dropped
