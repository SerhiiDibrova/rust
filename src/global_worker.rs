mod global_worker {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use log::{error, info};

    struct GlobalWorker {
        is_running: Arc<Mutex<bool>>,
        ongoing_tasks: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
    }

    impl GlobalWorker {
        fn new() -> Self {
            GlobalWorker {
                is_running: Arc::new(Mutex::new(false)),
                ongoing_tasks: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn reset(&self) {
            let running = self.is_running.clone();
            let ongoing_tasks = self.ongoing_tasks.clone();

            let mut running_lock = match running.lock() {
                Ok(lock) => lock,
                Err(e) => {
                    error!("Failed to acquire lock during reset: {}", e);
                    return;
                }
            };

            if !*running_lock {
                info!("Worker is already in a clean state.");
                return;
            }

            *running_lock = false;

            info!("Terminating ongoing tasks...");
            let mut tasks_lock = ongoing_tasks.lock().unwrap();
            for task in tasks_lock.drain(..) {
                if let Err(e) = task.join() {
                    error!("Error joining task: {:?}", e);
                }
            }

            info!("Resetting worker state...");
            if let Err(e) = self.perform_reset() {
                error!("Error during reset: {}", e);
            }
        }

        fn perform_reset(&self) -> Result<(), String> {
            // Logic to reset the worker's state
            Ok(())
        }

        fn start_task(&self) {
            let ongoing_tasks = self.ongoing_tasks.clone();
            let is_running = self.is_running.clone();

            let handle = thread::spawn(move || {
                let mut running_lock = is_running.lock().unwrap();
                *running_lock = true;

                while *running_lock {
                    thread::sleep(Duration::from_secs(1));
                }
            });

            ongoing_tasks.lock().unwrap().push(handle);
        }
    }
}