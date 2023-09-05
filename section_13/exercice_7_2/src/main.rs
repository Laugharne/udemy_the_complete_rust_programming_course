use std::rc::Rc;

fn main() {
	//Question 2: Create a variable that holds a String
	let string = String::from("string");

	{
		//Create a reference counting smart pointer that points to the above String.
		let rc_smart_pointer = Rc::new( string);
		
		//Print out how many references the smart pointer has.
		println!("reference counter = {}", Rc::strong_count(&rc_smart_pointer));

		//Code block
		{
			//Create another reference counting smart pointer that points to our first smart pointer
			let another_smart_pointer = rc_smart_pointer.clone();

			//Print out how many references each smart pointer has
			println!("reference counter original ptr = {}", Rc::strong_count(&rc_smart_pointer));
			println!("reference counter another ptr  = {}", Rc::strong_count(&another_smart_pointer));
		}
		//What value is dropped here?
		//Print out how many references out first smart pointer has
		println!("reference counter original ptr = {}", Rc::strong_count(&rc_smart_pointer));


	} //What value is dropped here?
	//Comment out the line below. What do you think will happen when you try to run the program now?
	//println!("rc_value: {}", rc_value);
}
