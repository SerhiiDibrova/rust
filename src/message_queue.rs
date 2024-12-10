package message_queue

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

struct MerlinShared {
    _globalArthurMessageQueue: MessageQueue<String>,
}

struct MessageQueue<T> {
    sender: Sender<T>,
    receiver: Arc<Mutex<Receiver<T>>>,
}

impl<T> MessageQueue<T> {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        MessageQueue {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    fn send(&self, message: T) -> Result<(), String> {
        self.sender.send(message).map_err(|e| e.to_string())
    }

    fn read(&self) -> Result<Option<T>, String> {
        let receiver = self.receiver.lock().map_err(|_| "Failed to lock receiver".to_string())?;
        receiver.recv().map(Some).map_err(|e| e.to_string())
    }

    fn read_all(&self) -> Result<Vec<T>, String> {
        let receiver = self.receiver.lock().map_err(|_| "Failed to lock receiver".to_string())?;
        let mut messages = Vec::new();
        while let Ok(message) = receiver.try_recv() {
            messages.push(message);
        }
        if messages.is_empty() {
            Err("No messages available".to_string())
        } else {
            Ok(messages)
        }
    }
}