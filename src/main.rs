extern crate nanomsg;
#[macro_use]
extern crate serde;
extern crate serde_json;

use std::collections::HashSet;
use std::io::Read;

use nanomsg::{Protocol, Socket};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "messageType", content = "data")]
pub enum PluginRegistrationRequest {
    RegisterPlugin { plugin_id: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "messageType", content = "data")]
pub enum PluginRegistrationReply {
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

        plugins.insert(plugin_id.clone());

        let reply = PluginRegistrationReply::RegisterPluginReply {
            plugin_id,
            ipc_base_addr,
        };

        serde_json::to_writer(&mut socket, &reply).expect("Failed to send reply");

        message.clear();
    }

    endpoint.shutdown();
}
