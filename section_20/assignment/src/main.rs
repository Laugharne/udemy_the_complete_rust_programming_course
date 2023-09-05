fn triangle( array: &mut Vec<i8>, size: usize) {
	if size < 1 {
		return
	}

	let mut tmp: Vec<i8> = Vec::new();

	for i in 0..size-1 {
		let x = array[i] + array[i+1];
		tmp.push(x);
	}

	triangle(&mut tmp, size-1);
	println!("{:?}", array);

}


fn main() {
	let mut array = vec![1,2,3,4,5];
	let size = array.len();

	triangle(&mut array, size);


}
