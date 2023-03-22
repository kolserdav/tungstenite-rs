use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{
        connect,
        handshake::server::{Request, Response},
        Message,
    },
    WebSocketStream,
};
use url::Url;
#[macro_use]
extern crate log;
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();

    tokio::spawn(async move {
        server().await;
    });
    client();
}

async fn server() {
    let server = TcpListener::bind("127.0.0.1:8081").await.unwrap();

    while let Ok((stream, _)) = server.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn owned_ws(ws: &mut WebSocketStream<TcpStream>, msg: Message) {
    ws.send(msg).await.unwrap();
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
    let mut ws_stream = accept_hdr_async(stream, callback)
        .await
        .expect("Error during the websocket handshake occurred");

    loop {
        info!("Waiting message...");
        let msg = ws_stream.next().await.unwrap();
        let msg = msg.unwrap();

        if msg.is_text() || msg.is_binary() {
            debug!("Server on message: {:?}", &msg);
            owned_ws(&mut ws_stream, msg).await;
        }
    }
}

fn client() {
    let (mut socket, response) =
        connect(Url::parse("ws://localhost:8081/socket").unwrap()).expect("Can't connect");
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
