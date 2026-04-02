use std::io::BufRead;
use serde::{Deserialize, Serialize};


use std::sync::mpsc::{
    channel,
    Sender
};

enum SystemType {
    Init,
    InitOk
}   

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T> {
    pub src: String,
    pub dest: String,
    pub body: Body<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<T> {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub contents: T,
}

struct Init {
    node_id: String,
    node_ids: Vec<String>
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
    fn test_input() {
        let (sender, receiver) = channel();
        let input = "line one\nline two\n";
        let reader = input.as_bytes();
        read(reader, sender);
        
        assert_eq!(receiver.recv().unwrap(), "line one");
        
        assert_ne!(receiver.recv().unwrap(), "line one");
    }
    
    #[test]
    fn test_type_mapping() {
        let (sender, receiver) = channel();
        let input = "line one\nline two\n";
        let reader = input.as_bytes();
        read(reader, sender);
        
        assert_eq!(receiver.recv().unwrap(), "line one");
        
        assert_ne!(receiver.recv().unwrap(), "line one");
    }
    
}
