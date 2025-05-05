# 🧪 Enigma — Signaling Node Test Suite

This file documents all automated tests currently implemented for the Enigma signaling node.

---

## ✅ File: `server_tests.rs`

| Endpoint        | Description                                 |
|-----------------|---------------------------------------------|
| `/check_user`   | Verifies the existence of a given `@user`. |
| `/announce`     | Stores the presence (IP + port) of a peer. |
| `/register`     | Creates a new `@user` identity.            |
| `/resolve`      | Retrieves the identity for a given `@user`.|
| `/sync`         | Merges a list of `@user`s into local state.|
| `/nodes`        | Returns known peer node URLs.              |

---

## ✅ File: `consensus_tests.rs`

| Function                   | Description                                                       |
|----------------------------|-------------------------------------------------------------------|
| `check_username_availability` | Checks if an `@user` exists across mocked nodes (`/check_user`).  |
| `broadcast_identity`          | Sends a new identity to remote `/sync` endpoints (mocked).        |

---

## 🛠 Notes

- All tests are written with `actix-rt` and `actix-web::test`.
- Mock nodes are spun up locally and respond with controlled data.
- Consensus logic is fully tested in isolation (no real network needed).

---

## 🚀 To run all tests

From the root of the project, run:

cargo test --all


You can also test a specific file:

cargo test --test server_tests
cargo test --test consensus_tests
