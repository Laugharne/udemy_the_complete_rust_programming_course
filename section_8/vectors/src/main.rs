use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
	let mut nums: Vec<i32> = vec![];

	nums.push(1);
	nums.push(2);
	nums.push(3);

	let pop: Option<i32> = nums.pop();	// return the last item
										// pushed on this stack
	// it returns an Option<T>
	// Return Non or Some(T)
	// in this case T = i32

	println!("{:?}", pop);	// -> Some(3)
	println!("{:?}", nums);	// -> [1,2]

	let two: i32 = nums[1];	// copy
	// &nums[1], create a reference if copy is not available

	println!("{:?}", two);	// 2
	println!("{:?}", nums);	// -> [1,2]

	let one: Option<&i32> = nums.first();	// return an Option<T>,
											// so None if vec is empty
											// or Some(T) is nums[0]
	println!("{:?}", one);	// Some(1)
	println!("{:?}", nums);	// -> [1,2]

	// .last()
	// .first_mut()
	// .last_mut()
	//	And as the name suggests, it's going to be the same as
	// first() and last(), but it's going to borrow mutable references

	println!("{}", nums.len());			// -> 2 value
	println!("{}", nums.is_empty());	// -> false (boolean)

	nums.insert(0, 10);
	println!("{:?}", nums);	// -> [10,1,2]
	nums.insert(3, 12);
	println!("{:?}", nums);	// -> [10,1,2,12]
	nums.insert(2, 25);
	println!("{:?}", nums);	// -> [10,1,25,2,12]

	nums.remove(3);
	println!("{:?}", nums);	// -> [10,1,25,12]

	nums.sort();
	println!("{:?}", nums);	// -> [

	nums.reverse();
	println!("{:?}", nums);	// -> [25,12,10,1]

	// after specifie dependencies section "Cargo.toml"
	//	rand = "0.8.4"
	nums.shuffle(&mut thread_rng());
	println!("{:?}", nums);

}
