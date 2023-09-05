use std::error::Error;
use async_std::prelude::*;
use serde::Serialize;
use std::marker::Unpin;

use serde::de::DeserializeOwned;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

// The question mark operator (point d'intérogation)
// will automatically be able to convert different
// err types into a chat err by using the
// standard libraries from trait to the type we implemented (ChatError)
//
// We use send and sync bounds to ensure that if a task that is spawned
// on another thread fails, it will be able to safely report the failure
// up to main.
// And if it doesn't, you'll be able to see it a little bit more as we
// continue our program.


// So the function we are going to create is going to create a
// JSON representation of our packet as a string.
// Then we will add a new line to the end of the string and write it all to leaving.

// where clause
pub async fn send_json<O, P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
	O : async_std::io::Write+Unpin,
	P: Serialize
// We have aour O generic which must implement Write and Unpin
// And our P generic which is must implement serialize
{
	// So we can turn our packet to a string
	// and then write it to our leaving argument.
	let mut json = serde_json::to_string(&packet)?;
	json.push('\n');
	leaving.write_all(json.as_bytes()).await?;
	Ok(())
}

// So the function we are going to implement is going
// to be relatively similar to our send JSON function.

// One generic type is going to implement the asynchronous buff, read and Unpin
// Our other generic type is going t need to make sure the serialized owned is
// implemented, which is the reason we are doing this is.
// So that our serialized values being returned to our caller a/k/a the client,
// need to outlive the buffer we are getting our messages from..
// So serialized own is always going to be independent from the buffer,
// which means we will satisfy that requirement.

pub fn receive<I, T>(incoming: I) -> imlpl Stream<Item = ChatResult<T>>
where
	I: async_std::io::BufRead + Unpin,
	T: DeserializeOwned,
{
	incoming.lines().map(|line| -> ChatResult<T> {
		let li = line?;
		let msg = serde_json::from_str::<T>(&li)?;
		Ok(msg)
	})
}

// incoming.lines() is giving us a stream of string values.
// We then map these values anf propagate the errors if needed.
// and assuming there are no errors, we will then get the string
// form back from the JSON value.
// From that we get a stream of chat result values, which is why
// our return type is a stream of chat results.
// " Stream<Item = ChatResult<T>> "

// The function itself is not really asynchronous.
// The only thing asychronous about it, is the fact that
// we return a stream which is asynchronous.