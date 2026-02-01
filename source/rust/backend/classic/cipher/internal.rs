// IMPORT
use chacha20poly1305::{aead::{Aead, KeyInit}, ChaCha20Poly1305, XChaCha20Poly1305, Key, Nonce, XNonce};
use pyo3::exceptions::*;
use pyo3::prelude::*;

// MAIN
#[pyfunction]
pub fn chencrypt(secretkey: &[u8], nonce: &[u8], buffer: &[u8]) -> PyResult<Vec<u8>> {
    if secretkey.len() != 32 {
        return Err(PyRuntimeError::new_err("Key must be 32 bytes"));
    }
    if nonce.len() != 12 {
        return Err(PyRuntimeError::new_err("Nonce must be 12 bytes"));
    }
    //
    let internalkey = Key::from_slice(secretkey);
    let internalcipher = ChaCha20Poly1305::new(internalkey);
    let internalnonce = Nonce::from_slice(nonce);
    //
    let ciphertext = internalcipher.encrypt(internalnonce, buffer)
        .map_err(|_| PyValueError::new_err("Encryption failed"));
    //
    ciphertext
}

#[pyfunction]
pub fn chdecrypt(secretkey: &[u8], nonce: &[u8], buffer: &[u8]) -> PyResult<Vec<u8>> {
    if secretkey.len() != 32 {
        return Err(PyValueError::new_err("Key must be 32 bytes"));
    }
    if nonce.len() != 12 {
        return Err(PyValueError::new_err("Nonce must be 12 bytes"));
    }
    //
    let internalkey = Key::from_slice(secretkey);
    let internalcipher = ChaCha20Poly1305::new(internalkey);
    let internalnonce = Nonce::from_slice(nonce);
    //
    let cleartext = internalcipher.decrypt(internalnonce, buffer)
        .map_err(|_| PyValueError::new_err("Decryption failed"));
    //
    cleartext
}

#[pyfunction]
pub fn xchencrypt(secretkey: &[u8], nonce: &[u8], buffer: &[u8]) -> PyResult<Vec<u8>> {
    if secretkey.len() != 32 {
        return Err(PyRuntimeError::new_err("Key must be 32 bytes"));
    }
    if nonce.len() != 24 {
        return Err(PyRuntimeError::new_err("Nonce must be 24 bytes"));
    }
    //
    let internalkey = Key::from_slice(secretkey);
    let internalcipher = XChaCha20Poly1305::new(internalkey);
    let internalnonce = XNonce::from_slice(nonce);
    //
    let ciphertext = internalcipher.encrypt(internalnonce, buffer)
        .map_err(|_| PyValueError::new_err("Encryption failed"));
    //
    ciphertext
}

#[pyfunction]
pub fn xchdecrypt(secretkey: &[u8], nonce: &[u8], buffer: &[u8]) -> PyResult<Vec<u8>> {
    if secretkey.len() != 32 {
        return Err(PyValueError::new_err("Key must be 32 bytes"));
    }
    if nonce.len() != 24 {
        return Err(PyValueError::new_err("Nonce must be 24 bytes"));
    }
    //
    let internalkey = Key::from_slice(secretkey);
    let internalcipher = XChaCha20Poly1305::new(internalkey);
    let internalnonce = XNonce::from_slice(nonce);
    //
    let cleartext = internalcipher.decrypt(internalnonce, buffer)
        .map_err(|_| PyValueError::new_err("Decryption failed"));
    //
    cleartext
}
