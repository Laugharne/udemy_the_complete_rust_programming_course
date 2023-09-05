#[derive(Debug)]
struct Item {
	name: String,
}

fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item> {
	items.into_iter().filter(|i| i.name == product).collect()
	// - into_iter() Creates a consuming iterator, that is,
	//	one that moves each value out of the vector (from start to end).
	// - filter() Creates an iterator which uses a closure to determine
	//	if an element should be yielded. And filter item with the same
	//	name as the product name.
	// - collect() transform an iterator into a collection.
}



#[derive(Debug)]
struct Range {
	start: u32,
	end  : u32,
}

impl Iterator for Range {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		if self.start >= self.end {
			return None;
		}
		let result = Some(self.start);
		self.start += 1;
		result
	}
}



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

	//	v is a Vector !
	//	|| drop(v)               -> FnOnce
	//	|args| v.contains(args)  -> Fn
	//	|args| v.push(args)      -> FnMut

	/*
	- The first example is going to be "FnOnce" because
	  we can only drop the value once.
	- The second example is going to implement "Fn" because
	  all we're doing is checking to see if the vector contains
	  the parameter that was passed in.  There's nothing
	  required in this that needs to be mutable.
	- The third example is "FnMut" because the vector would
	  need to be mutable in order for us to push the argument into the vector.
	*/

/*

	// IT WORKS

	let y: i32     = 5;
	let add_y = |x| x + y;
	let copy = add_y;		// this is closure being copied
	println!("{}", add_y( copy( 10)));
*/

/*
	// Example where neither copy or clone works...

	let mut y: i32     = 5;
	let mut add_y = |x| {y += x; y};
	let mut copy = add_y;		// this is closure being copied
	println!("{}", add_y( copy( 10)));
	// This does not work because mutable values do not implement copy or clone.
	*/


	let vec = vec![1,2,3];

	for val in vec.iter() {
		println!("{}", val);
	}

	println!("");

	let vec2 = vec![1,2,3];
	let mut iter = (&vec2).into_iter();

	while let Some(v) = iter.next() {
		println!("{}", v);	// -> exactly the same previously output
	}

	// An iterator is any type that implements the iterator trait
	// and iterable is any type that implements into_iterator.


	let mut vec: Vec<Item> = Vec::new();
	vec.push(Item {
		name: String::from("coat"),
	});
	vec.push(Item {
		name: String::from("shirt"),
	});
	vec.push(Item {
		name: String::from("shorts"),
	});
	vec.push(Item {
		name: String::from("shoes"),
	});

	let checked = check_inventory(vec, String::from("shirt"));
	println!("{:?}", checked);
	/*
	So we use into_iter() to create an iterator that's going
	to take ownership of the vector and then we cal filter
	to adapt that iterator into a new iterator that only
	contains elements for which the closure returns true.

	The closure capture the product parameter from the
	environment and it's going to compare the value with
	each item we created and only keeps the item we specified,
	which in this case was a "shirt"

	And we call collect() to gather the values returned by
	the adapted iterator into a vector that will be returned from the function.

	What if we wanted to create our own iterator ?
	*/

	let mut range = Range{start: 0, end: 10}	;
	// for r in range {
	// 	println!("{}", r);
	// }

	let vec: Vec<u32> = range.filter(|x| x % 2 == 0).collect();
	println!("{:?}", vec);	// -> [0, 2, 4, 6, 8]


}


/*
pub trait Iterator {
	type Item;
	fn next(&mut self) ->Option<Self::Item>;
	// many default methods
}
*/
