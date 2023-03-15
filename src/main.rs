use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use tungstenite::{connect, Message};
use url::Url;
#[macro_use]
extern crate log;

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();

    tokio::spawn(async move {
        server().await;
    });
    client();
}

async fn server() {
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();

    for stream in server.incoming() {
        tokio::spawn(accept_connection(stream.unwrap()));
    }
}

async fn accept_connection(stream: TcpStream) {
    let callback = |req: &Request, mut response: Response| {
        debug!("Received a new ws handshake");
        debug!("The request's path is: {}", req.uri().path());
        debug!("The request's headers are:");
        for (ref header, _value) in req.headers() {
            debug!("* {}: {:?}", header, _value);
        }

        let headers = response.headers_mut();
        headers.append("MyCustomHeader", ":)".parse().unwrap());

        Ok(response)
    };
    let mut websocket = accept_hdr(stream, callback).unwrap();

    loop {
        let msg = websocket.read_message().unwrap();
        if msg.is_binary() || msg.is_text() {
            websocket.write_message(msg).unwrap();
        }
    }
}

fn client() {
    let (mut socket, response) =
        connect(Url::parse("ws://localhost:3012/socket").unwrap()).expect("Can't connect");
    debug!("Connected to the server");
    debug!("Response HTTP code: {}", response.status());
    debug!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        debug!("* {}: {:?}", header, _value);
    }

    socket
        .write_message(Message::Text("Hello WebSocket".into()))
        .unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        debug!("Received: {}", msg);
    }
}
