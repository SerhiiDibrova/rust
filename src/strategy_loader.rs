use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use log::{error};

#[derive(Debug)]
struct Strategy {
    name: String,
    value: String,
}

lazy_static::lazy_static! {
    static ref STRATEGIES: Arc<Mutex<HashMap<String, Strategy>>> = Arc::new(Mutex::new(HashMap::new()));
}

fn load_strategies_from_file(file_path: &str) {
    let file = File::open(file_path);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to open file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to read line: {}", e);
                continue;
            }
        };

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            error!("Invalid line format: {}", line);
            continue;
        }

        let strategy = Strategy {
            name: parts[0].to_string(),
            value: parts[1].to_string(),
        };

        let mut strategies = STRATEGIES.lock().unwrap();
        if strategies.contains_key(&strategy.name) {
            error!("Duplicate strategy found: {}", strategy.name);
            continue;
        }

        strategies.insert(strategy.name.clone(), strategy);
    }
}