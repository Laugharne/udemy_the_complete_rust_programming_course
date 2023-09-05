
use std::collections::BinaryHeap;

fn main() {
	let mut bheap = BinaryHeap::new();

	bheap.push(1);
	bheap.push(18);
	bheap.push(20);
	bheap.push(5);

	println!("{:?}", bheap);	// -> [20,5,18,1]

	bheap.pop();
	println!("{:?}", bheap);	// -> [18,5,1]

	// peak() is very similar to pop
	// but instead of the value being
	// removed from the binary heap
	// peak() is going to leave the
	// value there for us, but it's
	// still going to return tha value to us
	println!("{:?}", bheap.peek());	// -> Some(18)
	// peek() is goint to return an Option<T>,
	// return None if empty, or Some<T> otherwise
	println!("{:?}", bheap);	// -> [18,5,1]
								// 18 is still on the front

	// This collection is useful in cases where
	// we care about priority.

}
