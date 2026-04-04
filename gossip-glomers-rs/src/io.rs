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

pub trait HandleMessage {
    type Message: serde::de::DeserializeOwned + Serialize + Send + 'static;
    fn handle(&mut self, node: Node<Self::Message>, msg::Mess)
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
