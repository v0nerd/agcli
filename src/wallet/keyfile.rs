//! Keyfile I/O — encrypted coldkeys, plaintext hotkeys.
//!
//! Coldkeys are encrypted with AES-256-GCM using a key derived from
//! Argon2id (matching the Python bittensor-wallet encryption scheme).

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use argon2::Argon2;
use rand::RngCore;
use sp_core::sr25519;
use std::fs;
use std::path::Path;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

/// Write mnemonic encrypted with password.
pub fn write_encrypted_keyfile(path: &Path, mnemonic: &str, password: &str) -> Result<()> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut salt);
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let key = derive_key(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow::anyhow!("cipher init: {}", e))?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, mnemonic.as_bytes())
        .map_err(|e| anyhow::anyhow!("encryption failed: {}", e))?;

    // Format: salt || nonce || ciphertext
    let mut data = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    data.extend_from_slice(&salt);
    data.extend_from_slice(&nonce_bytes);
    data.extend_from_slice(&ciphertext);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, data).context("write keyfile")?;
    Ok(())
}

/// Read and decrypt an encrypted keyfile, returning the mnemonic.
pub fn read_encrypted_keyfile(path: &Path, password: &str) -> Result<String> {
    let data = fs::read(path).context("read keyfile")?;
    if data.len() < SALT_LEN + NONCE_LEN {
        anyhow::bail!("keyfile too short");
    }

    let (salt, rest) = data.split_at(SALT_LEN);
    let (nonce_bytes, ciphertext) = rest.split_at(NONCE_LEN);

    let key = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow::anyhow!("cipher init: {}", e))?;
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow::anyhow!("Decryption failed — wrong password?"))?;

    String::from_utf8(plaintext).context("mnemonic is not valid UTF-8")
}

/// Write a plaintext keyfile (for hotkeys).
pub fn write_keyfile(path: &Path, mnemonic: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, mnemonic).context("write keyfile")?;
    Ok(())
}

/// Read a plaintext keyfile.
pub fn read_keyfile(path: &Path) -> Result<String> {
    fs::read_to_string(path).context("read keyfile")
}

/// Write public key to a file (hex-encoded).
pub fn write_public_key(path: &Path, public: &sr25519::Public) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, hex::encode(public.0)).context("write public key")?;
    Ok(())
}

/// Read a public key from file.
pub fn read_public_key(path: &Path) -> Result<sr25519::Public> {
    let hex_str = fs::read_to_string(path).context("read public key file")?;
    let hex_str = hex_str.trim().strip_prefix("0x").unwrap_or(hex_str.trim());
    let bytes = hex::decode(hex_str).context("invalid hex")?;
    if bytes.len() != 32 {
        anyhow::bail!("public key must be 32 bytes, got {}", bytes.len());
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(sr25519::Public::from_raw(arr))
}

fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; KEY_LEN]> {
    let mut key = [0u8; KEY_LEN];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow::anyhow!("key derivation failed: {}", e))?;
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_coldkey");
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let password = "test_password_123";

        write_encrypted_keyfile(&path, mnemonic, password).unwrap();
        let recovered = read_encrypted_keyfile(&path, password).unwrap();
        assert_eq!(mnemonic, recovered);
    }

    #[test]
    fn wrong_password_fails() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_coldkey");
        let mnemonic = "test mnemonic phrase";
        write_encrypted_keyfile(&path, mnemonic, "correct").unwrap();
        assert!(read_encrypted_keyfile(&path, "wrong").is_err());
    }
}
