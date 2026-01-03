//! Storage module

mod memory;

pub use memory::*;

use crate::cdm::{CdmRecord, ObjectRecord};
use crate::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// Storage backend trait
#[async_trait]
pub trait Storage: Send + Sync {
    // CDM operations
    async fn store_cdm(&self, cdm: CdmRecord) -> Result<()>;
    async fn get_cdm(&self, id: &str) -> Result<Option<CdmRecord>>;
    async fn list_cdms(&self) -> Result<Vec<CdmRecord>>;
    async fn withdraw_cdm(&self, id: &str) -> Result<()>;
    async fn cdm_count(&self) -> Result<usize>;
    
    // Object operations
    async fn store_object(&self, obj: ObjectRecord) -> Result<()>;
    async fn get_object(&self, id: &str) -> Result<Option<ObjectRecord>>;
    async fn list_objects(&self) -> Result<Vec<ObjectRecord>>;
    async fn withdraw_object(&self, id: &str) -> Result<()>;
    async fn object_count(&self) -> Result<usize>;
    
    // Message deduplication
    async fn has_seen_message(&self, message_id: &str) -> Result<bool>;
    async fn mark_message_seen(&self, message_id: &str) -> Result<()>;
}

/// Create storage from configuration
pub fn create_storage(storage_type: &str) -> Arc<dyn Storage> {
    match storage_type {
        "memory" | _ => Arc::new(MemoryStorage::new()),
    }
}
