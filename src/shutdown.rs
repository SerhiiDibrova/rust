use std::sync::{Arc, Mutex};
use tokio::signal;
use tokio::sync::Notify;

pub struct Shutdown {
    notify: Arc<Notify>,
}

impl Shutdown {
    pub fn new() -> Self {
        Shutdown {
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn listen_for_shutdown(&self) {
        let notify = self.notify.clone();
        tokio::spawn(async move {
            signal::ctrl_c().await.expect("Failed to install Ctrl+C signal handler");
            notify.notify_one();
        });
    }

    pub async fn initiate_shutdown(&self) {
        self.notify.notified().await;
    }
}