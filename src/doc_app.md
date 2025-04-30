Overview
This module defines the core orchestration layer of the Enigma client. It centralizes the initialization and coordination of all subsystems:

Cryptography (key generation, encryption, ratcheting)

Peer-to-peer networking (WebRTC, signaling)

Local encrypted storage

User identity management

Message construction and transmission

It exposes an internal API (EnigmaApp) for use by the user interface (e.g., Android/Kotlin or other frontend bindings).

Components
EnigmaApp
Main structure representing the application’s runtime context. Holds all major components:


Field	Description
user	The local user object (identity, keys)
storage	Sled-based encrypted local storage backend
webrtc	WebRTC client used for peer-to-peer messaging
encryption	Symmetric encryption engine (ChaCha20-Poly1305)
ratchet	Double Ratchet algorithm instance (key evolution)
signing	Digital signature key (Ed25519)
Methods
init(storage_path: &str, username: &str) -> Result<EnigmaApp>
Initializes a new EnigmaApp:

Opens or creates encrypted local storage.

Generates Ed25519 signing key.

Initializes a ratchet instance with a dummy shared secret.

Prepares a symmetric encryption engine (ChaCha20).

Initializes WebRTC stack for peer communication.

Stores user metadata (locally only).

⚠️ Key exchange is mocked with [0u8; 32] until the ratchet receives a real shared secret.

send_message(&self, to: &str, plaintext: &[u8]) -> Result<Message>
Encrypts and sends a message to a peer via WebRTC:

Locks the current encryption engine.

Encrypts the plaintext message.

Constructs a Message object with nonce, encrypted payload, metadata.

Serializes the message with bincode and sends it over the data channel.

Returns the local Message struct.

Security Considerations
The encryption engine uses AEAD (ChaCha20-Poly1305) with unique nonce per message.

Key management is delegated to the ratchet instance (rotation, forward secrecy).

Signature key is used for identity verification (planned for future Message.signature).

No sensitive data is stored unencrypted on disk.

