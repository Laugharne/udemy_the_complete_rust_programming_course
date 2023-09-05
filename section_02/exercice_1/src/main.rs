fn main() {
	let val1: i32 = 5;
	let val2: i32 = 2;
	let ans : i32 = val1 % val2;

	println!("{}", ans);

	let mut vec = vec![2, 4, 6, 8, 10];
	println!("{:?}", vec);
	vec.pop();
	vec.push(12);
	println!("{:?}", vec);

	let str = "Hello";
	let result = concat_string( str);
	println!("{}", result);

}


fn concat_string(str: &str) -> String {
	let str2 = "World";
	let concat = format!("{}{}",str,str2);
	concat
}


fn control_flow( arg: i32) {
	if arg == 1 {
		println!("The value is one");
	} else if arg < 25 {
		println!("The value is lesser than 25");
	} else if ( arg > 25) && ( arg < 50) {
		println!("The value is greater than 25 and lesser than 50");
	} else if  arg > 50 {
		println!("The value is greater than 50");
	}
}