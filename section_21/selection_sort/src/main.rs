
fn selection_sort(array: &mut Vec<i8>) -> Vec<i8> {
	// for i in 0..array.len() {
	// 	print!("[{}]:{} ", i, array[i]);
	// }
	// println!("");

	for i in 0..array.len() - 1 {
		let mut smallest = i;
		for j in (i+1)..array.len() {
			if array[j] < array[smallest] {
				smallest = j;
			}
		}
		array.swap(smallest, i);
	}
	array.to_vec()	// copy self into a new Vec
}

fn main() {
	let mut array: Vec<i8> = vec![4,3,2,1];

	println!("Before sorting: {:?}", array);
	selection_sort(&mut array);
	println!("After sorting : {:?}", array);	

	let mut array: Vec<i8> = vec![5,10,1,4,11];

	println!("Before sorting: {:?}", array);
	selection_sort(&mut array);
	println!("After sorting : {:?}", array);	

}

