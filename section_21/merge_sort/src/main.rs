use std::array;

fn merge_sort(arr: &mut [i32]) -> Vec<i32> {
	if arr.len() > 1 {
		let mid = arr.len() / 2;
		merge_sort(&mut arr[..mid]);	// left half
		merge_sort(&mut arr[mid..]);	// right half
		merge(arr, mid);
	}
	arr.to_vec()
}

fn merge(arr: &mut [i32], mid: usize) {
	let left  = arr[..mid].to_vec();  // left half to middle
	let right = arr[mid..].to_vec();  // middle to right half

	let mut l = 0;
	let mut r = 0;

	for val in arr {
		if r == right.len() || (l < left.len() && left[l] < right[r]) {
			*val  = left[l];
			l    += 1;
		} else {
			*val  = right[r];
			r    += 1;
		}
	}
}


fn main() {
	let mut vec = vec![4,7,3,5,1,2];
	merge_sort(&mut vec);
	println!("{:?}", vec);	// -> [1, 2, 3, 4, 5, 7]
}
