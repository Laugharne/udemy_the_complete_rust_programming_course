/*

Create a macro called "op" that takes three arguments.
All three arguments will be integers,
but the third argument is going to be how we tell the macro
what operation to perform.

Operations:

1 is add, 2 is subtract, 3 is multiply, 4 is divide, and 5 is mod.



	println!("{}", op!(5,2,1)); //should print 7

	println!("{}", op!(5,2,2)); //should print 3

	println!("{}", op!(5,2,3)); //should print 10

	println!("{}", op!(5,2,4)); //should print 2

	println!("{}", op!(5,2,5)); //should print 1

	println!("{}", op!(5,2,6)); //should print -1
*/


macro_rules! op {
	($a: expr, $b: expr, $op: expr) => {
		{
			let op = $op;
			let a  = $a;
			let b  = $b;
			match op {
				1 => a+b,
				2 => a-b,
				3 => a*b,
				4 => a/b,
				5 => a%b,
				_ => -1,
			}
		}
	};
}
fn main() {
	println!("{}", op!(5,2,1)); //should print 7
	println!("{}", op!(5,2,2)); //should print 3
	println!("{}", op!(5,2,3)); //should print 10
	println!("{}", op!(5,2,4)); //should print 2
	println!("{}", op!(5,2,5)); //should print 1
	println!("{}", op!(5,2,6)); //should print -1
}
