use async_std::{fs::File, io, prelude::*, task};

// We want file IO, Prelude and task,
// cause we’re going to do an asynchronous read
// from a file and then assign the contents of
// the file to a mutable string variable and then
// we want to output it to the terminal.

//  async keyword, it’s signify that this function is asynchronous.
async fn read_file(path: &str) -> io::Result<String> {
	let mut file = File::open(path).await?;
	let mut contents = String::new();

	file.read_to_string(&mut contents).await?;
	Ok(contents)
}


fn main() {
	let task = task::spawn( async {
		let result = read_file("read.txt").await;
		match result {
			Ok(k) => println!("{}", k),
			Err(e) => println!("Error reading from file : {}", e),
		}
	});
	// spawn() takes a future and starts running.
	// and we can see that it returns a joint handle,
	// just like we saw in our concurrency section.
	//
	// So a task is very similar to a thread with
	// some very minor differences.
	//
	// The task will be scheduled by the program
	// and if the task has to wait, the program is
	// responsible to waking it back up
	//
	// We have also an async block inside of our spawn
	// and it is necessary to have an async blank block
	// in order to call async functions.
	//
	// and then just like in threads, we can call a
	// blocking function "read_file()"
	// to allow for the computation to finish.

	println!("Task has started!");
	task::block_on(task);
	println!("Stopped task!");

/*
So task are one of the core abstractions in async
Like threads, they provide some practical functionality
over the raw concept

Tasks have a number of desirable properties such as
being allocated in one single allocation

They have backchannel which allows them to propagate
results and errors to the spawning task through the joint handle.

They also carry useful metadata for debugging and
they support task local storage.

The task type and our async library is very similar to a thread.
Which is great for us since we are allready familiar with threads
and as you can tell, kind of by the structure of our program here.

So in order for us to really understand asynchronous programming
and how it works and just get an overall better feel for it,
let's create a clien/server chat application.


*/

/*

Task has started!
File to read from!
Stopped task!

*/

}

/*
trait Future {
	type Output;
	fn poll(self: Pin<&mut self>, cx: &mut Context) -> Poll<Self::Output> {

	}
	// poll() will allow usto check on the
	// state of the current computation.

	// When a computation is done, poll()
	// will return poll::ready
	//
	// And when a computation is not done, poll()
	// will return poll::pending

}
*/