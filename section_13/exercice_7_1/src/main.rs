fn main() {
	//Question 1:
	//	Create a variable on the stack and a variable on the heap.
	//	Multiply their values and print out the results.

	let var_stack = 5;
	let var_heap = Box::new(10);

	let result = var_stack * *var_heap;

	println!("Result = {}", result);
}
