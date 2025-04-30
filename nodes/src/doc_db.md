📄 Documentation — nodes/src/db.rs
🔧 Purpose
This module provides a lightweight local persistence layer for Enigma signaling nodes using sled — a fast, embedded, key-value database written in Rust.

It is used to store:

Public identities (@user → PublicIdentity)

Peer presence data (IP → PeerPresence)

Any other serializable state for recovery or inspection

🧱 Main Structure
pub struct NodeDatabase
Represents a wrapper over a sled::Db instance, providing high-level operations like store/load/delete/scan.

🛠️ Methods
open(path: &str) -> Result<Self>
Opens or creates a new Sled database at the given path (e.g. "nodes/data").

store<T: Serialize>(key: &str, value: &T) -> Result<()>
Stores a serializable object under the specified key (e.g. "user/@alice").

load<T: DeserializeOwned>(key: &str) -> Result<Option<T>>
Loads an object from a key and deserializes it using bincode.

delete(key: &str) -> Result<()>
Removes the key and its value from the database.

load_all_with_prefix<T>(prefix: &str) -> Result<Vec<T>>
Scans the database and returns all values whose key starts with the given prefix.
Useful for batch loading all users ("user/") or all peers ("peer/").

flush() -> Result<()>
Ensures all pending changes are written to disk immediately.

🧪 Example Usage
rust
Copier
Modifier
let db = NodeDatabase::open("nodes/data")?;

db.store("user/@bob", &identity)?;
db.store("peer/192.168.1.42", &presence)?;

let user: Option<PublicIdentity> = db.load("user/@bob")?;
let all_users = db.load_all_with_prefix::<PublicIdentity>("user/")?;
📦 Dependencies
sled: high-performance embedded K/V store.

bincode: binary serialization of Rust structures.

anyhow: ergonomic error handling.

🔐 Notes
All data is stored in binary form.

The key convention (e.g. "user/...", "peer/...") is internal but recommended to distinguish record types.

This module does not implement TTL or expiration — these should be managed at the application level.
