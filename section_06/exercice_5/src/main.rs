#[derive(Debug)]
struct Car {
	mpg      : u32,
	color    : String,
	top_speed: u32,
}


#[derive(Debug)]
struct Motorcycle {
	mpg      : u32,
	color    : String,
	top_speed: u32,
}



pub trait Properties {
	fn set_mpg( &mut self, default: u32);
	fn set_color( &mut self, default: String);
	fn set_top_speed( &mut self, default: u32);
}


impl Properties for Car {
	fn set_mpg( &mut self, default: u32) {
		self.mpg = default;
	}

	fn set_color( &mut self, default: String) {
		self.color = default;
	}

	fn set_top_speed( &mut self, default: u32) {
		self.top_speed = default;
	}

}


impl Properties for Motorcycle {
	fn set_mpg( &mut self, default: u32) {
		self.mpg = default;
	}

	fn set_color( &mut self, default: String) {
		self.color = default;
	}

	fn set_top_speed( &mut self, default: u32) {
		self.top_speed = default;
	}

}


fn main() {
	let mut car = Car{mpg: 3, color: String::from("red"), top_speed: 160};
	car.set_mpg(4);
	car.set_color(String::from("green"));
	car.set_top_speed(180);
	println!("{:?}", car);

	let mut cycle = Motorcycle{mpg: 2, color: String::from("red"), top_speed: 170};
	cycle.set_mpg(3);
	cycle.set_color(String::from("yellow"));
	cycle.set_top_speed(190);
	println!("{:?}", cycle);

	print(vec![1,2,3]);
	print(3.0);
	print(String::from("Test"));
}


fn print<T: std::fmt::Debug>(val: T) {
	println!("{:?}", val);
}