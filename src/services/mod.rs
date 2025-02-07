mod services {
    use std::collections::HashMap;

    pub struct Service {
        pub name: String,
        pub status: String,
    }

    pub struct ServiceManager {
        services: HashMap<String, Service>,
    }

    impl ServiceManager {
        pub fn new() -> Self {
            ServiceManager {
                services: HashMap::new(),
            }
        }

        pub fn add_service(&mut self, name: String, status: String) {
            let service = Service { name: name.clone(), status };
            self.services.insert(name, service);
        }

        pub fn remove_service(&mut self, name: &str) {
            self.services.remove(name);
        }

        pub fn update_service_status(&mut self, name: &str, status: String) {
            if let Some(service) = self.services.get_mut(name) {
                service.status = status;
            }
        }

        pub fn get_service_status(&self, name: &str) -> Option<&String> {
            self.services.get(name).map(|service| &service.status)
        }

        pub fn list_services(&self) -> Vec<&Service> {
            self.services.values().collect()
        }
    }
}