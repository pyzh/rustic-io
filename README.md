rustic-io [<img src="https://travis-ci.org/nathansizemore/rustic-io.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io)
=========

rustic-io is a simple websocket server library written in Rust, inspired by socket.io.  It aims to be a easy to implement, fast, concurrent websocket server library for text and binary messages.

Borrows messaging implementation from [rust-ws](https://github.com/ehsanul/rust-ws)

#### Current State
Incomplete. JavaScript library is now available. Binary support coming soon.
* **What you can currently do**
  * JSON messaging between sockets
  * JSON broadcasting to all sockets

#### TODOs
* **HTTP Header**
  * Implement better parsing for incoming HTTP header
    * Right now, all it cares about is the Sec-WebSocket-Key field
* **Binary Messages**
  * Implement the shit

#### Example Usage

##### On the Server
```rust
#[deriving(Decodable, Encodable)]
pub struct Foo {
    msg: String
}

fn main() {
    let mut server = rustic_io::new_server("127.0.0.1", "1338");
    server.on("some_event", function_to_execute);
    rustic_io::start(server);
}

fn function_to_execute(data: json::Json, server: Server) {
    let json_object = data.as_object().unwrap();
    let msg = json_object.find(&String::from_str("msg")).unwrap().as_string().unwrap();
    server.broadcast("echo", json::encode(&Foo {
        msg: String::from_str(msg)
    }));
}
```

#### Example Projects
* [Echo Server](https://github.com/nathansizemore/rustic-io-echo-server) [<img src="https://travis-ci.org/nathansizemore/rustic-io-echo-server.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io-echo-server)
  