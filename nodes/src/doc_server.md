📄 Documentation — nodes/src/server.rs
🔧 Purpose
This file defines the main signaling node server for the Enigma network.
It provides REST endpoints used to:

Register and validate new @user identities.

Share and look up peer connection data (IP, port).

Synchronize state with other nodes (known users, presence info).

Expose a list of known nodes to help bootstrap the network.

This architecture ensures Enigma is:

Fully decentralized (anyone can run a node).

Unstoppable (no single point of failure).

Federated (nodes can synchronize and help propagate presence/data).

🚦 Supported REST Endpoints

Endpoint	Method	Description
/register	POST	Register a new @user identity (public key + signature).
/resolve/:user	GET	Resolve an @user to their latest known public identity.
/announce	POST	Announce IP/port presence of a peer (for WebRTC discovery).
/sync	POST	Synchronize local state with another node (users/presence).
/nodes	GET	Return a list of known peer nodes.
⚙️ Configuration — config.toml
The server loads its configuration from nodes/config.toml:

toml
Copier
Modifier
[node]
mode = "public"            # "public", "private", or "relay-only"
bind_address = "0.0.0.0"
bind_port = 1488
max_users = 1000

[sync]
enabled = true
initial_nodes = [
    "https://node1.enigma.net:1488",
    "https://node2.enigma.org:1488"
]
Available modes:

Mode	Description
public	Accept all incoming connections and share data with the network.
private	Only accept local clients, no external sync or public IP exposure.
relay-only	Accept incoming sync and resolve requests, but never expose own users/IPs.
🧠 Core Structures
PublicIdentity
Represents a public identity for a registered @user.

rust
Copier
Modifier
pub struct PublicIdentity {
    pub username: String,
    pub public_key: String,
    pub signature: String,
    pub timestamp: u64,
}
This structure is signed by the user's private key and validated by peers.

PeerPresence
Used to announce the presence of a peer (e.g., IP and port for WebRTC signaling).

rust
Copier
Modifier
pub struct PeerPresence {
    pub ip: String,
    pub port: u16,
    pub timestamp: u64,
}
IP/port information is ephemeral and refreshed via /announce.

AppState
Holds in-memory server state, shared across all HTTP handlers:

known_users: Map of registered users.

active_peers: Map of known IP/port presence info.

known_nodes: List of other nodes this one syncs with.

🔁 Sync Strategy
At startup, the node loads initial known peers from config.

On /register, it checks for duplicates, accepts if free.

On /sync, it merges remote user maps into its own.

On /announce, it records a peer's live IP/port.

Advanced features like quorum-based validation or TTL expiration are left for future work.

📦 Dependencies
actix-web for the HTTP server.

serde, serde_json, toml for config and data serialization.

std::sync::Mutex for shared memory-safe access to internal state.

🚧 Future Improvements
Add authentication tokens for syncing between trusted nodes.

Add automatic expiry for stale IPs / old identities.

Use persistent storage (e.g. sled or SQLite) for offline node reboot.

Add WebSocket-based broadcast sync instead of polling.
