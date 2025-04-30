use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Role of a member in a group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GroupRole {
    Owner,
    Admin,
    Member,
    ReadOnly,
}

/// Member of a group (user with rights)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub username: String,         // @user
    pub role: GroupRole,          // Rights in the group
    pub joined_at: DateTime<Utc>, // When this user joined
}

/// Secure group metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,                     // Unique group ID
    pub name: String,                 // Display name
    pub is_channel: bool,            // If true: one-to-many, not chat
    pub created_at: DateTime<Utc>,   // Group creation date
    pub creator: String,             // @user of group creator
    pub members: Vec<GroupMember>,   // Member list
    pub encrypted_key: Vec<u8>,      // Shared group key encrypted for this client
}
