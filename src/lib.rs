// ========== External dependencies ==========
extern crate anyhow;
extern crate serde;
extern crate uuid;

// ========== Module declarations ==========
pub mod app;
pub mod crypto;
pub mod models;
pub mod network;
pub mod storage;
pub mod ui;
pub mod bindings;

// ========== Re-exports for high-level usage ==========
pub use app::EnigmaApp;
pub use crypto::{encryption, ratchet, signature};
pub use models::{user, message, group};
pub use network::{signaling, webrtc_client, discovery};
pub use storage::{db, persistence};
pub use ui::UI;
pub use bindings::android;
