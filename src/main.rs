use std::{net::TcpListener, thread::spawn};
use tungstenite::{connect, Message};
use url::Url;

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

fn main() {
    spawn(|| {
        server();
    });
    client();
}

fn server() {
    env_logger::init();
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}: {:?}", header, _value);
                }

                let headers = response.headers_mut();
                headers.append("MyCustomHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}

fn client() {
    let (mut socket, response) =
        connect(Url::parse("ws://localhost:3012/socket").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}: {:?}", header, _value);
    }

    socket
        .write_message(Message::Text("Hello WebSocket".into()))
        .unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
