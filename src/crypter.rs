use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit}; // Or Aes128Gcm
use oqs::kem;
use rand::Rng;
use std::io::{Error, ErrorKind};
use std::result::Result;

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
pub fn encrypt(key: super::KEM, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match key {
        super::KEM::Kyber(shared_secret) => {
            let key = &shared_secret.clone().into_vec();
            let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
            let mut nonce = rand::thread_rng().gen::<[u8; 12]>().to_vec();
            let ciphertext = cipher
                .encrypt(GenericArray::from_slice(&nonce), data)
                .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to encrypt: {e}")))?;
            nonce.extend(ciphertext);
            Ok(nonce)
        }
        super::KEM::Rsa(pub_key) => {
            let mut ciphertext = vec![0; super::RSA.size() as usize];

            let _ = pub_key.public_encrypt(data, &mut ciphertext, openssl::rsa::Padding::PKCS1)?;
            Ok(ciphertext) // RSA doesn't use a nonce
        }
        super::KEM::None => Ok(vec![]),
    }
}

pub fn decrypt(key: &super::KEM, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match key {
        crate::KEM::Kyber(shared_secret) => decrypt_aes_gcm(shared_secret, data),
        crate::KEM::Rsa(_) => decrypt_rsa(data),
        crate::KEM::None => Ok(vec![]),
    }
}

fn decrypt_rsa(buf: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut plaintext = vec![0; super::RSA.size() as usize];
    let _ = super::RSA.private_decrypt(buf, &mut plaintext, openssl::rsa::Padding::PKCS1)?;
    Ok(plaintext)
}

fn decrypt_aes_gcm(
    key: &kem::SharedSecret,
    data: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let nonce = &data[..12];
    let ciphertext = &data[12..];

    let key = GenericArray::clone_from_slice(&key.clone().into_vec());
    let cipher = Aes256Gcm::new(&key);
    let nonce = GenericArray::from_slice(nonce);
    let ciphertext: &[u8] = ciphertext.into();
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to decrypt: {e}")))?;

    Ok(plaintext)
}
