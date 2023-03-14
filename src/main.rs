use std::{net::TcpListener, thread::spawn};
use tungstenite::{connect, Message};
use url::Url;
#[macro_use]
extern crate log;
use tungstenite_rs::{make_answer, show_streams, AnswerFn, HelperAttr};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};
mod try_vec;
use try_vec::try_vec;

make_answer!();

// Example: Basic function
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

// Example: Attribute with input
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

// Example: Multiple tokens in the input
#[show_streams(multiple => tokens)]
fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

// Example:
#[show_streams { delimiters }]
fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"

#[derive(AnswerFn)]
struct Struct;

#[derive(HelperAttr, Debug)]
struct Struct1 {
    #[helper]
    field: (),
}

fn main() {
    try_vec();

    invoke2();
    invoke3();
    invoke4();
    let st = Struct1 { field: () };
    println!("{}:{}:{:?}", answer1(), answer2(), st,);
    env_logger::builder().format_timestamp(None).init();

    spawn(|| {
        server();
    });
    client();
}

fn server() {
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    for stream in server.incoming() {
        spawn(move || {
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
