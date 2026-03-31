use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");
    read();
}



fn read() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match &line {
            Ok(s) => {
                println!("Read line: {:?}", s);
            }

            Err(s) => {
                println!("Couldn't read line: {:?}", s);
            }
        }
    }
}
