use std::collections::LinkedList;

pub struct Queue<T> {
	element: LinkedList<T>,
}

impl<T> Queue<T> {
	pub fn new() -> Queue<T> {
		Queue { element: LinkedList::new(),}
	}

	pub fn enqueue(&mut self, value: T) {
		self.element.push_back( value)
	}

	pub fn dequeue(&mut self) -> Option<T> {
		self.element.pop_front()
	}

	pub fn peek(&self) -> Option<&T> {
		self.element.front()
	}

	pub fn length(&self) -> usize {
		self.element.len()
	}

	pub fn is_empty(&self) -> bool {
		self.element.is_empty()
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_enqueue() {
		let mut que = Queue::new();
		que.enqueue(1);
		que.enqueue(2);
		assert_eq!(que.is_empty(), false);
	}
	#[test]
	fn test_dequeue() {
		let mut que = Queue::new();
		que.enqueue(1);
		que.enqueue(2);
		assert_eq!(que.dequeue(), Some(1));
	}

	#[test]
	fn test_peek() {
		let mut que = Queue::new();
		que.enqueue(1);
		que.enqueue(2);
		assert_eq!(que.peek(), Some(&1));
	}

	#[test]
	fn test_length() {
		let mut que = Queue::new();
		que.enqueue(1);
		que.enqueue(2);
		assert_eq!(que.length(), 2);
	}
}