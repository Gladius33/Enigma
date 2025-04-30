use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{
    Ed25519KeyPair, KeyPair, Signature, UnparsedPublicKey, ED25519,
};
use std::sync::Arc;

/// Represents an Ed25519 key pair for signing operations.
pub struct SigningKey {
    key_pair: Arc<Ed25519KeyPair>,
}

impl SigningKey {
    /// Generates a new Ed25519 key pair.
    pub fn generate() -> Result<Self, ring::error::Unspecified> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)?;
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;
        Ok(Self {
            key_pair: Arc::new(key_pair),
        })
    }

    /// Signs a message and returns the signature.
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.key_pair.sign(message)
    }

    /// Returns the public key bytes.
    pub fn public_key_bytes(&self) -> &[u8] {
        self.key_pair.public_key().as_ref()
    }
}

/// Verifies a signature using the provided public key.
pub fn verify_signature(
    public_key_bytes: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<(), ring::error::Unspecified> {
    let public_key = UnparsedPublicKey::new(&ED25519, public_key_bytes);
    public_key.verify(message, signature)
}
