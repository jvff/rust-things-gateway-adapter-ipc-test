extern crate nanomsg;

use std::io::Read;

use nanomsg::{Protocol, Socket};

fn main() {
    let mut socket = Socket::new(Protocol::Rep).expect("Failed to open socket");
    let mut endpoint = socket
        .bind("ipc:///tmp/gateway.addonManager")
        .expect("Failed to bind socket");
    let mut string = String::new();

    loop {
        socket
            .read_to_string(&mut string)
            .expect("Failed to read from socket");
        println!("{}", string);
        string.clear();
    }

    endpoint.shutdown();
}
