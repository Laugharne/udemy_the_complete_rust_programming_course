use std::rc::Rc;
use std::cell::RefCell;

struct Flagger {
	is_true: RefCell<bool>,
}

struct FlaggerRc {
	is_true: Rc<RefCell<bool>>,
}

fn main() {
	let t = (12, "eggs");
	let b = Box::new(t);
	println!("{:?}", b);	// -> (12, "eggs")

	// Pointer also come with a deference operator

	let x = 5;
	let y = &x;

	assert_eq!(5, x);
	assert_eq!(5, *y);

	// Now with a Box<T>
	let x = 5;
	let y = Box::new(x);

	assert_eq!(5, x);
	assert_eq!(5, *y);

	println!("{:?}", y);
	println!("{:?}", *y);

	let s1 = Rc::new(String::from("Pointer"));
	let s2 = s1.clone();
	let s3 = s2.clone();

	println!("{} {} {}", s1, s2, s3);	// -> Pointer Pointer Pointer
	println!("{} {} {}", s1.contains("Point"), s2, s3.contains("ter"));	// -> true Pointer true

	let flag = Flagger{is_true: RefCell::new(true)};
	// borrow() returns Ref<T>
	// borrow_mut() returns RefMut<T>

	let reference = flag.is_true.borrow();
	println!("{}", reference);	// -> true

	// let's do a mutable borrow
	let flag2 = Flagger{is_true: RefCell::new(true)};
	let mut mut_ref = flag2.is_true.borrow_mut();
	*mut_ref = false;	// dereference first to access inside
	println!("{}", mut_ref);	// -> false
	// so we're able to mutability change an ummutable value
	// So we cannot have multiple borrows out uless we pair it with an Rc<T>

	let flag3 = FlaggerRc{is_true: Rc::new( RefCell::new(true))};

	let reference3 = Rc::new(flag3.is_true.clone());
	println!("{:?}", reference3);	// -> RefCell { value: true }

	//let mut mut_ref = flag3.is_true.borrow_mut();
	let mut mut_ref = reference3.borrow_mut();
	// both previous lines have the same results
	*mut_ref = false;	// dereference first to access inside
	println!("{}", mut_ref);	// -> false
	println!("{:?}", reference3);	// -> RefCell { value: <borrowed> }


}
