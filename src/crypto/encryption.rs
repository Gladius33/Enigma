use ring::aead::{
    Aad, BoundKey, CHACHA20_POLY1305, LessSafeKey, Nonce, NonceSequence, OpeningKey,
    SealingKey, UnboundKey, AES_256_GCM,
};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};

/// Size of the symmetric key in bytes (256 bits)
pub const SYMMETRIC_KEY_LEN: usize = 32;

/// Size of the nonce in bytes (96 bits)
pub const NONCE_LEN: usize = 12;

/// AEAD encryption engine using ChaCha20-Poly1305 with unique nonce per message
pub struct EncryptionEngine {
    key: LessSafeKey,
    nonce_seq: NonceCounter,
}

/// Secure random-based nonce counter, ensures unique nonces per session
struct NonceCounter {
    counter: u128,
}

impl NonceCounter {
    fn new() -> Self {
        let rng = SystemRandom::new();
        let mut seed = [0u8; 12];
        rng.fill(&mut seed).unwrap();

        let mut counter_bytes = [0u8; 16];
        counter_bytes[4..].copy_from_slice(&seed);
        let counter = u128::from_be_bytes(counter_bytes);

        Self { counter }
    }

    fn next(&mut self) -> Result<Nonce, Unspecified> {
        let nonce_u128 = self.counter;
        self.counter += 1;

        let bytes = &nonce_u128.to_be_bytes()[4..]; // 12 bytes
        Ok(Nonce::assume_unique_for_key(bytes.try_into().unwrap()))
    }
}

impl EncryptionEngine {
    /// Initialize the encryption engine from a 32-byte symmetric key
    pub fn new(key_bytes: &[u8]) -> Result<Self, Unspecified> {
        assert_eq!(key_bytes.len(), SYMMETRIC_KEY_LEN);

        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, key_bytes)?;
        Ok(Self {
            key: LessSafeKey::new(unbound_key),
            nonce_seq: NonceCounter::new(),
        })
    }

    /// Encrypts data and returns a vector containing the ciphertext + tag
    pub fn encrypt(&mut self, plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, Unspecified> {
        let nonce = self.nonce_seq.next()?;
        let mut in_out = plaintext.to_vec();
        in_out.resize(plaintext.len() + CHACHA20_POLY1305.tag_len(), 0);

        self.key.seal_in_place_append_tag(
            nonce,
            Aad::from(associated_data),
            &mut in_out,
        )?;

        Ok(in_out)
    }

    /// Decrypts data and verifies authenticity
    pub fn decrypt(&self, ciphertext: &[u8], nonce_bytes: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, Unspecified> {
        let nonce = Nonce::try_assume_unique_for_key(nonce_bytes.try_into().unwrap())?;
        let mut in_out = ciphertext.to_vec();

        let plaintext = self.key.open_in_place(nonce, Aad::from(associated_data), &mut in_out)?;
        Ok(plaintext.to_vec())
    }
}
