#[derive(Debug)]
struct City {
	city: String,
	population: u64,
}

/*
fn sort_pop(city: &mut Vec<City>) {
	city.sort_by_key(pop_helper);
}

fn pop_helper(pop: &City) -> u64 {
	pop.population
}
*/

fn sort_pop_closure(pop: &mut Vec<City>) {
	pop.sort_by_key(|p| p.population);
}

fn main() {
	let a = City{city: String::from("A"), population: 100};
	let b = City{city: String::from("B"), population: 57};
	let c = City{city: String::from("C"), population: 140};
	let d = City{city: String::from("D"), population: 15};
	let e = City{city: String::from("E"), population: 70};

	let mut vec: Vec<City> = Vec::new();
	vec.push(a);
	vec.push(b);
	vec.push(c);
	vec.push(d);
	vec.push(e);

	sort_pop_closure( &mut vec);	// sort by ascending population
//	sort_pop( &mut vec);	// sort by ascending population
	println!("{:?}", vec);	// -> [City { city: "D", population: 15 }, City { city: "B", population: 57 }, City { city: "E", population: 70 }, City { city: "A", population: 100 }, City { city: "C", population: 140 }]

	let add = |x: i32| -> i32 { x+1};

	let add_v2 = |x| x+1;
	add_v2(1);	// like this the compiler is able to infer the type (i32)

	let example = |x| x;
	let string = example(String::from("string"));	// the compiler is able to infer the type (String)
	//let num = example(2);
	// 46 |     let num = example(2);
	//    |               ------- ^- help: try using a conversion method: `.to_string()`
	//    |               |       |
	//    |               |       expected struct `String`, found integer
	//    |               arguments to this function are incorrect




}
