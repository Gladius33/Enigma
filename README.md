# Enigma
Uncensorable &amp; End to End encrypted P2P private messaging

**Enigma** is a fully decentralized secure messaging system.  
There is no central server — all data is encrypted and stored locally.  
All communications are **peer-to-peer**, and **end-to-end encrypted (E2EE)**.

---

## ⚙️ Project Goals

- **End-to-End Encryption (E2EE)** with Perfect Forward Secrecy.
- **No data stored on servers** — ever.
- **Direct peer-to-peer communication** via WebRTC.
- **Interoperability** through a Rust core accessible from Android or Flutter.
- **Account creation by distributed consensus** — uniqueness guaranteed across the network.
- **Full feature support**:
  - Text messaging
  - File/image/video transfer
  - Voice/video calls
  - Private groups and broadcast channels

---

## 📦 Technologies Used

| Component     | Stack Used |
|---------------|------------|
| Core Language | Rust (safe, portable, memory-efficient) |
| Cryptography  | AES-256-GCM / ChaCha20-Poly1305, ECDSA, Double Ratchet |
| Networking    | WebRTC (`webrtc-rs`), Tokio |
| Local storage | Sled (encrypted embedded DB) |
| Android       | JNI bindings (Rust ⇄ Kotlin) |
| Signaling     | Lightweight distributed Rust-based nodes |
| Cross-platform | Optional Flutter interface via FFI |

---

## 🧩 Architecture Overview

```text
+-----------------------------+
|     User Interface (UI)     |
|    (Kotlin or Flutter FFI)  |
+-----------------------------+
|         JNI / FFI           |
+-----------------------------+
|        Rust Core Logic      |
| - Encryption                |
| - Peer-to-peer comm (WebRTC)|
| - Message handling          |
| - File transfer             |
| - Audio/video calls         |
| - Group/channel mgmt        |
+-----------------------------+
|  Encrypted Local Database   |
+-----------------------------+
|    Distributed Signaling    |
|         (Rust nodes)        |
+-----------------------------+
📁 Project Directory Structure
src/crypto/ → encryption, signature, Perfect Forward Secrecy

src/network/ → WebRTC, signaling, peer discovery

src/models/ → user/message/group data models

src/storage/ → secure local persistence

src/bindings/ → Android JNI bridge (Rust ↔ Kotlin)

nodes/ → Signaling node logic (Rust microservice)

🛡️ Security Principles
AES-256-GCM or ChaCha20-Poly1305 encryption for all data

ECDSA digital signatures to authenticate identities

Double Ratchet Algorithm for Perfect Forward Secrecy

Zero metadata exposure (metadata also encrypted)

Manual key fingerprint verification via QR code or code comparison

Consensus-based @user ID registration to prevent spoofing or duplication

🚀 Getting Started
bash
Copier
Modifier
# Native build (e.g., Linux)
cargo build --release

# Android build (as shared library .so)
cargo ndk -t armeabi-v7a -o ./android_bindings/libs build --release
✨ Coming Soon
Kotlin interface (Android native client)

Flutter interface (cross-platform UI)

QR code generator for user identity

Native WebRTC audio/video call handling

Group and channel message routing

🧪 Testing
Each core module includes its own unit tests.
An integrated test suite is planned to cover crypto, networking, and persistence.

📜 License
This project is licensed under MIT or Apache 2.0 (dual license).

🤝 Contributing
Pull Requests are welcome.
We especially appreciate ideas and help around:

cryptographic correctness,

protocol improvements,

secure transport audit.
