use reqwest::Client;
use anyhow::{Result, Context};
use std::collections::HashSet;
use std::time::Duration;

/// Attempts to contact a list of seed nodes and return reachable nodes
pub async fn discover_reachable_nodes(seed_nodes: &[String]) -> Result<HashSet<String>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;

    let mut reachable = HashSet::new();

    for url in seed_nodes {
        let endpoint = format!("{}/nodes", url.trim_end_matches('/'));
        match client.get(&endpoint).send().await {
            Ok(resp) if resp.status().is_success() => {
                reachable.insert(url.clone());
                if let Ok(list) = resp.json::<Vec<String>>().await {
                    for peer in list {
                        reachable.insert(peer);
                    }
                }
            }
            _ => continue,
        }
    }

    Ok(reachable)
}
