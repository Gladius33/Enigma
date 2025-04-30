use sled::{Db, IVec};
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Debug)]
pub struct NodeDatabase {
    db: Db,
}

impl NodeDatabase {
    /// Open or create the local Sled database at a given path
    pub fn open(path: &str) -> Result<Self> {
        let db = sled::open(path).context("Failed to open node database")?;
        Ok(Self { db })
    }

    /// Store a serializable object under a key
    pub fn store<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let encoded = bincode::serialize(value)?;
        self.db.insert(key.as_bytes(), encoded)?;
        Ok(())
    }

    /// Load a deserializable object from a key
    pub fn load<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        if let Some(ivec) = self.db.get(key)? {
            let decoded = bincode::deserialize(&ivec)?;
            Ok(Some(decoded))
        } else {
            Ok(None)
        }
    }

    /// Delete a key
    pub fn delete(&self, key: &str) -> Result<()> {
        self.db.remove(key)?;
        Ok(())
    }

    /// Load all values matching a prefix (e.g. "user/", "peer/")
    pub fn load_all_with_prefix<T: DeserializeOwned>(&self, prefix: &str) -> Result<Vec<T>> {
        let mut results = Vec::new();
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (_key, value) = item?;
            let decoded: T = bincode::deserialize(&value)?;
            results.push(decoded);
        }
        Ok(results)
    }

    /// Flush changes to disk
    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }
}
