use std::{collections::HashMap, thread};
use tokio::sync::oneshot;
use serde_json::Value;
use crate::protocol::envelope::Envelope;

pub struct Node {
    node_id: String,
    //keep track of outbound messages by msg id
    outbound_messages: HashMap<usize, Envelope<Value>>
}

pub fn run() {
    //handle Init + InitOk
    thread::spawn(move || {
                    loop {
                        thread::sleep(Duration::from_millis(500));
                        let _ = outbound_clone.send(Envelope {
                            src: node_id_clone.clone(),
                            dest: node_id_clone.clone(),
                            body: Message::Tick {},
                        });
                    }
                });
}