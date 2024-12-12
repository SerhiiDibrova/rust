mod message_queue {
    use std::sync::{Mutex, Condvar};
    use std::collections::VecDeque;
    use std::sync::Arc;
    use std::fmt::Debug;

    pub struct MessageQueue<T> {
        queue: Mutex<VecDeque<T>>,
        condvar: Condvar,
    }

    impl<T: Debug> MessageQueue<T> {
        pub fn new() -> Self {
            MessageQueue {
                queue: Mutex::new(VecDeque::new()),
                condvar: Condvar::new(),
            }
        }

        pub fn push(&self, message: T) -> Result<(), String> {
            let mut queue = self.queue.lock().map_err(|_| "Failed to lock the mutex".to_string())?;
            queue.push_back(message);
            self.condvar.notify_one();
            Ok(())
        }

        pub fn pop(&self) -> Result<T, String> {
            let mut queue = self.queue.lock().map_err(|_| "Failed to lock the mutex".to_string())?;
            while queue.is_empty() {
                queue = self.condvar.wait(queue).map_err(|_| "Failed to wait on condvar".to_string())?;
            }
            Ok(queue.pop_front().unwrap())
        }

        pub fn is_empty(&self) -> bool {
            let queue = self.queue.lock().expect("Failed to lock the mutex");
            queue.is_empty()
        }

        pub fn clear(&self) -> Result<(), String> {
            let mut queue = self.queue.lock().map_err(|_| "Failed to lock the mutex".to_string())?;
            queue.clear();
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::thread;

        #[test]
        fn test_message_queue() {
            let queue = Arc::new(MessageQueue::new());
            let queue_clone = Arc::clone(&queue);

            let producer = thread::spawn(move || {
                for i in 0..10 {
                    queue_clone.push(i).unwrap();
                }
            });

            let consumer = thread::spawn(move || {
                let mut messages = Vec::new();
                for _ in 0..10 {
                    messages.push(queue.pop().unwrap());
                }
                messages
            });

            producer.join().unwrap();
            let messages = consumer.join().unwrap();
            assert_eq!(messages, (0..10).collect::<Vec<_>>());
        }

        #[test]
        fn test_pop_empty_queue() {
            let queue = Arc::new(MessageQueue::new());
            let consumer = {
                let queue_clone = Arc::clone(&queue);
                thread::spawn(move || {
                    let result = queue_clone.pop();
                    assert!(result.is_ok());
                })
            };

            consumer.join().unwrap();
        }

        #[test]
        fn test_clear_queue() {
            let queue = Arc::new(MessageQueue::new());
            queue.push(1).unwrap();
            queue.push(2).unwrap();
            assert!(!queue.is_empty());
            queue.clear().unwrap();
            assert!(queue.is_empty());
        }
    }
}