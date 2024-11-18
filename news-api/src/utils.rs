use anyhow::{anyhow, Context, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use hmac::{Hmac, Mac};
use rand::random;
use sha2::Sha256;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

pub fn parse_timestamp(timestamp_str: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S%.6f").ok()
}

pub fn generate_session_id(user_id: i32, secret_key: &str) -> Result<String> {
    let mut mac =
        HmacSha256::new_from_slice(secret_key.as_bytes()).context("[news-api] hmac 256 error")?;

    mac.update(user_id.to_string().as_bytes());
    mac.update(Uuid::new_v4().to_string().as_bytes());

    let session_id = hex::encode(mac.finalize().into_bytes());

    Ok(session_id)
}

pub struct PasswordHash {
    pub value: String,
    pub salt: String,
}

pub fn generate_password_hash(password: &str, pepper: &str) -> Result<PasswordHash> {
    let salt: [u8; 16] = random();
    let salt_hex = hex::encode(salt);

    let salted_password = format!("{:?}{}{}", salt_hex, password, pepper);
    let password_hash = hash(&salted_password, DEFAULT_COST).context("[news-api] hashing error")?;

    Ok(PasswordHash {
        value: password_hash,
        salt: salt_hex,
    })
}

pub fn verify_password(
    input_password: &str,
    stored_hash: &str,
    stored_salt: &str,
    pepper: &str,
) -> Result<()> {
    let salted_password = format!("{:?}{}{}", stored_salt, input_password, pepper);

    match verify(&salted_password, stored_hash)? {
        true => Ok(()),
        false => Err(anyhow!("[news-api] invalid password")),
    }
}
