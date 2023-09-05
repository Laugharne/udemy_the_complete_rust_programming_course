use std::thread;
// 2	use std::time::Duration;

fn main() {
	// std::thread::spawn(move || {
	// 	println!("Hello from a thread!");
	// });
	// nothing (or rarely) happen,
	// cause the main thread is actually finishing
	// before our thread spawnw and is able to print 
	// out our message.

	// 2	thread::sleep(Duration::from_secs(1));	// now it works !

	let handle = std::thread::spawn(move || {
		println!("Hello from a thread!");
	});

	handle.join().unwrap();
	// Si la valeur de Result est la variante Ok,
	// unwrap va retourner la valeur contenue dans le Ok.

	// join is a form of blocking.
	// So we're going to block our main thread
	// until our join handle the we assign to handle.
	// is finished executing, and that is what join() is going to do for us.

	println!("Hello from main!");

}
