use crate::crypto::encryption::EncryptionEngine;
use crate::crypto::ratchet::Ratchet;
use crate::crypto::signature::{SigningKey, verify_signature};
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
    /// Initializes a new EnigmaApp context with all components
    pub async fn init(storage_path: &str, username: &str) -> Result<Self> {
        let storage = Arc::new(Storage::open(storage_path)?);

        // Generate keys
        let signing_key = SigningKey::generate()?;
        let ratchet = Ratchet::new(&[0u8; 32]); // placeholder shared secret
        let encryption = EncryptionEngine::new(&[0u8; 32])?; // placeholder, must be derived from ratchet later

        let user = LocalUser {
            uuid: uuid::Uuid::new_v4(),
            username: username.to_owned(),
            signing_private_key: signing_key.key_pair.as_ref().private_key().as_ref().to_vec(),
            encryption_private_key: vec![0u8; 32],
            encryption_public_key: vec![0u8; 32],
        };

        let webrtc = Arc::new(WebRTCClient::new().await?);

        Ok(Self {
            user,
            storage,
            webrtc,
            encryption: Mutex::new(encryption),
            ratchet: Mutex::new(ratchet),
            signing: Arc::new(signing_key),
        })
    }

    /// Sends a message to a peer
    pub async fn send_message(&self, to: &str, plaintext: &[u8]) -> Result<Message> {
        let mut encryption = self.encryption.lock().await;
        let encrypted = encryption.encrypt(plaintext, b"message")?;
        let nonce = &encrypted[encrypted.len() - 12..]; // assume last 12 bytes

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
