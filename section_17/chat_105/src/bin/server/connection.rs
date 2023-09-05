use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::Arc;
use chat::utils::{self, CharResult};
use chat::utils::{Client, Server};

pub async fn handle(socket: TcpStream, chats: Arc<ChatTracker>) -> ChatResult<()> {

}