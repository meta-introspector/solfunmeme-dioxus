use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Digest, Sha256};
use rand::RngCore;

// WalletManager and associated structs have been moved to solfunmeme_models