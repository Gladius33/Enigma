#[cfg(test)]
mod tests {
    use super::super::ratchet::Ratchet;
    use ring::rand::SystemRandom;

    // Ensure that a message can be encrypted and decrypted with the same ratchet
    #[test]
    fn test_encrypt_decrypt_same_instance() {
        let shared = b"shared_secret_for_test_123456789012";
        let mut r = Ratchet::new(shared);

        let msg = b"Top secret";
        let ciphertext = r.encrypt(msg).expect("encrypt failed");
        let plaintext = r.decrypt(&ciphertext).expect("decrypt failed");

        assert_eq!(plaintext, msg);
    }

    // Ensure that encryption produces different ciphertexts for same input (nonce-based)
    #[test]
    fn test_nonce_uniqueness() {
        let shared = b"another_test_key_which_is_shared";
        let mut r = Ratchet::new(shared);

        let msg = b"same content";
        let c1 = r.encrypt(msg).expect("encrypt failed");
        let c2 = r.encrypt(msg).expect("encrypt failed");

        assert_ne!(c1, c2, "Ciphertexts should differ due to random nonces");
    }

    // Check that decrypt fails if ciphertext is tampered
    #[test]
    fn test_decrypt_modified_ciphertext_fails() {
        let shared = b"some_shared_context_between_peers";
        let mut r = Ratchet::new(shared);

        let msg = b"confidential";
        let mut ct = r.encrypt(msg).expect("encrypt failed");

        ct[10] ^= 0xFF; // corrupt some byte
        let result = r.decrypt(&ct);

        assert!(result.is_err(), "Modified ciphertext should not decrypt");
    }

    // Test that DH ratchet resets keys and changes root
    #[test]
    fn test_dh_ratchet_changes_state() {
        let shared = b"consistent_key_material";
        let mut r1 = Ratchet::new(shared);
        let mut r2 = Ratchet::new(shared);

        let root_before = r1.root_key;

        // Perform a DH ratchet step on r1 using r2's public key
        let pk = r2.public_key();
        r1.dh_ratchet(pk).expect("ratchet failed");

        assert_ne!(r1.root_key, root_before, "Root key should change after DH ratchet");
        assert_eq!(r1.sending_chain_key, [0u8; 32], "Sending CK should reset");
        assert_eq!(r1.receiving_chain_key, [0u8; 32], "Receiving CK should reset");
    }
}
