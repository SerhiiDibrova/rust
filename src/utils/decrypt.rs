use aes::{Aes128, BlockDecrypt, NewBlockCipher};
use block_modes::{BlockMode, Cbc};
use block_modes::cipher::generic_array::GenericArray;
use std::convert::TryInto;

#[derive(Debug)]
pub enum DecryptError {
    InvalidKey,
    InvalidData,
    DecryptionFailed,
    InvalidUtf8,
}

pub fn decrypt_message(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<String, DecryptError> {
    if key.len() != 16 {
        return Err(DecryptError::InvalidKey);
    }
    if iv.len() != 16 {
        return Err(DecryptError::InvalidData);
    }
    if encrypted_data.len() % 16 != 0 {
        return Err(DecryptError::InvalidData);
    }
    
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let iv = GenericArray::from_slice(iv);
    let cipher = Cbc::<Aes128, _>::new(cipher, iv);
    
    let mut buffer = encrypted_data.to_vec();
    let decrypted_data = cipher.decrypt_vec(&mut buffer).map_err(|_| DecryptError::DecryptionFailed)?;
    
    String::from_utf8(decrypted_data).map_err(|_| DecryptError::InvalidUtf8)
}