#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[tokio::test]
    async fn test_app_initialization() {
        let test_path = "test_data/enigma_init";
        if Path::new(test_path).exists() {
            fs::remove_dir_all(test_path).unwrap();
        }

        let result = EnigmaApp::init(test_path, "@testuser").await;
        assert!(result.is_ok(), "Failed to initialize EnigmaApp");

        let app = result.unwrap();

        // Vérification du nom d'utilisateur
        assert_eq!(app.user.username, "@testuser");

        // Vérification des clés de signature
        assert_eq!(app.signing.key_pair.public.as_bytes().len(), 32);
        assert_eq!(app.user.signing_private_key.len(), 32);

        // Vérification des clés de chiffrement (X25519)
        assert_eq!(app.user.encryption_private_key.len(), 32);
        assert_eq!(app.user.encryption_public_key.len(), 32);

        // Test d’encryption réelle avec la clé partagée
        let message = b"hello world!";
        let ciphertext = {
            let enc = app.encryption.lock().await;
            enc.encrypt(message, b"test").expect("Encryption failed")
        };
        assert!(ciphertext.len() > message.len());

        // Cleanup
        fs::remove_dir_all(test_path).unwrap();
    }
}
