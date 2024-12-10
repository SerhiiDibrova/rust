package response

use serde::{Serialize, Deserialize};
use aes::{Aes256, NewBlockCipher};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use std::convert::TryInto;
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize)]
pub enum ResponseType {
    NEW,
    REPLACED,
    CANCELLED,
}

pub struct Response {
    response_type: ResponseType,
    data: Vec<u8>,
}

impl Response {
    pub fn new(response_type: ResponseType, data: Vec<u8>) -> Self {
        Response { response_type, data }
    }

    pub async fn send_response(&self, key: &[u8; 32], sender: &mpsc::Sender<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_valid() {
            return Err("Invalid response type".into());
        }
        let encrypted_data = self.encrypt(key)?;
        sender.send(encrypted_data).await.map_err(|e| format!("Send error: {}", e))?;
        Ok(())
    }

    fn encrypt(&self, key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let iv: [u8; 16] = rand::thread_rng().gen();
        let cipher = Aes256::new(key.into());
        let mut buffer = self.serialize()?;
        let pos = buffer.len();
        buffer.resize(pos + 16, 0);
        Cbc::<Aes256, Pkcs7>::new(cipher, &iv.into()).encrypt(&mut buffer[pos..]).map_err(|e| format!("Encryption error: {}", e))?;
        let mut result = iv.to_vec();
        result.extend_from_slice(&buffer[pos..]);
        Ok(result)
    }

    fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        bincode::serialize(self).map_err(|e| format!("Serialization error: {}", e).into())
    }

    fn is_valid(&self) -> bool {
        matches!(self.response_type, ResponseType::NEW | ResponseType::REPLACED | ResponseType::CANCELLED)
    }
}