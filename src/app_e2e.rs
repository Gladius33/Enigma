#[cfg(test)]
mod e2e {
    use crate::crypto::encryption::EncryptionEngine;
    use crate::crypto::handshake::{generate_identity_bundle, x3dh_initiate, verify_signed_prekey};
    use crate::crypto::ratchet::Ratchet;

    #[tokio::test]
    async fn test_end_to_end_encryption_between_two_clients() {
        // Step 1: Generate identity and bundle for Bob
        let (bob_identity, bob_spk, bob_bundle) = generate_identity_bundle().expect("Bob bundle gen failed");

        // Step 2: Alice verifies Bob's bundle and initiates X3DH
        verify_signed_prekey(&bob_bundle).expect("Invalid signature");
        let x3dh_result = x3dh_initiate(&bob_bundle).expect("X3DH initiation failed");

        // Step 3: Alice sets up Ratchet + EncryptionEngine with derived key
        let mut alice_ratchet = Ratchet::new(&x3dh_result.shared_secret);
        let mut alice_enc = EncryptionEngine::new(&x3dh_result.shared_secret).expect("engine");

        // Step 4: Bob reconstructs the shared secret (mocked same key here)
        let mut bob_ratchet = Ratchet::new(&x3dh_result.shared_secret);
        let mut bob_enc = EncryptionEngine::new(&x3dh_result.shared_secret).expect("engine");

        // Step 5: Alice encrypts a message
        let msg = b"hello Bob!";
        let encrypted = alice_enc.encrypt(msg, b"context").expect("Alice encrypt");

        // Step 6: Bob decrypts it
        let decrypted = bob_enc.decrypt(&encrypted, b"context").expect("Bob decrypt");

        assert_eq!(decrypted, msg, "E2E encrypted-decrypted message should match");
    }
}
