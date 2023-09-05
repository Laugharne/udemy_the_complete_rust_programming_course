fn main() {
	
	let mut val = vec![1,3,5,7];
	let r = check_vec( &val);
	//println!("{}", r);
	val.push(15);
	println!("{:?}", val);

	let mut integer: i8 = 1;
	add_two( &mut integer);
	println!("{}", integer);

}


fn check_vec( val: &Vec<i32>) -> bool {
	val[0] == 1
}


fn add_two( val: &mut i8) {
	*val = *val + 2;
}