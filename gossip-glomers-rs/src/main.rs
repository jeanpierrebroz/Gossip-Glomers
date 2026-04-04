use serde::{Deserialize, Serialize};
use std::io::BufRead;

use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{Sender, channel};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MessageType {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
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

fn read<R: BufRead>(reader: R, sender: Sender<String>) {
    for line in reader.lines() {
        match line {
            Ok(s) => {
                let _ = sender.send(s);
                //handle mapping types in the receiver
            }

            Err(s) => {
                panic!("Something went horribly wrong reading input: {}", s);
            }
        }
    }
}

//next tests: pass in user-defined enum and ensure correct types are sent through receiver. ensure it panics if unknown type is encountered

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_init() {
        //real message from maelstrom
        let input = r#"{
            "src": "c1",
            "dest": "n1",
            "body": {
                "type": "init",
                "msg_id": 1,
                "node_id": "n1",
                "node_ids": ["n1", "n2"]
            }
        }"#;

        let msg: Message<MessageType> = parse(input);

        assert_eq!(msg.src, "c1");
        match msg.body.contents {
            MessageType::Init { ref node_id, .. } => assert_eq!(node_id, "n1"),
            _ => panic!("Expected Init variant"),
        }
    }

    #[test]
    #[should_panic]
    fn test_parse_unknown_type_panics() {
        //unhandled type
        let input = r#"{
            "src": "c1",
            "dest": "n1",
            "body": {
                "type": "calculate",
                "msg_id": 1
            }
        }"#;

        //this should trigger the panic inside the parse function
        let _: Message<MessageType> = parse(input);
    }

    #[test]
    #[should_panic]
    fn test_parse_malformed_json_panics() {
        let input = r#"{"src": "broken", "body": "not_an_object"}"#;
        let _: Message<MessageType> = parse(input);
    }
}

struct Node {
    id: Option<usize>,
    node_ids: Option<Vec<String>>,
    msg_counter: AtomicUsize,
}

fn parse<T>(message: &str) -> Message<T>
where
    T: serde::de::DeserializeOwned,
{
    match serde_json::from_str::<Message<T>>(message) {
        Ok(msg) => msg,
        Err(e) => panic!("Parsing failed: {}. Input was: {}", e, message),
    }
}


// TODO: create node struct
// TODO: pass unhandled system types to user-defined handler
// TODO: auto-set the old msg_id to be in reply to the message they're replying to