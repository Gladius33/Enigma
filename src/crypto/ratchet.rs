use ring::agreement::{EphemeralPrivateKey, PublicKey, UnparsedPublicKey, X25519};
use ring::rand::{SecureRandom, SystemRandom};
use ring::hkdf::{Salt, HKDF_SHA256};
use ring::aead::{LessSafeKey, UnboundKey, CHACHA20_POLY1305, Nonce, Aad, SealingKey, OpeningKey, BoundKey, NONCE_LEN};
use ring::aead;
use anyhow::{Result, anyhow};

/// Represents the state of the Double Ratchet algorithm.
pub struct Ratchet {
    root_key: [u8; 32],
    sending_chain_key: [u8; 32],
    receiving_chain_key: [u8; 32],
    dh_private_key: EphemeralPrivateKey,
    dh_public_key: PublicKey,
    peer_dh_public_key: Option<Vec<u8>>,
    rng: SystemRandom,
}

impl Ratchet {
    /// Initializes a new Ratchet instance with a shared secret.
    pub fn new(shared_secret: &[u8]) -> Self {
        let rng = SystemRandom::new();
        let dh_private_key = EphemeralPrivateKey::generate(&X25519, &rng).unwrap();
        let dh_public_key = dh_private_key.compute_public_key().unwrap();

        let salt = Salt::new(HKDF_SHA256, shared_secret);
        let prk = salt.extract(&[]);
        let okm = prk.expand(&[], HKDF_SHA256).unwrap();
        let mut root_key = [0u8; 32];
        okm.fill(&mut root_key).unwrap();

        Self {
            root_key,
            sending_chain_key: [0u8; 32],
            receiving_chain_key: [0u8; 32],
            dh_private_key,
            dh_public_key,
            peer_dh_public_key: None,
            rng,
        }
    }

    /// Returns the current public key for transmission to the peer.
    pub fn public_key(&self) -> &[u8] {
        self.dh_public_key.as_ref()
    }

    /// Performs a DH ratchet with the peer's public key.
    pub fn dh_ratchet(&mut self, peer_public_key: &[u8]) -> Result<()> {
        let peer_key = UnparsedPublicKey::new(&X25519, peer_public_key);
        let shared_secret = self.dh_private_key.agree(&peer_key)
            .map_err(|_| anyhow!("DH agreement failed"))?;

        let salt = Salt::new(HKDF_SHA256, &self.root_key);
        let prk = salt.extract(shared_secret.as_ref());
        let okm = prk.expand(&[], HKDF_SHA256).unwrap();
        okm.fill(&mut self.root_key).unwrap();

        // Reset sending/receiving chain keys after DH
        self.sending_chain_key = [0u8; 32];
        self.receiving_chain_key = [0u8; 32];
        self.peer_dh_public_key = Some(peer_public_key.to_vec());

        Ok(())
    }

    /// Derives the next key in a chain (sending or receiving).
    fn kdf_chain(chain_key: &[u8]) -> ([u8; 32], [u8; 32]) {
        let salt = Salt::new(HKDF_SHA256, chain_key);
        let prk = salt.extract(&[]);
        let mut new_chain_key = [0u8; 32];
        let mut message_key = [0u8; 32];
        prk.expand(&[b"chain"], HKDF_SHA256).unwrap().fill(&mut new_chain_key).unwrap();
        prk.expand(&[b"msg"], HKDF_SHA256).unwrap().fill(&mut message_key).unwrap();
        (new_chain_key, message_key)
    }

    /// Encrypts a message using the current sending chain.
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let (new_ck, mk) = Self::kdf_chain(&self.sending_chain_key);
        self.sending_chain_key = new_ck;

        let key = UnboundKey::new(&CHACHA20_POLY1305, &mk)?;
        let sealing_key = LessSafeKey::new(key);

        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce_bytes)?;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        let mut buffer = plaintext.to_vec();
        buffer.resize(buffer.len() + CHACHA20_POLY1305.tag_len(), 0);

        sealing_key.seal_in_place_append_tag(nonce, Aad::from(b"msg"), &mut buffer)?;

        let mut output = nonce_bytes.to_vec();
        output.extend_from_slice(&buffer);

        Ok(output)
    }

    /// Decrypts a message using the current receiving chain.
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < NONCE_LEN + CHACHA20_POLY1305.tag_len() {
            return Err(anyhow!("Invalid ciphertext length"));
        }

        let nonce = Nonce::try_assume_unique_for_key(&ciphertext[..NONCE_LEN])?;
        let mut buffer = ciphertext[NONCE_LEN..].to_vec();

        let (new_ck, mk) = Self::kdf_chain(&self.receiving_chain_key);
        self.receiving_chain_key = new_ck;

        let key = UnboundKey::new(&CHACHA20_POLY1305, &mk)?;
        let opening_key = LessSafeKey::new(key);

        let plaintext = opening_key
            .open_in_place(nonce, Aad::from(b"msg"), &mut buffer)?
            .to_vec();

        Ok(plaintext)
    }
}
