// ********************* interface ********************* //
use crate::app::common::prelude::AppResult;

pub trait CryptoUtilsTrait {
    fn hash(&self, message: &str) -> AppResult<String>;
    fn verify(&self, attempted_msg: &str, encoded_salt_hash: &str) -> AppResult<()>;
}

pub trait CryptoUtilsProvider {
    type CryptoUtils: CryptoUtilsTrait;
    fn crypto_utils(&self) -> &Self::CryptoUtils;
}

// ********************* implementation ********************* //
use std::num::NonZeroU32;

use base64::{engine::general_purpose::STANDARD, Engine};
use ring::{
    pbkdf2,
    rand::{SecureRandom, SystemRandom},
};

use crate::app::common::prelude::{AppError, AppErrorKind, WrapToAppResult};

pub struct Pbkdf2CryptoUtils {
    algorithm: pbkdf2::Algorithm,
    iterations: NonZeroU32,
    salt_len: usize,
    credential_len: usize,
}

impl Default for Pbkdf2CryptoUtils {
    fn default() -> Self {
        Self {
            algorithm: pbkdf2::PBKDF2_HMAC_SHA256,
            iterations: NonZeroU32::new(10_000).expect("Hash iterations cannot be zero"),
            salt_len: 16,
            credential_len: 32,
        }
    }
}
impl CryptoUtilsTrait for Pbkdf2CryptoUtils {
    fn hash(&self, message: &str) -> AppResult<String> {
        let mut salt = vec![0_u8; self.salt_len];
        SystemRandom::new()
            .fill(&mut salt)
            .wrap("Failed to generate salt", AppErrorKind::default())?;

        let mut pbkdf2_hash = vec![0_u8; self.credential_len];
        pbkdf2::derive(
            self.algorithm,
            self.iterations,
            &salt,
            message.as_bytes(),
            &mut pbkdf2_hash,
        );

        let salt_and_hash = [salt, pbkdf2_hash].concat();
        Ok(STANDARD.encode(salt_and_hash))
    }

    fn verify(&self, attempted_msg: &str, encoded_salt_hash: &str) -> AppResult<()> {
        let salt_and_hash = STANDARD
            .decode(encoded_salt_hash)
            .wrap("Base64 decoding failed", AppErrorKind::default())?;

        if salt_and_hash.len() != self.salt_len + self.credential_len {
            return Err(AppError::new(
                format!(
                    "Invalid attempted_msg, salt_len: {}, credential_len: {}, decoded_msg_len: {}",
                    self.salt_len,
                    self.credential_len,
                    salt_and_hash.len()
                ),
                AppErrorKind::default(),
            ));
        }

        let (salt, hash) = salt_and_hash.split_at(self.salt_len);
        pbkdf2::verify(
            self.algorithm,
            self.iterations,
            salt,
            attempted_msg.as_bytes(),
            hash,
        )
        .wrap("Incorrect password", AppErrorKind::MalformedCredential)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let crypto_utils = Pbkdf2CryptoUtils::default();
        let message = "test_message";

        let hash = crypto_utils.hash(message);
        assert!(hash.is_ok(), "Hashing should succeed.");
        let hash = hash.unwrap();

        let verify = crypto_utils.verify(message, &hash);
        assert!(
            verify.is_ok(),
            "Verification should succeed with correct password."
        );
    }

    #[test]
    fn test_hash_and_verify_with_wrong_message() {
        let crypto_utils = Pbkdf2CryptoUtils::default();

        let message = "test_message";
        let wrong_message = "wrong_message";

        let hash = crypto_utils.hash(message);
        assert!(hash.is_ok(), "Hashing should succeed.");
        let hash = hash.unwrap();

        let verify = crypto_utils.verify(wrong_message, &hash);
        assert!(
            verify.is_err(),
            "Verification should fail with incorrect password."
        );
    }

    #[test]
    fn test_hash_and_verify_with_wrong_hash() {
        let crypto_utils = Pbkdf2CryptoUtils::default();
        let message = "test_message";

        let hash = crypto_utils.hash(message);
        assert!(hash.is_ok(), "Hashing should succeed.");
        let hash = hash.unwrap();

        let verify = crypto_utils.verify(message, &hash[1..]);
        assert!(
            verify.is_err(),
            "Verification should fail with incorrect hash."
        );
    }
}
