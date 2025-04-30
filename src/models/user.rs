use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Public representation of a user in the system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicIdentity {
    pub username: String,                // Unique @user identifier
    pub signing_public_key: Vec<u8>,    // Ed25519 public key (for signatures)
    pub encryption_public_key: Vec<u8>, // ECDH X25519 public key (for key exchange)
    pub signature: Vec<u8>,             // Signature of (username + keys) by the user
}

/// Local representation of a user (includes keys and private data).
#[derive(Debug)]
pub struct LocalUser {
    pub uuid: Uuid,                         // Local stable ID
    pub username: String,                   // Chosen @user
    pub signing_private_key: Vec<u8>,       // Ed25519 private key (PKCS#8 format)
    pub encryption_private_key: Vec<u8>,    // X25519 private key
    pub encryption_public_key: Vec<u8>,     // X25519 public key
}

impl PublicIdentity {
    /// Returns the signed message content (username + keys).
    pub fn signed_payload(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend(self.username.as_bytes());
        data.extend(&self.signing_public_key);
        data.extend(&self.encryption_public_key);
        data
    }
}
