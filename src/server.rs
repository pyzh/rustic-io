// Copyright (c) 2014 Nathan Sizemore

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.


use std::str;

/*
 * Struct representing a WebSocket server
 */
pub struct Server {
    pub ip: String,
    pub port: String,
    pub events: Vec<Event>
}

impl Server {

    /*
     * Constructs a new Server
     */
    pub fn new(ip_addr: &str, port: &str) -> Server {
        return Server {
            ip: String::from_str(ip_addr),
            port: String::from_str(port),
            events: Vec::new()
        }
    }

    /*
     * Adds the passed event and function pointer to end of the events vector
     */
    pub fn on(&mut self, event_name: &str, execute: fn(data: Json, socket: Socket)) {
        self.events.push(Event::new(event_name, execute));
    }
}

/*
 * Returns a clone of the current server
 */
impl Clone for Server {
    fn clone(&self) -> Server {
        Server {
            ip: self.ip.clone(),
            port: self.port.clone(),
            events: self.events.clone()
        }
    }
}