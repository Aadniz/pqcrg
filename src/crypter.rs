use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit}; // Or Aes128Gcm
use oqs::kem::SharedSecret;
use rand::Rng;

/// Encrypts the given data using the provided shared secret key.
///
/// # Arguments
///
/// * `key` - The shared secret key used for encryption.
/// * `data` - The data to be encrypted.
///
/// # Returns
///
/// A tuple containing the nonce and the ciphertext, or an error if encryption fails.
pub fn encrypt(key: SharedSecret, data: &[u8]) -> Result<(Vec<u8>, Vec<u8>), aes_gcm::Error> {
    let key = &key.clone().into_vec();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = rand::thread_rng().gen::<[u8; 12]>();
    let ciphertext = cipher.encrypt(GenericArray::from_slice(&nonce), data)?;
    Ok((nonce.to_vec(), ciphertext))
}

/// Decrypts the given ciphertext using the provided shared secret key and nonce.
///
/// # Arguments
///
/// * `key` - The shared secret key used for decryption.
/// * `nonce` - The nonce used in the encryption process.
/// * `ciphertext` - The ciphertext to be decrypted.
///
/// # Returns
///
/// The decrypted data as a vector of bytes, or an error if decryption fails.
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
