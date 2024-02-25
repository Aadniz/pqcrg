use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit}; // Or Aes128Gcm
use oqs::kem::SharedSecret;
use rand::Rng;

// This function encrypts the data
pub fn encrypt(key: SharedSecret, data: &[u8]) -> Result<(Vec<u8>, Vec<u8>), aes_gcm::Error> {
    let key = &key.clone().into_vec();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = rand::thread_rng().gen::<[u8; 12]>();
    let ciphertext = cipher.encrypt(GenericArray::from_slice(&nonce), data)?;
    Ok((nonce.to_vec(), ciphertext))
}

// This function decrypts the data
pub fn decrypt(
    key: SharedSecret,
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let key = &key.into_vec();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let plaintext = cipher.decrypt(GenericArray::from_slice(nonce), ciphertext)?;
    Ok(plaintext)
}
