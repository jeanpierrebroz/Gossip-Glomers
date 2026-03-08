use std::{collections::HashMap, thread};
use tokio::sync::oneshot;
use serde_json::Value;
use crate::protocol::envelope::Envelope;

pub struct Node {
    node_id: String,
    outbound_messages: HashMap<usize, Envelope<Value>>
}

pub async fn run() {
    while let Some(msg) = stream.next_message().await? {
        let node = Arc::clone(&self);
    
        match msg.body {
            // 1. Check if this is a response to an RPC we sent
            InternalBody::ReadOk { value, in_reply_to } => {
                let mut room = self.waiting_room.lock().await;
                if let Some(tx) = room.remove(&in_reply_to) {
                    let _ = tx.send(InternalBody::ReadOk { value, in_reply_to });
                }
            }
    
            // 2. Handle Handshake
            InternalBody::Init { node_id } => {
                node.
            }
    
            // 3. Delegate User Logic
            InternalBody::User(user_body) => {
                let handler = Arc::clone(&self.handler);
                tokio::spawn(async move {
                    handler.handle_message(node, msg.replace_body(user_body)).await
                });
            }
            _ => {}
        }
    }
}