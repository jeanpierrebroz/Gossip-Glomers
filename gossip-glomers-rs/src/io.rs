use crate::node::Node;
use crate::protocol::Protocol;
use serde::{Deserialize, Serialize};

pub trait Handler {
    fn handle(&mut self, msg: Message<Protocol>, node: &Node);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message<T> {
    pub src: String,
    pub dest: String,
    pub body: Body<T>,
}

impl Message<Protocol> {
    pub fn reply(self, contents: Protocol, msg_id: usize) -> Message<Protocol> {
        Message {
            src: self.dest,
            dest: self.src,
            body: Body {
                msg_id,
                in_reply_to: Some(self.body.msg_id),
                contents,
            },
        }
    }

    pub fn into_reply(
        self,
        msg_id: usize,
    ) -> (impl FnOnce(Protocol) -> Message<Protocol>, Protocol) {
        let contents = self.body.contents;
        let in_reply_to = self.body.msg_id;
        let build = move |reply_contents| Message {
            src: self.dest,
            dest: self.src,
            body: Body {
                msg_id,
                in_reply_to: Some(in_reply_to),
                contents: reply_contents,
            },
        };
        (build, contents)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Body<T> {
    pub msg_id: usize,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub contents: T,
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