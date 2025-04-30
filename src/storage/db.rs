use sled::{Db, IVec};
use std::path::Path;
use anyhow::{Result, Context};

/// Represents the local encrypted storage engine.
pub struct Storage {
    db: Db,
}

impl Storage {
    /// Opens or creates a new encrypted database at the specified path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path).context("Failed to open sled database")?;
        Ok(Self { db })
    }

    /// Stores a value under the given key.
    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.insert(key, value)?;
        Ok(())
    }

    /// Retrieves a value for the given key.
    pub fn get(&self, key: &[u8]) -> Result<Option<IVec>> {
        let result = self.db.get(key)?;
        Ok(result)
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
