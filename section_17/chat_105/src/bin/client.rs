use async_std::prelude::*;
use async_std::{io, net};
use chat_103::Server;
use chat_103::utils::{self, ChatResult};

use chat::{Server, Client};
use std::sync::Arc;
use async_std::task;

async fn send(mut send: net::TcpStream) -> ChatResult<()> {
	println!("Options: njoin CHAT\npost CHAT MESSAGE\nType Ctrl-D on Unix or Ctrl-Z on Windows to close the connection.");

	  // So we now we have that out the way, we're going
	  // to take the standard input and then wrap it it into
	  // a buffer reader so that our data is now contained inside a buffer.

	let mut options = io::BufReader::new(io::stdin()).lines();

	  // So we call lines that the users input is processed on a line by line
	  // basis.
	  // Since it is processed line by line, we can start setting up logic to parse
	  // each line and match it on a value in our client enum.
	  // We want our logic to send the value to the server if it's successfully matches.
	  //
	  // Otherwise, we want it to generate an error and continue in a loop waiting
	  // for the next command.

	while let Some(option_result) = option.next().await {
		let opt = option_result?;
		let req = match parse_input(&opt) {
			Some(req) => req,
			None      => continue,
		};

		utils::send_json(&mut send, &req).await?;
		send.flush().await?;
	}
	Ok(())

	// So we created a function called parse_input()
	// This function is going to allow us to get the
	// values our user passes in and then match on them
	// so that we know what we want our user to do.
}


async fn messages(server: net::TcpStream) -> ChatResult<()> {
	let buf = io::BufReader::new(server);
	let mut stream = utils::receive(buf);

	while let Some(msg) = stream.next().await {
		match msg? {
			Server::Message { chat_name, message } => {
				println!("Chat Name: {}\n Message: {}\n", chat_name, message);
			}
			Server::Error(message) => {
				println!("Error received: {}", message);
			}
		}
	}
	Ok(())
}


fn main() -> ChatResult<()> {
	let addr = std::env::args().nth(1).expect("Address:Port");

	task::block_on(async {
		let socket = net::TcpStream::connect(addr).await?;
		socket.set_nodelay(true)?;
		let send = send(socket.clone());
		let replies = messages(socket);
		replies.race(send).await?;
		Ok(())
	})
}




// We need a helper function that is going to give us
// our values and take out the white space accordingly.
// get_value()

fn get_value(mut input: &str) -> Option<&str, &str> {
	input = input.trim_start();  // leading whitespace removed.

	if input.is_empty() {
		return None;
	}

	match input.find(char::is_whitespace) {
		Some(whitespace) => Some((
			&input[0..whitespace],   // input from the begining to the white space
			&input[whitespace..],    // input from the white space to the end
		)),
		None => Some((input, "")),
	}
}
// So we are returning to String Slices inside our option
// because we want the command such as post message or
// join chat; which will be the first value we're going
// to get from the user.

fn parse_input(line: &str) -> Option<Client> {
	let (input, remainder)  = get_value(line)?;
	if input               == "join" {
		let (chat, remainder) = get_value(remainder)?;
		if !remainder.trim_start.is_empty() {
			return None;
		}
		return Some(Client::Join {
			chat_name: Arc::new(chat.to_string()),
		});
	} else if input == "post" {
		let (chat, remainder) = get_value(remainder)?;
		let message           = remainder.trim_start().to_string();
		return Some(Client::Post {
			chat_name: Arc::new(chat.to_string()),
			message  : Arc::new(message),
		});
	} else {
		println!("Unrecognized input: {:?} ", line);
		return None;
	}
}
