mod config {
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::Read;
    use std::env;
    use std::error::Error;

    const DEFAULT_PORT: u16 = 8080;
    const MAX_CONNECTIONS: usize = 100;

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        pub connection: ConnectionConfig,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ConnectionConfig {
        pub server_address: String,
        pub port: u16,
        pub timeout: u64,
    }

    pub fn load_configuration(file_path: &str) -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn load_config() -> Result<Config, Box<dyn Error>> {
        let file_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.json".to_string());
        load_configuration(&file_path)
    }

    pub fn get_default_port() -> u16 {
        DEFAULT_PORT
    }

    pub fn get_max_connections() -> usize {
        MAX_CONNECTIONS
    }
}