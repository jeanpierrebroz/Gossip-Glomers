use crate::node::Node;
use crate::protocol::Protocol;
use serde::{Deserialize, Serialize};
use std::io::BufRead;
use std::sync::mpsc::Sender;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T> {
    pub src: String,
    pub dest: String,
    pub body: Body<T>,
}

impl Message<Protocol> {
    pub fn reply(&self, contents: Protocol, msg_id: usize) -> Message<Protocol> {
        Message {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body: Body {
                msg_id: msg_id,
                in_reply_to: Some(self.body.msg_id),
                contents,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<T> {
    pub msg_id: usize,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub contents: T,
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

        let msg: Message<Protocol> = parse(input);

        assert_eq!(msg.src, "c1");
        match msg.body.contents {
            Protocol::Init { ref node_id, .. } => assert_eq!(node_id, "n1"),
            _ => panic!("Expected Init variant"),
        }
    }

    #[test]
    fn test_parse_valid_read() {
        //real message from maelstrom
        let input = r#"{
            "src": "c1",
            "dest": "n1",
            "body": {
                "type": "read_ok",
                "messages": [1, 2, 3],
                "msg_id": 1
            }
        }"#;

        let msg: Message<Protocol> = parse(input);

        assert_eq!(msg.src, "c1");
        let correct: Vec<usize> = vec![1, 2, 3];
        match msg.body.contents {
            Protocol::ReadOk { ref messages, .. } => assert_eq!(messages, &Some(correct)),
            _ => panic!("Expected Init variant"),
        }
    }

    #[test]
    fn test_parse_valid_read_kv() {
        //real message from maelstrom
        let input = r#"{
            "src": "seq-kv",
            "dest": "n1",
            "body": {
                "type": "read_ok",
                "value": 1234,
                "msg_id": 1
            }
        }"#;

        let msg: Message<Protocol> = parse(input);

        assert_eq!(msg.src, "seq-kv");
        match msg.body.contents {
            Protocol::ReadOk {
                value: Some(ref value),
                ..
            } => {
                let count = value.as_u64().expect("value should be a number") as usize;
                assert_eq!(count, 1234);
            }
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
        let _: Message<Protocol> = parse(input);
    }

    #[test]
    #[should_panic]
    fn test_parse_malformed_json_panics() {
        let input = r#"{"src": "broken", "body": "not_an_object"}"#;
        let _: Message<Protocol> = parse(input);
    }
}

pub trait HandleMessage {
    type Message: serde::de::DeserializeOwned + Serialize + Send + 'static;
    fn handle(&mut self, node: Node);
}

pub fn parse<T>(message: &str) -> Message<T>
where
    T: serde::de::DeserializeOwned,
{
    match serde_json::from_str::<Message<T>>(message) {
        Ok(msg) => msg,
        Err(e) => panic!("Parsing failed: {}. Input was: {}", e, message),
    }
}

//user takes
