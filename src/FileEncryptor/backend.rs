use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm_siv::Aes256GcmSiv;
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey, Nonce as ChaChaNonce};
use pbkdf2::pbkdf2;
use rand::RngCore;
use sha2::Sha256;
use hmac::Hmac;
use std::fmt;
use std::fs::{read, write};
use std::io::{self, Read, Write};
use std::path::{PathBuf};

const SALT_LEN: usize = 16;
const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const ITERATIONS: u32 = 100_000;

#[derive(Debug)]
pub enum CryptoError {
    AesGcm(aes_gcm::Error),
    Io(io::Error),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::AesGcm(e) => write!(f, "AES-GCM error: {:?}", e),
            CryptoError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for CryptoError {}

impl From<aes_gcm::Error> for CryptoError {
    fn from(err: aes_gcm::Error) -> CryptoError {
        CryptoError::AesGcm(err)
    }
}

impl From<io::Error> for CryptoError {
    fn from(err: io::Error) -> CryptoError {
        CryptoError::Io(err)
    }
}

#[derive(Debug, Clone)]
pub enum Algorithm {
    AesGcm,
    AesGcmSiv,
    ChaCha20Poly1305,
}

impl Algorithm { //set id for each Algorithm
    fn identifier(&self) -> u8 {
        match self {
            Algorithm::AesGcm => 1,
            Algorithm::AesGcmSiv => 2,
            Algorithm::ChaCha20Poly1305 => 3,
        }
    }

    fn from_identifier(id: u8) -> Option<Self> {
        match id {
            1 => Some(Algorithm::AesGcm),
            2 => Some(Algorithm::AesGcmSiv),
            3 => Some(Algorithm::ChaCha20Poly1305),
            _ => None,
        }
    }
}

pub fn derive_key_from_password(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, ITERATIONS, &mut key);
    key
}

pub fn encrypt_file(input_path: &PathBuf, output_path: &PathBuf, password: &str, algorithm: Algorithm) -> Result<(), CryptoError> {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    let key = derive_key_from_password(password, &salt);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let plaintext = read(input_path)?;

    let ciphertext = match algorithm {
        Algorithm::AesGcm => {
            let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
            cipher.encrypt(&nonce, plaintext.as_ref())?
        }
        Algorithm::AesGcmSiv => {
            let cipher = Aes256GcmSiv::new(Key::<Aes256GcmSiv>::from_slice(&key));
            cipher.encrypt(&nonce, plaintext.as_ref())?
        }
        Algorithm::ChaCha20Poly1305 => {
            let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(&key));
            let nonce = ChaChaNonce::from_slice(nonce.as_slice());
            cipher.encrypt(nonce, plaintext.as_ref())?
        }
    };

    let mut output = Vec::new();
    output.push(algorithm.identifier());
    output.extend_from_slice(&salt);
    output.extend_from_slice(nonce.as_slice());
    output.extend_from_slice(&ciphertext);

    write(output_path, output)?;
    Ok(())
}

pub fn decrypted_file(input_path: &PathBuf, password: &str) -> Result<Vec<u8>, CryptoError> {
    let data = read(input_path)?;
    let (id, rest) = data.split_at(1);
    let algorithm = Algorithm::from_identifier(id[0]).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Unknown algorithm identifier"))?;
    let (salt, rest) = rest.split_at(SALT_LEN);
    let (nonce, ciphertext) = rest.split_at(NONCE_LEN);

    let key = derive_key_from_password(password, salt);
    let nonce = Nonce::from_slice(nonce);

    match algorithm {
        Algorithm::AesGcm => {
            let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
            Ok(cipher.decrypt(nonce, ciphertext.as_ref())?)
        }
        Algorithm::AesGcmSiv => {
            let cipher = Aes256GcmSiv::new(Key::<Aes256GcmSiv>::from_slice(&key));
            Ok(cipher.decrypt(nonce, ciphertext.as_ref())?)
        }
        Algorithm::ChaCha20Poly1305 => {
            let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(&key));
            let nonce = ChaChaNonce::from_slice(nonce.as_slice());
            Ok(cipher.decrypt(nonce, ciphertext.as_ref())?)
        }
    }
}

pub fn create_decrypted_file(input_path: &PathBuf, output_path: &PathBuf, password: &str) -> Result<(), CryptoError> {
    let plaintext = decrypted_file(input_path, password)?;

    write(output_path, plaintext)?;
    Ok(())
}

pub fn read_file(input_path: &PathBuf, password: &str) -> Result<String, CryptoError> {
    let plaintext = String::from_utf8(decrypted_file(input_path, password)?).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(plaintext)
}