mod strategy {
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct Strategy {
        name: String,
        parameter1: f64,
        parameter2: f64,
        id: String,
    }

    impl Strategy {
        pub fn new(name: String, parameter1: f64, parameter2: f64) -> Result<Self, String> {
            if name.trim().is_empty() || name.len() > 50 {
                return Err("Invalid strategy name".to_string());
            }
            if parameter1.is_nan() || parameter2.is_nan() {
                return Err("Parameters must be valid numbers".to_string());
            }
            let id = format!("{}-{}", name, uuid::Uuid::new_v4());
            Ok(Strategy { name, parameter1, parameter2, id })
        }

        pub fn get_id(&self) -> &str {
            &self.id
        }

        pub fn is_strategy_active(token: &str) -> bool {
            // Placeholder for actual implementation
            token == "active"
        }

        pub fn activate_strategy(&self, token: &str) {
            // Placeholder for actual implementation
            if token == "activate" {
                println!("Activating strategy: {}", self.name);
            }
        }
    }

    pub fn check_strategy_existence(name: &str, strategies: &HashSet<String>) -> bool {
        if name.trim().is_empty() || name.len() > 50 {
            return false;
        }
        strategies.contains(name)
    }
}