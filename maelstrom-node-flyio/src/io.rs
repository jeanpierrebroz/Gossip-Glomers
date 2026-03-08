use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::protocol::{Envelope, InternalBody};
use serde::de::DeserializeOwned;

pub struct MessageStream<B> {
    reader: io::Lines<BufReader<io::Stdin>>,
    _marker: std::marker::PhantomData<B>,
}

impl<B: DeserializeOwned> MessageStream<B> {
    pub fn new() -> Self {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin).lines();
        Self {
            reader,
            _marker: std::marker::PhantomData,
        }
    }

    pub async fn next_message(&mut self) -> anyhow::Result<Option<Envelope<InternalBody<B>>>> {
        if let Some(line) = self.reader.next_line().await? {
            let msg = serde_json::from_str(&line)?;
            Ok(Some(msg))
        } else {
            Ok(None)
        }
    }
}