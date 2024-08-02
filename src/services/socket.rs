
use std::env;
use rust_socketio::{asynchronous::{Client,ClientBuilder}};
use rust_socketio::error::Error as SocketError;

pub struct SocketService {
    pub socket:Client
}

impl SocketService {
   pub async fn init() ->Result<SocketService,SocketError>{
        //loading the env variables
        let socket_uri=env::var("SOCKET_URI").expect("failed to read the env variable");
        //connecting to the socket server
        let socket=ClientBuilder::new(socket_uri).namespace("/").connect().await?;
        println!("connected to socket server");
        return Ok(Self {
            socket,
        });
    }
}
