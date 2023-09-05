fn bubble_sort(array: &mut Vec<i32>) -> Vec<i32> {
	let mut sorted = true;

	for _ in 1..array.len() - 1 {
		sorted = true;
		for j in 0..=array.len()-2 {
			if array[j] > array[j+1] {
				array.swap(j, j+1);
				sorted = false;
			}
		}
		if sorted {
			break;
		}
	}
	array.to_vec()
}

fn main() {
	let mut vec1 = vec![5,4,1,3,2];
	bubble_sort(&mut vec1);
	println!("{:?}", vec1);

	let mut vec2 = vec![5,1,4,2,8];
	bubble_sort(&mut vec2);
	println!("{:?}", vec2);
}
