// struct Point<T> {
// 	x: T,	//i32
// 	y: T,	//i32
// }

// now we are able to have two different types in here...
struct Point<T,U> {
	x: T,	//i32
	y: U,	//&str
}


trait Overview {
	fn overview(&self) -> String {
		String::from("This is a Rust course!")
	}
}

struct Course {
	headline: String,
	author  : String,
}


// We imlplement Drop for Course
impl Drop for Course {
	fn drop(&mut self) {
		println!("Dropping: {}", self.author)
	}
}

struct AnotherCourse {
	headline: String,
	author  : String,
}

impl Overview for Course {}
// impl Overview for Course {
// 		fn overview(&self) -> String {
// 		format!("{} - {}", self.author, self.headline)
// 	}
// }

impl Overview for AnotherCourse {
	fn overview(&self) -> String {
		format!("{} - {}", self.author, self.headline)
	}
}


use std::ops::Add;

#[derive(Debug)]
struct PointFT<T> {
	x: T,
	y: T,
}

impl<T> Add for PointFT<T>
	where	// where  is going to restrict T to types
			// that can be added to themselves yielding another T value.
			// just saying like we want to add a float to a float already
	T: Add<Output = T> {
		type Output = Self;
		fn add(self, rhs: Self) -> Self {
			PointFT {
				x: self.x + rhs.x,
				y: self.y + rhs.y,
			}
		}
	}

fn main() {
	let coord  = Point{x: 5.0, y: 5.0};
	let coord2 = Point{x: 'x', y: 'y'};
	let coord3 = Point{x: 'x', y: "5.0"};
	let coord4 = Point{x: 'x', y: 5.0};

	let course1 = Course{headline: String::from("Headline!"), author: String::from("Tyler")};
	let course2 = AnotherCourse{headline: String::from("Another Headline!"), author: String::from("Another Tyler")};

	println!("{}", course1.overview());
	println!("{}", course2.overview());

	call_overview(&course1);
	call_overview(&course2);

	// We implement Drop for Course
	// we manually call this drop trait
	// and we manually freed up our resources,

	//drop(course1); 

	// if we delete the previous line and rerun,
	// we see that it still calls dropping.
	// because here is where course1 went out of the scope
	// and so it called our implementation of drop


	let coordFT = PointFT{x: 5.0, y: 5.0};
	let coordFT2 = PointFT{x: 1.0, y: 2.0};

	let sum = coordFT +coordFT2;
	println!("{:?}", sum);

} // course1 is out of scope and it called our implementation of drop


trait Clone: Sized {
	fn clone(&self) -> Self;
	fn clone_from(&mut self, source: &Self) {
		*self = source.clone()
	}
}


//fn call_overview(item: &impl Overview) {
// SAME EXACT THING !!
fn call_overview<T: Overview>(item: &T) {
		println!("Overview: {}", item.overview());
}

// fn overview(item1: &impl Overview, item2: &impl Overview)
//
// So this allows item1 and item2 to be of different types,
// but we can force them to be of the same type if we want it to.

// fn overview<T: Overview>(item1: &T, item2: &T)
//
//This one we they have to be the same type.

// fn overview(item1: &impl Overview + AnotherTrait)
//
// So "+ AnotherTrait" alloww us to have multiple trait bounds.

// fn overview<T: Overview + AnotherTrait>(item1: &T, item2: &T)
//
// 


