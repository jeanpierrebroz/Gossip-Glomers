use gossip_glomers_rs::node::Node;
use gossip_glomers_rs::protocol::{Protocol, Protocol::Echo};
use gossip_glomers_rs::io::Body;

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

#[cfg(test)]
mod tests {
    use super::*;
    


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

#[cfg(test)]
mod retry_tests {
    use super::*;
    use std::time::Duration;
    use gossip_glomers_rs::node::RpcRetryConfig;

    fn make_node(timeout_ms: usize, max_retries: usize) -> Node {
        Node::new(
            "n1".to_string(),
            vec![],
            RpcRetryConfig {
                timeout_ms,
                max_retries,
                backoff_multiplier: 2,
            },
        )
    }

    #[test]
    fn test_send_adds_to_pending() {
        let node = make_node(1000, 3);
        let msg = make_msg(Protocol::Echo { echo: "hi".to_string() });
        node.send(msg);
        assert_eq!(node.pending_count(), 1);
    }

    #[test]
    fn test_ack_removes_from_pending() {
        let node = make_node(1000, 3);
        let msg = make_msg(Protocol::Echo { echo: "hi".to_string() });
        let msg_id = msg.body.msg_id;
        node.send(msg);
        assert_eq!(node.pending_count(), 1);
        node.ack(&msg_id);
        assert_eq!(node.pending_count(), 0);
    }

    #[test]
    fn test_ack_nonexistent_is_noop() {
        let node = make_node(1000, 3);
        node.ack(&999);
        assert_eq!(node.pending_count(), 0);
    }

    #[test]
    fn test_retry_loop_drops_after_max_retries() {
        let node = make_node(50, 2); 
        let msg = make_msg(Protocol::Echo { echo: "hi".to_string() });
        node.send(msg);
        assert_eq!(node.pending_count(), 1);

        std::thread::sleep(Duration::from_millis(351)); //1ms after it should expire
        assert_eq!(node.pending_count(), 0);
        
    }

    #[test]
    fn test_acked_message_not_retried() {
        let node = make_node(50, 3);
        let msg = make_msg(Protocol::Echo { echo: "hi".to_string() });
        let msg_id = msg.body.msg_id;
        node.send(msg);
        node.ack(&msg_id);

        std::thread::sleep(Duration::from_millis(400));

        assert_eq!(node.pending_count(), 0);
    }
}
