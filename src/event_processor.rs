mod event_processor {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::collections::HashMap;
    use log::{info, error};

    struct EventData {
        event_type: String,
        details: HashMap<String, String>,
    }

    struct GlobalContainer {
        events: Vec<EventData>,
    }

    fn process_events(global_container: Arc<Mutex<GlobalContainer>>, strategy: fn(&EventData) -> bool) {
        let events = global_container.lock().unwrap().events.clone();
        let mut handles = vec![];

        for event in events {
            let event_clone = event.clone();
            let strategy_clone = strategy.clone();
            let handle = thread::spawn(move || {
                let token = extract_token(&event_clone);
                if strategy_clone(&event_clone) {
                    handle_market_event(token, &event_clone);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            if let Err(e) = handle.join() {
                error!("Thread panicked: {:?}", e);
            }
        }
    }

    fn extract_token(event: &EventData) -> String {
        event.details.get("token").cloned().unwrap_or_default()
    }

    fn handle_market_event(token: String, event: &EventData) {
        info!("Handling market event for token: {}", token);
    }
}