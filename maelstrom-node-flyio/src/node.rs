use std::{collections::HashMap, thread};
use tokio::sync::oneshot;
use serde_json::Value;
use crate::protocol::{InternalBody, envelope::Envelope};

pub struct Node<B> {
    pub(crate) node_id: Mutex<Option<String>>,
    pub(crate) msg_id: AtomicUsize,
    pub(crate) waiting_room: Mutex<HashMap<usize, oneshot::Sender<InternalBody<B>>>>,
    pub(crate) handler: Arc<dyn crate::protocol::client::Handler<B>>,
}

impl<B> Node<B> 
where B: Serialize + DeserializeOwned + Send + Sync + 'static 
{
    pub async fn run(self: Arc<Self>, mut stream: MessageStream<B>) -> anyhow::Result<()> {
        while let Some(msg) = stream.next_message().await? {
            let node = Arc::clone(&self);
    
            match msg.body {
                InternalBody::ReadOk { value, in_reply_to } => {
                    let mut room = self.waiting_room.lock().await;
                    if let Some(tx) = room.remove(&in_reply_to) {
                        let _ = tx.send(InternalBody::ReadOk { value, in_reply_to });
                    }
                }
    
                InternalBody::Init { node_id, .. } => {
                    let mut id_lock = self.node_id.lock().await;
                    *id_lock = Some(node_id);
                    
                    self.reply(&msg, InternalBody::InitOk).await?;
                }
    
                InternalBody::User(user_body) => {
                    let handler = Arc::clone(&self.handler);
                    tokio::spawn(async move {
                        let user_msg = msg.replace_body(user_body);
                        if let Err(e) = handler.handle_message(node, user_msg).await {
                            eprintln!("Handler error: {}", e);
                        }
                    });
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub async fn reply(&self, req: &Envelope<InternalBody<B>>, body: InternalBody<B>) -> anyhow::Result<()> {
        let node_id = self.node_id.lock().await.as_ref().cloned().unwrap_or_default();
        let reply = Envelope {
            src: self.node_id,
            dest: req.src,
            body,
        };
        
        let json = serde_json::to_string(&reply)?;
        println!("{}", json);
        Ok(())
    }
    
    pub async fn send(&self, body: InternalBody<B>, dest: String) -> anyhow::Result<()> {
        let msg = Envelope {
            src: self.node_id,
            dest: dest,
            body
        };
        
        let json = serde_json::to_string(&msg)?;
        println!("{}", json);
        Ok(())
        
    }
}