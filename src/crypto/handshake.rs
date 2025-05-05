use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey, StaticSecret};
use ed25519_dalek::{Keypair as EdKeypair, PublicKey as EdPublicKey, Signature, Signer, Verifier};
use rand_core::OsRng;
use hkdf::Hkdf;
use sha2::Sha256;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct IdentityKey {
    pub keypair: EdKeypair,
}

#[derive(Clone)]
pub struct SignedPreKey {
    pub secret: StaticSecret,
    pub public: X25519PublicKey,
    pub signature: Signature,
}

#[derive(Clone)]
pub struct EphemeralKey {
    pub secret: EphemeralSecret,
    pub public: X25519PublicKey,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct X3DHBundle {
    pub identity_pub: EdPublicKey,
    pub spk_pub: X25519PublicKey,
    pub spk_signature: Signature,
}

pub struct X3DHInitResult {
    pub ephemeral: EphemeralKey,
    pub shared_secret: [u8; 32],
}

/// Generate identity + signed prekey bundle
pub fn generate_identity_bundle() -> Result<(IdentityKey, SignedPreKey, X3DHBundle)> {
    let id_key = IdentityKey {
        keypair: EdKeypair::generate(&mut OsRng),
    };

    let spk_secret = StaticSecret::new(OsRng);
    let spk_public = X25519PublicKey::from(&spk_secret);
    let spk_bytes = spk_public.as_bytes();

    let spk_signature = id_key.keypair.sign(spk_bytes);

    let spk = SignedPreKey {
        secret: spk_secret,
        public: spk_public,
        signature: spk_signature,
    };

    let bundle = X3DHBundle {
        identity_pub: id_key.keypair.public,
        spk_pub: spk_public,
        spk_signature,
    };

    Ok((id_key, spk, bundle))
}

/// Verify the signed prekey with the identity public key
pub fn verify_signed_prekey(bundle: &X3DHBundle) -> Result<()> {
    bundle
        .identity_pub
        .verify(bundle.spk_pub.as_bytes(), &bundle.spk_signature)
        .map_err(|_| anyhow!("Invalid SPK signature"))
}

/// Execute the X3DH initiator step to generate shared secret
pub fn x3dh_initiate(bundle: &X3DHBundle) -> Result<X3DHInitResult> {
    verify_signed_prekey(bundle)?;

    let ek = EphemeralKey {
        secret: EphemeralSecret::new(OsRng),
        public: X25519PublicKey::from(&EphemeralSecret::new(OsRng)),
    };

    let dh1 = ek.secret.diffie_hellman(&bundle.spk_pub);
    let dh2 = ek.secret.diffie_hellman(&X25519PublicKey::from(bundle.identity_pub.to_bytes()));
    let dh3 = StaticSecret::new(OsRng).diffie_hellman(&bundle.spk_pub); // simulate IK·SPK (only if known)

    let mut dh_concat = Vec::new();
    dh_concat.extend_from_slice(dh1.as_bytes());
    dh_concat.extend_from_slice(dh2.as_bytes());
    dh_concat.extend_from_slice(dh3.as_bytes());

    let hk = Hkdf::<Sha256>::new(None, &dh_concat);
    let mut okm = [0u8; 32];
    hk.expand(b"x3dh derived key", &mut okm)?;

    Ok(X3DHInitResult {
        ephemeral: ek,
        shared_secret: okm,
    })
}
