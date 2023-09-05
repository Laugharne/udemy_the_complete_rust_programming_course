type Link<T> = Option<Box<Node<T>>>;


pub struct List<T> {
	head: Link<T>,
}


struct Node <T> {
	element: T,
	next   : Link<T>,
}


pub struct IntoIter<T>(List<T>);
pub struct Iter<'a, T> {
	next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
	next: Option<&'a mut Node<T>>,
}



impl<T> List<T> {
	pub fn new() -> Self {
		List { head: None}
	}

	pub fn push(&mut self, element: T) {
		let new_node = Box::new(Node {
			element: element,
			next   : self.head.take(),
		});

		self.head = Some(new_node);
	}

	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|node| {
			self.head = node.next;
			node.element
		})
	}

	pub fn peek(&self) -> Option<&T> {
		self.head.as_ref().map(|node| {	// as_ref Converts from &Option<T> 
														// to Option<&T>
			&node.element
		})
	}

	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node| &mut node.element)
	}

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}

	pub fn iter(&self) -> Iter<T> {
		Iter {
			next: self.head.as_deref()
		}
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		IterMut {
			next: self.head.as_deref_mut()
		}
	}

}


impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

impl<'a, T>  Iterator for Iter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.next.as_deref();
			&node.element
		})
	}
}


impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.next.as_deref_mut();
			&mut node.element
		})
	}
}


// The reason we need to implement the drop trait for our list
// is because the drop would be done recursively
// the recursion could potentially blow the stack up and that is really bad.
// So we need to an iterative drop to avoid using the recursive drop.
impl<T> Drop for List<T> {
	fn drop(&mut self) {
		let mut link = self.head.take();
		while let Some(mut boxed_node) = link {
			link = boxed_node.next.take();
		}
	}
}


mod test {
	use super::*;

	#[test]
	fn basics() {
		let mut list = List::new();

		// Check if empty list behaves correctly
		assert_eq!(list.pop(), None);

		//Populate List
		list.push(1);
		list.push(2);
		list.push(3);

		// Check normal removal
		assert_eq!(list.pop(), Some(3));
		assert_eq!(list.pop(), Some(2));

		// Push some more values on
		list.push(4);
		list.push(5);

		assert_eq!(list.pop(), Some(5));
		assert_eq!(list.pop(), Some(4));

		// Exhaustive removal check
		assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), None);
	}

	#[test]
	fn peek( ) {
		let mut list = List::new();
		assert_eq!(list.peek(), None);
		assert_eq!(list.peek_mut(), None);

		list.push(1);
		list.push(2);
		list.push(3);
		list.push(4);
		list.push(5);

		assert_eq!(list.peek(), Some(&5));
		assert_eq!(list.peek_mut(), Some(&mut 5));

		list.peek_mut().map(|value| {
			*value = 42
		});

		assert_eq!(list.peek(), Some(&42));
		assert_eq!(list.pop(), Some(42));
	}

	#[test]
	fn iter() {
		let mut list = List::new();
		list.push(1);
		list.push(2);
		list.push(3);

		let mut iter = list.iter();
		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next(), Some(&1));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn iter_mut() {
		let mut list = List::new();

		list.push(1);
		list.push(2);
		list.push(3);

		let mut iter = list.iter_mut();
		assert_eq!(iter.next(), Some(&mut 3));
		assert_eq!(iter.next(), Some(&mut 2));
		assert_eq!(iter.next(), Some(&mut 1));

	}
}