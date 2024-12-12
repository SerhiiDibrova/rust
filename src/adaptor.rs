mod adaptor {
    use std::sync::{Arc, Mutex};
    use log::{info, error};

    pub struct AdaptorConfig {
        pub name: String,
        pub exchange_name: String,
        pub path: String,
    }

    pub struct Adaptor {
        name: String,
        exchange_name: String,
        path: String,
        is_cleaned_up: bool,
    }

    impl Adaptor {
        pub fn new(config: &AdaptorConfig) -> Self {
            Adaptor {
                name: config.name.clone(),
                exchange_name: config.exchange_name.clone(),
                path: config.path.clone(),
                is_cleaned_up: false,
            }
        }

        pub fn cleanup(&mut self) {
            if self.is_cleaned_up {
                return;
            }
            if let Err(e) = self.release_resources() {
                error!("Error during cleanup of adaptor {}: {}", self.name, e);
            }
            self.is_cleaned_up = true;
        }

        fn release_resources(&self) -> Result<(), String> {
            // Implement resource release logic here
            Ok(())
        }
    }

    pub fn load_adaptors(adaptor_list: &[AdaptorConfig]) {
        for adaptor_config in adaptor_list {
            if adaptor_config.name.is_empty() || adaptor_config.exchange_name.is_empty() || adaptor_config.path.is_empty() {
                error!("Invalid adaptor configuration: {:?}", adaptor_config);
                continue;
            }
            let adaptor = Adaptor::new(adaptor_config);
            let exchange_name = &adaptor.exchange_name;
            let path = &adaptor.path;
            info!("Loading adaptor: {}", adaptor_config.name);
            info!("Exchange name: {}", exchange_name);
            info!("Path: {}", path);
            if let Err(e) = load_adaptor(&adaptor, exchange_name, path) {
                error!("Failed to load adaptor {}: {}", adaptor_config.name, e);
            }
        }
    }

    fn load_adaptor(adaptor: &Adaptor, exchange_name: &str, path: &str) -> Result<(), String> {
        // Load the adaptor logic here
        Ok(())
    }
}