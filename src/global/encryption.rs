use serde_json::Value;
use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use rand::Rng;
use std::error::Error;
use base64;

pub fn encrypt_response(connection_id: &str, response_type: &str, json_response: &str) -> Result<String, Box<dyn Error>> {
    if connection_id.len() != 32 {
        return Err("Invalid connection ID length".into());
    }

    let json_value: Value = serde_json::from_str(json_response).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let key = Key::from_slice(connection_id.as_bytes());
    let cipher = Aes256Gcm::new(key);
    
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let data_to_encrypt = format!("{}:{}", response_type, json_value);
    let encrypted_data = cipher.encrypt(nonce, data_to_encrypt.as_bytes())
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
    let encrypted_response = base64::encode(&encrypted_data);
    Ok(encrypted_response)
}