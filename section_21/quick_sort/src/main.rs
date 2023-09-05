fn main() {
	let mut array = vec![8,5,1,2,7,3,4];
	let len = array.len();
	quick_sort(&mut array, 0, len-1);
	println!("{:?}", array);	// -> [1, 2, 3, 4, 5, 7, 8]
}


fn quick_sort(arr: &mut [i32], start: usize, end: usize) -> Vec<i32> {
	if start < end {
		let part = partition(arr, start, end);
		quick_sort(arr, start, part-1);
		quick_sort(arr, part+1, end);
	}

	arr.to_vec()
}


fn partition(arr: &mut [i32], start: usize, end: usize) -> usize {
	let mut i = start;
	let pivot = end;

	for j in start..=end-1 {
		if arr[j] < arr[pivot] {
			arr.swap(i, j);
			i += 1;
		}		
	}

	arr.swap(i, pivot);
	i
}