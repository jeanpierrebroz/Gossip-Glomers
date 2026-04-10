use gossip_glomers_rs::io::{Handler, Message};
use gossip_glomers_rs::node::Node;
use gossip_glomers_rs::protocol::Protocol;
use gossip_glomers_rs::run;

struct EchoHandler;

impl Handler<Protocol> for EchoHandler {
    fn handle(&mut self, msg: Message<Protocol>, node: &Node) {
        let (build_reply, contents) = msg.into_reply(node.get_next_msg_id());
        match contents {
            Protocol::Echo { echo } => {
                node.send(build_reply(Protocol::EchoOk { echo }));
            }
            _ => panic!("Unexpected message type"),
        }
    }
}

fn main() {
    run(EchoHandler, None);
}