use super::echo;

use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::mpsc;
use libnyb::{deserialize, serialize, OpenError, Packet, Payload, Task, ID};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use log::{debug, warn};

pub type USBReader = mpsc::Receiver<Packet>;
pub type USBWriter = mpsc::Sender<Packet>;
pub type TaskSender = mpsc::Sender<Payload>;
type TaskReceiver = mpsc::Receiver<Payload>;
type TaskMap = Arc<RwLock<HashMap<ID, TaskSender>>>;
// type Task<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

#[derive(Debug, Clone)]
pub struct TaskManager {
    map: TaskMap,
    writer: USBWriter,
}

impl TaskManager {
    pub fn new(writer: mpsc::Sender<Packet>) -> TaskManager {
        TaskManager { 
            map: Arc::new(RwLock::new(HashMap::new())),
            writer: writer,
        }
    }

    pub async fn get_task(&self, id: ID) -> Option<TaskSender> {
        self.map.read().await.get(&id).cloned()
    }

    pub async fn create_channel(&self, id: ID) -> Channel {
        let (sender, channel) = Channel::new(id, &self.writer);
        self.map.write().await.insert(id, sender);
        channel
    }

    pub async fn start_task(&self, id: ID, task: Task) -> Result<(), OpenError> {
        if self.map.read().await.contains_key(&id) {
            warn!(target: "ctrl", "id {id} already taken");
            return Err(OpenError::IDTaken)
        }

        let (sender, channel) = Channel::new(id, &self.writer);
        
        let task_fn = match task {
            Task::Echo => Box::pin(echo::echo_task(channel)),
            // PULL_TASK => Box::pin(services::pull_task(task_channel)),
            // PUSH_TASK => Box::pin(services::push_task(task_channel)),
            _ => {
                warn!(target: "ctrl", "unknown task num {:?}", task);
                return Err(OpenError::UnkownTask);
            }
        };
        
        let task_map_clone = self.map.clone();
        let writer_clone = self.writer.clone();
        tokio::spawn(async move {
            task_map_clone.write().await.insert(id, sender);
            let result = task_fn.await;
            debug!("removing stream {} from map", id);
            if let Some(_) = task_map_clone.write().await.remove(&id){
                if let Ok(data) = serialize(&result) {
                    let _ = writer_clone.send(Packet(id, data)); // Potential for this to happen before Open(id) is sent
                }
            }
        });

        Ok(())
    }

}


#[derive(Debug)]
pub struct Channel {
    id: ID,
    rx: TaskReceiver,
    tx: USBWriter,
}

impl Channel {
    pub fn new(
        id: ID,
        writer: &USBWriter,
    ) -> (TaskSender, Channel) {
        let (tx, rx) = mpsc::channel(10);
        (
            tx,
            Channel {
                id: id,
                rx: rx,
                tx: writer.clone(),
            },
        )
    }

    // Need to add better error handling to recv & send
    pub async fn recv<T: DeserializeOwned>(&mut self) -> T
    {
        let payload = self.rx.recv().await.unwrap();
        deserialize(&payload).unwrap()
    }

    pub async fn send<T: Serialize>(&self, data: T) -> usize 
    {
        let payload = serialize(&data).unwrap();
        let size = payload.len();
        let packet = Packet(self.id, payload);
        self.tx.send(packet).await.unwrap();
        size
    }
}
