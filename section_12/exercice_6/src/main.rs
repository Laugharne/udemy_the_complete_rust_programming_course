/*

Create a vector with the values 1, 3, 5, 7, and 9.
Then use an iterator and a closure to multiply all
of the values by 10 and store the result in another vector.
Print out the vector to confirm your results.

*/


fn main() {
	let vec_in: Vec<i32> = vec![1,3,5,7,9];
	let vec_out: Vec<i32> = (&vec_in).into_iter().map(|x| x*10).collect();

	println!("{:?}", vec_out);	// -> [10, 30, 50, 70, 90]
}
