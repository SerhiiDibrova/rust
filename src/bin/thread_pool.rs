package main

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::vec::Vec;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
    _errorCode: Arc<Mutex<Option<String>>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        if size == 0 {
            panic!("ThreadPool size must be greater than 0");
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let _errorCode = Arc::new(Mutex::new(None));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver), Arc::clone(&_errorCode)));
        }

        ThreadPool { workers, sender, _errorCode }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Err(_) = self.sender.send(job) {
            let mut error_code = self._errorCode.lock().unwrap();
            *error_code = Some("Failed to send job to worker".to_string());
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>, _errorCode: Arc<Mutex<Option<String>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    if let Err(e) = std::panic::catch_unwind(|| job()) {
                        let mut error_code = _errorCode.lock().unwrap();
                        *error_code = Some(format!("Job execution failed: {:?}", e));
                    }
                }
                Err(_) => {
                    let mut error_code = _errorCode.lock().unwrap();
                    *error_code = Some("Worker channel closed".to_string());
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender);
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}