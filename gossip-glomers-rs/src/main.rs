use std::io::BufRead;

use std::sync::mpsc::{
    channel,
    Sender
};

enum SystemType {
    Init,
    InitOk
}   

struct BaseMessage<T> {
    src: String,
    dest: String,
    body: T
}

struct Init {
    
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
            }

            Err(_s) => {
                panic!("Unexpected input type found, panicking. Ensure you're covering all possible input types. ");

            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_input() {
        let (sender, recv) = channel();
        let input = "line one\nline two\n";
        let reader = input.as_bytes();
        read(reader, sender);
        
        
        
    }
}