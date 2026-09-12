use sha2::{Digest, Sha256};
use std::sync::OnceLock;
use vortex_otp_lib::{OtpCharSet::Numeric, generate_otp};

static _KEY: OnceLock<&[u8]> = OnceLock::new();

pub async fn encrypt(_key: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    data
}

pub async fn decrypt(_key: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    data
}

pub fn uuid() -> Vec<u8> {
    let mut uuid = uuid::Uuid::new_v4().as_bytes().to_vec();
    let timestamp = chrono::Utc::now().timestamp_millis().to_be_bytes();
    uuid.extend_from_slice(&timestamp);
    uuid
}

pub fn sha_256(key: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(key);
    hasher.finalize().to_vec()
}

pub fn generate_top() -> Result<String, String> {
    let otp = generate_otp(6, Numeric)?;
    Ok(otp)
}
