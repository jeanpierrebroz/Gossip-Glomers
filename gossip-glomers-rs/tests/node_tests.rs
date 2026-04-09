use gossip_glomers_rs::node::Node;
use gossip_glomers_rs::protocol::{Protocol, Protocol::Echo};

use gossip_glomers_rs::io::{Handler, Message};
struct EchoHandler {}

impl Handler for EchoHandler {
    fn handle(&mut self, msg: Message<Protocol>, node: &Node) {
        let copy = msg.clone();
        match msg.body.contents {
            Echo { echo } => {
                let reply = Protocol::EchoOk { echo: echo };
                let response = copy.reply(reply, node.get_next_msg_id());
                node.send(response);
            }
            _ => panic!("Unexpected Message Type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gossip_glomers_rs::io::{Body, Message};

    fn make_msg(contents: Protocol) -> Message<Protocol> {
        Message {
            src: "c1".to_string(),
            dest: "n1".to_string(),
            body: Body {
                msg_id: 1,
                in_reply_to: None,
                contents,
            },
        }
    }

    #[test]
    fn test_echo_reply_contents() {
        let msg = make_msg(Protocol::Echo {
            echo: "hello".to_string(),
        });
        let reply = msg.reply(
            Protocol::EchoOk {
                echo: "hello".to_string(),
            },
            2,
        );
        match reply.body.contents {
            Protocol::EchoOk { echo } => assert_eq!(echo, "hello"),
            _ => panic!("Expected EchoOk"),
        }
    }

    #[test]
    fn test_echo_reply_routing() {
        let msg = make_msg(Protocol::Echo {
            echo: "ping".to_string(),
        });
        let reply = msg.reply(
            Protocol::EchoOk {
                echo: "ping".to_string(),
            },
            2,
        );
        assert_eq!(reply.src, "n1");
        assert_eq!(reply.dest, "c1");
        assert_eq!(reply.body.in_reply_to, Some(1));
    }

    #[test]
    fn test_echo_preserves_content() {
        let msg = make_msg(Protocol::Echo {
            echo: "preserve me".to_string(),
        });
        let reply = msg.reply(
            Protocol::EchoOk {
                echo: "preserve me".to_string(),
            },
            2,
        );
        match reply.body.contents {
            Protocol::EchoOk { echo } => assert_eq!(echo, "preserve me"),
            _ => panic!("Expected EchoOk"),
        }
    }

    #[test]
    #[should_panic(expected = "Unexpected Message Type")]
    fn test_non_echo_panics() {
        let node = Node::new("n1".to_string(), vec![], Default::default());
        let mut handler = EchoHandler {};
        let msg = make_msg(Protocol::Generate);
        handler.handle(msg, &node);
    }
}
