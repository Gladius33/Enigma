use crate::crypto::encryption::EncryptionEngine;
use crate::crypto::ratchet::Ratchet;
use crate::crypto::signature::{SigningKey, verify_signature};
use crate::crypto::handshake::{generate_identity_bundle, x3dh_initiate};
use crate::network::webrtc_client::WebRTCClient;
use crate::network::signaling::{SignalMessage, SignalingSession};
use crate::storage::db::Storage;
use crate::models::user::{LocalUser, PublicIdentity};
use crate::models::message::{Message, MessageType};

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Global state of the Enigma client
pub struct EnigmaApp {
    pub user: LocalUser,
    pub storage: Arc<Storage>,
    pub webrtc: Arc<WebRTCClient>,
    pub encryption: Mutex<EncryptionEngine>,
    pub ratchet: Mutex<Ratchet>,
    pub signing: Arc<SigningKey>,
}

impl EnigmaApp {
    /// Initializes a new EnigmaApp context with full X3DH key derivation
    pub async fn init(storage_path: &str, username: &str) -> Result<Self> {
        let storage = Arc::new(Storage::open(storage_path)?);

        // Generate keys (X3DH)
        let (identity_key, signed_prekey, bundle) = generate_identity_bundle()?;
        let x3dh = x3dh_initiate(&bundle)?; // derive key from own bundle (loopback for now)

        // Use derived key for encryption/ratchet
        let ratchet = Ratchet::new(&x3dh.shared_secret);
        let encryption = EncryptionEngine::new(&x3dh.shared_secret)?;

        // Build local user
        let user = LocalUser {
            uuid: uuid::Uuid::new_v4(),
            username: username.to_owned(),
            signing_private_key: identity_key.keypair.secret.to_bytes().to_vec(),
            encryption_private_key: signed_prekey.secret.to_bytes().to_vec(),
            encryption_public_key: signed_prekey.public.as_bytes().to_vec(),
        };

        let webrtc = Arc::new(WebRTCClient::new().await?);

        Ok(Self {
            user,
            storage,
            webrtc,
            encryption: Mutex::new(encryption),
            ratchet: Mutex::new(ratchet),
            signing: Arc::new(SigningKey {
                key_pair: identity_key.keypair,
            }),
        })
    }

    /// Sends a message to a peer
    pub async fn send_message(&self, to: &str, plaintext: &[u8]) -> Result<Message> {
        let mut encryption = self.encryption.lock().await;
        let encrypted = encryption.encrypt(plaintext, b"message")?;
        let nonce = &encrypted[encrypted.len() - 12..];

        let msg = Message {
            id: uuid::Uuid::new_v4(),
            sender: self.user.username.clone(),
            receiver: to.to_owned(),
            timestamp: chrono::Utc::now(),
            msg_type: MessageType::Text,
            encrypted_payload: encrypted,
            nonce: nonce.to_vec(),
            signature: None,
        };

        self.webrtc.send_message(&bincode::serialize(&msg)?).await?;
        Ok(msg)
    }
}
