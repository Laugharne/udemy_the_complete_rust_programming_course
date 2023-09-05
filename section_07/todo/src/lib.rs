mod list {	// module list

	pub struct Tasks {
		pub item: String,
	}

/* -> move to things_todo.rs
	pub mod things_todo {
		pub fn add_activity() {}
		fn update_activity() {}
		fn marked_completed() {}
	}	// additional module
*/

/* -> move to /things_todo/items_completed.rs
	mod items_completed {
		fn remove_task() {}
		fn move_back_todo() {}
	}	// additional module
}
*/
// we have two modules (things_todo, items_completed)
// inside our module


mod things_todo;
use crate::things_todo::add_activity;

use things_todo::items_completed;	//+
use things_todo::items_completed::test::test;	//+

fn lets_add_tasks() {
	let task = list::Tasks {item: String::from("Tasks")};
	// compilation error cause "item" of struct "Tasks" is a private field
	// we need to add pub behind it to make it also public

//	list::things_todo::add_activity();	// relative path
	// "things_todo" is private
	//		pub mod things_todo{}
	//		pub fn add_activity(){}

//	crate::list::things_todo::add_activity();	// absolute path
												// becauuse we start at the root crate


//	things_todo::add_activity();	// relative path
//	crate::things_todo::add_activity();	// absolute path

	add_activity();	// relative path

	items_completed::remove_task();	//+
	test();	//+
}