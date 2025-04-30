use crate::server::PublicIdentity;
use std::collections::HashMap;
use reqwest::Client;
use anyhow::{Result, anyhow};

/// Checks if a given @user is available across a set of remote nodes.
/// If none of the nodes report this user as existing, we consider it safe to register.
pub async fn check_username_availability(
    username: &str,
    known_nodes: &[String],
) -> Result<bool> {
    let client = Client::new();
    let mut exists_somewhere = false;

    for node_url in known_nodes {
        let url = format!("{}/resolve/{}", node_url.trim_end_matches('/'), username);
        match client.get(&url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    exists_somewhere = true;
                    break;
                }
            }
            Err(_) => continue, // Node unreachable → ignore
        }
    }

    Ok(!exists_somewhere)
}

/// Propagates a new identity to known peers (broadcast).
pub async fn broadcast_identity(
    identity: &PublicIdentity,
    known_nodes: &[String],
) -> Result<()> {
    let client = Client::new();
    let payload = serde_json::to_string(identity)?;

    for node_url in known_nodes {
        let url = format!("{}/sync", node_url.trim_end_matches('/'));
        let mut map = HashMap::new();
        map.insert(identity.username.clone(), identity.clone());

        let _ = client.post(&url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&map)?)
            .send()
            .await;
    }

    Ok(())
}
