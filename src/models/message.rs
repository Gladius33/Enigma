use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Type of message being sent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    File,
    Image,
    Voice,
    Video,
    CallOffer,
    CallAnswer,
    CallHangup,
    GroupInvite,
}

/// Represents a payload transmitted between users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,                    // Unique identifier of the message
    pub sender: String,             // Username of sender (@user)
    pub receiver: String,           // Username or group id
    pub timestamp: DateTime<Utc>,   // Time of sending
    pub msg_type: MessageType,      // Type of message
    pub encrypted_payload: Vec<u8>, // The actual encrypted content
    pub nonce: Vec<u8>,             // Nonce used during encryption
    pub signature: Option<Vec<u8>>, // Optional signature (if applicable)
}
