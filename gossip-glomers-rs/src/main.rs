use serde::{Deserialize, Serialize};
use std::io::{BufRead, Stdout};

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{Sender, channel};


fn main() {
    let (sender, _recv) = channel();
    read(std::io::stdin().lock(), sender);
}



// TODO: create node struct
// TODO: pass unhandled system types to user-defined handler
// TODO: auto-set the old msg_id to be in reply to the message they're replying to