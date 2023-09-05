
struct User {
	active: bool,
	username: String,
	sign_in_account: u32,
}

struct Coordinates(i32, i32, i32);

struct UnitStruct;


struct Square {
	width: u32,
	height: u32,
}

impl Square {
	fn area( &self) -> u32 {
		self.width * self.height
	}

	fn whats_my_width( &self) -> u32 {
		self.width
	}

	// !!! &mut self  not just  &self !!!
	fn change_width( &mut self, new_width: u32) {
		self.width = new_width;
	}
}


struct MyString<'a> {
	text: &'a str,
}
// we have to do this because when we create an
// instance of MyString, which let'ss just go ahead



fn main() {
	let user1 = User{active: true, username: String::from("Tyler"), sign_in_account: 0};
	println!("{}", user1.username);

	let user2 = build_user("Tyler2".to_string());
	//	"Tyler2".to_string()
	//	String::from("Tyler2")
	println!("{}", user2.username);

	let cords = Coordinates(1,2,3);

	// 1..5 for a range
	// dot dot ( .. ) is a shorthand for the struct value range.
	// 1..5, .. Range {start: 1; end: 5}


	let sq = Square{width: 5, height: 5};
	println!("{}", sq.area());
	println!("{}", sq.whats_my_width());

	// !!! new sq variable must be mut cause we modify it !!!
	let mut sq = Square{width: 5, height: 5};
	sq.change_width(10);
	println!("{}", sq.whats_my_width());

	// // lifetime
	// let r;
	// {
	// 	let x = 5 ;
	// 	r = &x;
	// }// x is dropped

	// println!("{}", r); //'a

	// &i32
	// &'a i32
	// &'a mut i32


	let str1 = String::from("This is my string");
	let x = MyString{text: str1.as_str()};
	// we going to set it as a string slice
	// we explicitly declare the lifetime because
	// we want to make sure that the instance of MyString
	// cannot outlive the reference it holds in its text field,
	// which in case is str1 because if MyString lives longer
	// than the string we created here, which is what we passed in as a 

	let s: &'static str = "I have a static lifetime";


}

// what we can see here is the lifetime annotations.
// Indicate that the reference x must both must live
// as long as the generic lifetime.
// So we can see that all the function,
// the functions using the same lifetime, annotation,
// the parameters using it, and also the string slice.
//
// So the function signatures going to tell us that
// for some lifetime of 'a the function takes
// parameter x which is a string slice and it must live.
// As at least as long as the lifetime of 'a.
//
// So when we're returning a reference from a function,
// the lifetime parameter for the return type needs to
// match the lifetime for one of the parameters.
fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
	y
} // 'a for one parameter, 'b for second parameter


fn build_user(username: String) -> User {
	User {
		username,
	//	username: username,
		active: true,
		sign_in_account: 1,
	}
}