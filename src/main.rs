extern crate nanomsg;
#[macro_use]
extern crate serde;
extern crate serde_json;

use std::collections::HashSet;
use std::io::{Read, Write};
use std::thread;

use nanomsg::{Protocol, Socket};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "messageType", content = "data")]
pub enum PluginRegistrationRequest {
    #[serde(rename_all = "camelCase")]
    RegisterPlugin { plugin_id: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "messageType", content = "data")]
pub enum PluginRegistrationReply {
    #[serde(rename_all = "camelCase")]
    RegisterPluginReply {
        plugin_id: String,
        ipc_base_addr: String,
    },
}

fn main() {
    let mut plugins = HashSet::new();
    let mut socket = Socket::new(Protocol::Rep).expect("Failed to open socket");
    let mut endpoint = socket
        .bind("ipc:///tmp/gateway.addonManager")
        .expect("Failed to bind socket");
    let mut message = String::new();

    loop {
        socket
            .read_to_string(&mut message)
            .expect("Failed to read from socket");

        let request: PluginRegistrationRequest =
            serde_json::from_str(&message).expect("Failed to deserialize response");
        let PluginRegistrationRequest::RegisterPlugin { plugin_id } = request;
        let ipc_base_addr = format!("gateway.plugin.{}", plugin_id);
        let plugin_socket_address = format!("ipc:///tmp/{}", ipc_base_addr);

        plugins.insert(plugin_id.clone());

        thread::spawn(|| handle_plugin(plugin_socket_address));

        let reply = PluginRegistrationReply::RegisterPluginReply {
            plugin_id,
            ipc_base_addr,
        };

        let reply_message = serde_json::to_string(&reply).expect("Failed to send reply");
        socket.write_all(reply_message.as_bytes());

        message.clear();
    }

    endpoint.shutdown();
}

fn handle_plugin(socket_address: String) {
    let mut socket = Socket::new(Protocol::Pair).expect("Failed to open socket for plugin");
    let mut endpoint = socket
        .bind(&socket_address)
        .expect("Failed to bind plugin socket");
    let mut message = String::new();

    loop {
        socket
            .read_to_string(&mut message)
            .expect("Failed to read from plugin socket");
        println!("Plugin sent: {}", message);
        message.clear();
    }

    endpoint.shutdown();
}
