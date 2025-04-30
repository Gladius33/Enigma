use crate::app::EnigmaApp;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Abstraction over a frontend environment
pub struct UI {
    app: Arc<Mutex<EnigmaApp>>,
}

impl UI {
    /// Create the UI layer, connected to the core EnigmaApp
    pub fn new(app: Arc<Mutex<EnigmaApp>>) -> Self {
        Self { app }
    }

    /// Handle a text message being composed and sent to a recipient
    pub async fn send_text_message(&self, to: &str, content: &str) -> Result<()> {
        let mut app = self.app.lock().await;
        let _ = app.send_message(to, content.as_bytes()).await?;
        Ok(())
    }

    /// Placeholder for future UI trigger: display incoming message
    pub async fn on_incoming_message(&self, from: &str, content: &str) {
        println!("[{}] says: {}", from, content);
        // Could later push to a channel, emit signal, update view, etc.
    }

    /// Placeholder for notification when new peer connection established
    pub async fn on_peer_connected(&self, peer: &str) {
        println!("Connected with peer: {}", peer);
    }

    /// Placeholder for errors or alerts
    pub fn notify_error(&self, msg: &str) {
        eprintln!("[Error] {}", msg);
    }
}
