//	use std::thread;
use std::sync::mpsc;	// Multi Procedure Single Consumer
						// FIFO queue communication primitives.

fn main() {

	// So now, let's look at how we can create multi producers

	let (transmitter, receiver) = mpsc::sync_channel(1000);
	//let (transmitter, receiver) = mpsc::channel();
	let tx = transmitter.clone();

	std::thread::spawn(move || {
		let vec = vec![
			String::from("Transmitting"),
			String::from("From"),
			String::from("Original"),
		];

		for val in vec {
			transmitter.send(val).unwrap();
		}
	});

	std::thread::spawn(move || {
		let vec = vec![
			String::from("Clone"),
			String::from("Is"),
			String::from("Transmitting"),
		];

		for val in vec {
			tx.send(val).unwrap();
		}
	});

	// TWO threads spawned
	//	our 1st producer is going to transmit from original.
	//	our 2nd one is clone his transmitting.

	// So now we want to set our receiver up to be able
	// to receive all of these messages
	for rec in receiver {
		println!("{}", rec); 
	}
	/*
		Transmitting
		From
		Original
		Clone
		Is
		Transmitting
	*/

	// we see transmitting from original and now we see clone is transmitting
	// We had multiple producers and one single consumer.

}
