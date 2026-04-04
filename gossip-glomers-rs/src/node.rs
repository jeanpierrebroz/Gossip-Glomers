use std::marker::PhantomData;
use std::io::Stdout;
use std::sync::atomic::AtomicUsize;
use std::sync::{Mutex, Arc};


struct Node<T> {
    pub id: Option<String>,
    pub node_ids: Option<Vec<String>>,
    msg_counter: Arc<AtomicUsize>,
    writer: Arc<Mutex<Stdout>>,
    _phantom: PhantomData<T>,
}

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            id: &self.id,
            node_ids: Arc::clone(&self.node_ids),
            msg_counter: Arc::clone(&self.msg_counter),
            writer: Arc::clone(&self.writer),
            _phantom: PhantomData
        }
    }
}

impl<T> Node<T> {
    pub fn new(id: String, node_ids: Vec<String>) -> Self {
        Self {
            id: &id
            node_ids: Arc::new(Mutex::new(node_ids)),
            msg_counter: Arc::new(AtomicUsize::new(1)),
            writer: Arc::new(Mutex::new(std::io::stdout())),
            _phantom: PhantomData,
        }
    }

    pub fn get_next_msg_id(&self) -> usize {
        self.msg_counter.fetch_add(1, Ordering::SeqCst)
    }
}
