//! In-memory storage implementation

use crate::cdm::{CdmRecord, ObjectRecord};
use crate::storage::Storage;
use crate::{Error, Result};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

/// In-memory storage backend
pub struct MemoryStorage {
    cdms: RwLock<HashMap<String, CdmRecord>>,
    objects: RwLock<HashMap<String, ObjectRecord>>,
    seen_messages: RwLock<HashSet<String>>,
}

impl MemoryStorage {
    /// Create a new in-memory storage
    pub fn new() -> Self {
        Self {
            cdms: RwLock::new(HashMap::new()),
            objects: RwLock::new(HashMap::new()),
            seen_messages: RwLock::new(HashSet::new()),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn store_cdm(&self, cdm: CdmRecord) -> Result<()> {
        let mut cdms = self.cdms.write().map_err(|_| Error::Storage("lock poisoned".into()))?;
        cdms.insert(cdm.cdm_id.clone(), cdm);
        Ok(())
    }

    async fn get_cdm(&self, id: &str) -> Result<Option<CdmRecord>> {
        let cdms = self.cdms.read().map_err(|_| Error::Storage("lock poisoned".into()))?;
        Ok(cdms.get(id).cloned())
    }

    async fn list_cdms(&self) -> Result<Vec<CdmRecord>> {
        let cdms = self.cdms.read().map_err(|_| Error::Storage("lock poisoned".into()))?;
        Ok(cdms.values().cloned().collect())
    }

    async fn withdraw_cdm(&self, id: &str) -> Result<()> {
        let mut cdms = self.cdms.write().map_err(|_| Error::Storage("lock poisoned".into()))?;
        if cdms.remove(id).is_none() {
            return Err(Error::NotFound(format!("CDM not found: {}", id)));
        }
        Ok(())
    }

    async fn cdm_count(&self) -> Result<usize> {
        let cdms = self.cdms.read().map_err(|_| Error::Storage("lock poisoned".into()))?;
        Ok(cdms.len())
    }

    async fn store_object(&self, obj: ObjectRecord) -> Result<()> {
        let mut objects = self.objects.write().map_err(|_| Error::Storage("lock poisoned".into()))?;
        objects.insert(obj.object_id.clone(), obj);
        Ok(())
    }

    async fn get_object(&self, id: &str) -> Result<Option<ObjectRecord>> {
        let objects = self.objects.read().map_err(|_| Error::Storage("lock poisoned".into()))?;
        Ok(objects.get(id).cloned())
    }

    async fn list_objects(&self) -> Result<Vec<ObjectRecord>> {
        let objects = self.objects.read().map_err(|_| Error::Storage("lock poisoned".into()))?;
        Ok(objects.values().cloned().collect())
    }

    async fn withdraw_object(&self, id: &str) -> Result<()> {
        let mut objects = self.objects.write().map_err(|_| Error::Storage("lock poisoned".into()))?;
        if objects.remove(id).is_none() {
            return Err(Error::NotFound(format!("Object not found: {}", id)));
        }
        Ok(())
    }

    async fn object_count(&self) -> Result<usize> {
        let objects = self.objects.read().map_err(|_| Error::Storage("lock poisoned".into()))?;
        Ok(objects.len())
    }

    async fn has_seen_message(&self, message_id: &str) -> Result<bool> {
        let seen = self.seen_messages.read().map_err(|_| Error::Storage("lock poisoned".into()))?;
        Ok(seen.contains(message_id))
    }

    async fn mark_message_seen(&self, message_id: &str) -> Result<()> {
        let mut seen = self.seen_messages.write().map_err(|_| Error::Storage("lock poisoned".into()))?;
        seen.insert(message_id.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cdm::generate_demo_cdm;

    #[tokio::test]
    async fn test_cdm_storage() {
        let storage = MemoryStorage::new();
        let cdm = generate_demo_cdm();
        let cdm_id = cdm.cdm_id.clone();
        
        // Store
        storage.store_cdm(cdm.clone()).await.unwrap();
        assert_eq!(storage.cdm_count().await.unwrap(), 1);
        
        // Get
        let retrieved = storage.get_cdm(&cdm_id).await.unwrap().unwrap();
        assert_eq!(retrieved.cdm_id, cdm_id);
        
        // List
        let all = storage.list_cdms().await.unwrap();
        assert_eq!(all.len(), 1);
        
        // Withdraw
        storage.withdraw_cdm(&cdm_id).await.unwrap();
        assert_eq!(storage.cdm_count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_message_seen() {
        let storage = MemoryStorage::new();
        
        assert!(!storage.has_seen_message("msg-1").await.unwrap());
        storage.mark_message_seen("msg-1").await.unwrap();
        assert!(storage.has_seen_message("msg-1").await.unwrap());
    }
}
