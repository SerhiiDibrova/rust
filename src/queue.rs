mod queue {
    use std::vec::Vec;

    pub struct Message {
        pub content: String,
    }

    pub struct MessageQueue {
        messages: Vec<Message>,
    }

    impl MessageQueue {
        pub fn new() -> Self {
            MessageQueue {
                messages: Vec::new(),
            }
        }

        pub fn is_queue_empty(&self) -> bool {
            self.messages.is_empty()
        }

        pub fn dequeue_message(&mut self) -> Option<Message> {
            if !self.is_queue_empty() {
                Some(self.messages.remove(0))
            } else {
                None
            }
        }

        pub fn enqueue_message(&mut self, message: Message) {
            self.messages.push(message);
        }
    }
}