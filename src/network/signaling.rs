use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Message types for signaling between peers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalMessage {
    Offer {
        from: String,
        to: String,
        sdp: String,
    },
    Answer {
        from: String,
        to: String,
        sdp: String,
    },
    IceCandidate {
        from: String,
        to: String,
        candidate: String,
    },
    Join {
        username: String,
    },
    Leave {
        username: String,
    },
    Error {
        message: String,
    },
}

/// Represents a signaling session
#[derive(Debug)]
pub struct SignalingSession {
    pub id: Uuid,
    pub username: String,
    pub connected_at: DateTime<Utc>,
}

impl SignalingSession {
    /// Creates a new signaling session
    pub fn new(username: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            connected_at: Utc::now(),
        }
    }
}
