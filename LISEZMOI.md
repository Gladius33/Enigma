# Enigma - Messagerie P2P Chiffrée de bout en bout (E2EE) incensurable

**Enigma** est une messagerie entièrement décentralisée, sans aucun serveur central.  
Elle repose sur des connexions **P2P** directes, avec un **chiffrement local** de tous les messages, appels, fichiers, et métadonnées.

---

## ⚙️ Objectifs du projet

- **Chiffrement de bout en bout (E2EE)** avec Perfect Forward Secrecy.
- **Aucune donnée stockée sur serveur**.
- **Communication directe P2P** (WebRTC).
- **Interopérabilité** via une base Rust appelable depuis Android et (plus tard) Flutter.
- **Création de compte distribuée et vérifiable** (par consensus de nœuds).
- **Support complet** :
  - Messagerie texte
  - Fichiers, images, vidéos
  - Appels audio/vidéo
  - Groupes et chaînes

---

## 📦 Technologies principales

| Composant | Stack |
|----------:|:------|
| Langage | Rust (natif, sécurisé, portable) |
| Crypto | AES-256-GCM ou ChaCha20-Poly1305, ECDSA, Double Ratchet |
| Réseau | WebRTC (via `webrtc-rs`), Tokio |
| Stockage local | Sled (base de données embarquée chiffrée) |
| Interfaçage Android | JNI (Rust ↔ Kotlin) |
| Signalisation | Réseau de nœuds Rust légers, interconnectés |
| Multi-plateforme | Interface prévue Flutter via FFI Rust |

---

## 🧩 Architecture

```text
+-----------------------------+
|    Interface utilisateur    |
|  (Kotlin ou Flutter via FFI)|
+-----------------------------+
|         JNI / FFI           |
+-----------------------------+
|        Core en Rust         |
| - Chiffrement               |
| - Réseau (WebRTC)           |
| - Gestion des messages      |
| - Appels audio/vidéo        |
| - Groupes et canaux         |
+-----------------------------+
|    Stockage local chiffré   |
+-----------------------------+
|     Réseau de signalisation |
|     (nœuds distribués)      |
+-----------------------------+
📁 Structure des dossiers
src/crypto/ → Chiffrement, signature, Perfect Forward Secrecy

src/network/ → WebRTC, signalisation, découverte de pairs

src/models/ → Données utilisateurs/messages/groupes

src/storage/ → Sauvegarde locale chiffrée

src/bindings/ → JNI Android (pont Rust ⇄ Kotlin)

nodes/ → Code des nœuds publics (signalisation distribuée)

🛡️ Sécurité
Chiffrement local avec AES-256-GCM ou ChaCha20-Poly1305

Signature ECDSA pour vérifier l'identité des contacts

Double Ratchet pour le renouvellement permanent des clés

Zero metadata leak (les métadonnées sont elles aussi chiffrées)

QR code vérifiable ou code de sécurité affiché à l'ajout d'un contact

Validation distribuée pour la création des identifiants @user

🚀 Lancement du projet
bash
Copier
Modifier
# Compilation native (Linux)
cargo build --release

# Compilation Android (librairie .so)
cargo ndk -t armeabi-v7a -o ./android_bindings/libs build --release
✨ À venir
Interface Kotlin (application Android native)

Interface Flutter (multi-plateforme)

Générateur de QR code avec clé publique signée

Client WebRTC audio/vidéo natif

Interface de groupes et canaux

🧪 Tests
Les tests unitaires sont dans chaque sous-module.
Un script de tests sera ajouté pour exécuter toutes les vérifications crypto, réseau, et persistance.

📜 Licence
Ce projet est sous licence MIT ou Apache 2.0 au choix.

🤝 Contribuer
Toutes les contributions sont bienvenues!
Les idées de modules, sécurité renforcée, audit et review cryptographiques sont particulièrement appréciés.
Ce projet est votre projet!
