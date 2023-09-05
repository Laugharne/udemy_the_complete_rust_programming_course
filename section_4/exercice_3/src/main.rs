struct Car {
	mpg      : u32,
	color    : String,
	top_speed: u32,
}

impl Car {

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
	println!("{}", car.mpg);
	println!("{}", car.color);
	println!("{}", car.top_speed);
}
