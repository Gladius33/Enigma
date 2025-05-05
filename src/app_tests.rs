#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use async_trait::async_trait;

    // Test EnigmaApp initialization with X3DH key derivation
    #[tokio::test]
    async fn test_app_initialization() {
        let test_path = "test_data/enigma_init";
        if Path::new(test_path).exists() {
            fs::remove_dir_all(test_path).unwrap();
        }

        let result = EnigmaApp::init(test_path, "@testuser").await;
        assert!(result.is_ok(), "Failed to initialize EnigmaApp");

        let app = result.unwrap();

        assert_eq!(app.user.username, "@testuser");

        // Validate signature key length
        assert_eq!(app.signing.key_pair.public.as_bytes().len(), 32);
        assert_eq!(app.user.signing_private_key.len(), 32);

        // Validate encryption key pair (X25519)
        assert_eq!(app.user.encryption_private_key.len(), 32);
        assert_eq!(app.user.encryption_public_key.len(), 32);

        // Ensure encryption works
        let message = b"hello world!";
        let ciphertext = {
            let enc = app.encryption.lock().await;
            enc.encrypt(message, b"test").expect("Encryption failed")
        };
        assert!(ciphertext.len() > message.len());

        fs::remove_dir_all(test_path).unwrap();
    }

    // Mock implementation of WebRTC to capture outgoing data
    struct MockWebRTCClient {
        pub last_sent: Arc<Mutex<Option<Vec<u8>>>>,
    }

    #[async_trait]
    impl crate::network::webrtc_client::WebRTC for MockWebRTCClient {
        async fn send_message(&self, data: &[u8]) -> anyhow::Result<()> {
            let mut lock = self.last_sent.lock().await;
            *lock = Some(data.to_vec());
            Ok(())
        }
    }

    // Test that send_message encrypts and emits a message correctly
    #[tokio::test]
    async fn test_send_message_encryption() {
        let test_path = "test_data/enigma_msg";
        if Path::new(test_path).exists() {
            fs::remove_dir_all(test_path).unwrap();
        }

        let app = EnigmaApp::init(test_path, "@sender").await.unwrap();

        let mock_sent = Arc::new(Mutex::new(None));
        let mock_webrtc = Arc::new(MockWebRTCClient {
            last_sent: Arc::clone(&mock_sent),
        });

        let app = EnigmaApp {
            webrtc: mock_webrtc.clone(),
            ..app
        };

        let msg = app.send_message("@recipient", b"Secret!").await.unwrap();
        assert_eq!(msg.sender, "@sender");
        assert_eq!(msg.receiver, "@recipient");
        assert!(msg.encrypted_payload.len() > 0);
        assert_eq!(msg.nonce.len(), 12);

        let sent_data = mock_sent.lock().await.clone();
        assert!(sent_data.is_some());

        fs::remove_dir_all(test_path).unwrap();
    }
}

