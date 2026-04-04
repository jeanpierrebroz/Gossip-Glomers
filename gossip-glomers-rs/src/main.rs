use serde::{Deserialize, Serialize};
use std::io::{BufRead, Stdout};

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{Sender, channel};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MessageType<T> {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    #[serde(untagged)]
    Custom(T)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T> {
    pub src: String,
    pub dest: String,
    pub body: Body<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<T> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub contents: T,
}


fn main() {
    let (sender, _recv) = channel();
    read(std::io::stdin().lock(), sender);
}



// TODO: create node struct
// TODO: pass unhandled system types to user-defined handler
// TODO: auto-set the old msg_id to be in reply to the message they're replying to