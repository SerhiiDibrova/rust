package adaptor_loader

use std::ffi::CString;
use std::ptr;
use std::collections::HashMap;
use libloading::{Library, Symbol};
use log::{error, info};

const _GLOBAL_SHARED_LIB_ENTRY_FUNCTION_NAME: &str = "create_adaptor_instance";

struct AdaptorLoader {
    thread_pool: ThreadGroup,
    global_adaptors: HashMap<Exchange, AdaptorConnection>,
}

pub struct AdaptorConnection {
    // Fields for managing the adaptor connection
}

impl AdaptorLoader {
    pub fn new(thread_pool: ThreadGroup) -> Self {
        AdaptorLoader {
            thread_pool,
            global_adaptors: HashMap::new(),
        }
    }

    pub fn load_adaptor(&mut self, dll_path: &str, exchange: Exchange) -> Result<(), String> {
        let lib = Library::new(dll_path).map_err(|e| {
            error!("Failed to load DLL: {}", e);
            "Failed to load DLL".to_string()
        })?;

        let entry: Symbol<unsafe extern fn() -> *mut AdaptorConnection> = unsafe {
            lib.get::<Symbol<unsafe extern fn() -> *mut AdaptorConnection>>(CString::new(_GLOBAL_SHARED_LIB_ENTRY_FUNCTION_NAME).unwrap().as_bytes())
                .map_err(|e| {
                    error!("Entry function not found: {}", e);
                    "Entry function not found".to_string()
                })?
        };

        let adaptor_instance = unsafe { entry() };
        if adaptor_instance.is_null() {
            error!("Failed to create adaptor instance");
            return Err("Failed to create adaptor instance".to_string());
        }

        let adaptor_connection = unsafe { &*adaptor_instance };

        self.global_adaptors.insert(exchange, adaptor_connection.clone());

        info!("Adaptor loaded successfully for exchange: {:?}", exchange);
        Ok(())
    }
}