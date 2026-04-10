use gossip_glomers_rs::io::{Handler, Message};
use gossip_glomers_rs::node::Node;
use gossip_glomers_rs::protocol::Protocol;
use gossip_glomers_rs::run;

struct UidHandler {
    counter: u64,
}

impl Handler<Protocol> for UidHandler {
    fn handle(&mut self, msg: Message<Protocol>, node: &Node) {
        let (build_reply, contents) = msg.into_reply(node.get_next_msg_id());
        match contents {
            Protocol::Generate => {
                let id = format!("{}-{}", node.id, self.counter);
                self.counter += 1;
                node.send(build_reply(Protocol::GenerateOk { id }));
            }
            _ => panic!("Unexpected message type"),
        }
    }
}

fn main() {
    run(UidHandler { counter: 0 }, None);
}
