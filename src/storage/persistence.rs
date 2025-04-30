use sled::{Db, IVec};
use std::path::Path;
use anyhow::{Result, Context};
use serde::{Serialize, de::DeserializeOwned};

/// Represents the local encrypted storage engine.
pub struct Persistence {
    db: Db,
}

impl Persistence {
    /// Opens or creates a new encrypted database at the specified path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path).context("Failed to open sled database")?;
        Ok(Self { db })
    }

    /// Stores a serializable value under the given key.
    pub fn put<T: Serialize>(&self, key: &[u8], value: &T) -> Result<()> {
        let serialized = bincode::serialize(value)?;
        self.db.insert(key, serialized)?;
        Ok(())
    }

    /// Retrieves a deserializable value for the given key.
    pub fn get<T: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<T>> {
        if let Some(ivec) = self.db.get(key)? {
            let deserialized = bincode::deserialize(&ivec)?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    /// Deletes a value for the given key.
    pub fn delete(&self, key: &[u8]) -> Result<()> {
        self.db.remove(key)?;
        Ok(())
    }

    /// Flushes the database to ensure all operations are persisted.
    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }
}
