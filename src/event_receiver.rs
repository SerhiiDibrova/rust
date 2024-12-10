package event_receiver

use std::sync::{Arc, Mutex};
use log::{info, error};

struct EventReceiver;

impl EventReceiver {
    pub fn receive_event(&self, token: u32) {
        let event_container = MerlinShared::_globalEventContainer.lock().unwrap();
        if let Some(strategies) = event_container.get(&token) {
            let mut activated = false;
            for strategy in strategies {
                if strategy.is_activated() {
                    activated = true;
                    let strategy_clone = strategy.clone();
                    let strategy_thread = MerlinShared::_globalStrategyThread.clone();
                    let task = move || {
                        if let Err(e) = std::panic::catch_unwind(|| {
                            strategy_clone.market_event_manager(token);
                        }) {
                            error!("Error executing strategy for token {}: {:?}", token, e);
                        }
                    };
                    strategy_thread.lock().unwrap().execute(task);
                    info!("Strategy activated for token: {:?}", strategy_clone);
                }
            }
            if !activated {
                info!("No activated strategies for token: {}", token);
            }
        } else {
            error!("Token not found: {}", token);
        }
    }
}

mod MerlinShared {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    pub static _globalEventContainer: Arc<Mutex<HashMap<u32, Vec<Strategy>>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static _globalStrategyThread: Arc<Mutex<StrategyThread>> = Arc::new(Mutex::new(StrategyThread {}));

    pub struct StrategyThread;

    impl StrategyThread {
        pub fn execute<F>(&self, task: F)
        where
            F: FnOnce() + Send + 'static,
        {
            std::thread::spawn(task);
        }
    }

    #[derive(Clone)]
    pub struct Strategy {
        activated: bool,
    }

    impl Strategy {
        pub fn is_activated(&self) -> bool {
            self.activated
        }

        pub fn market_event_manager(&self, token: u32) {
            // Implementation for handling market events
        }
    }
}