pub mod envelope; 
pub mod client;
pub mod kv;

use serde::{Deserialize, Serialize};

pub use envelope::Envelope;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InternalBody<B> {
    Init { node_id: String},
    InitOk, 
    Error{ code: u16, text: String},
    
Read { key: usize, msg_id: usize},
ReadOk { value: usize, in_reply_to: usize},

#[serde(untagged)]
User(B)
}