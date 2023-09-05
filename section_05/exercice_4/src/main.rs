enum Shape {
	triangle,
	square,
	pentagon,
	octagon,
}

impl Shape {
	fn corners( &self) -> i32 {
		match self {
			Shape::triangle => 3,
			Shape::square   => 4,
			Shape::pentagon => 5,
			Shape::octagon  => 8,
		}
	}
}

fn main() {
	let triangle = Shape::triangle;
	let square = Shape::square;
	println!("{}", triangle.corners());
	println!("{}", square.corners());
}
