use std::{cell::RefCell, thread::LocalKey};

use catalyze_shared::api_error::ApiError;
use ic_stable_structures::{Cell, memory_manager::MemoryId, Storable};

use super::storage_api::Memory;

pub type CellStorageRef<V> = &'static LocalKey<RefCell<Cell<Option<V>, Memory>>>;

pub trait CellStorage<V: Storable + Clone + 'static> {
    const NAME: &'static str;

    fn memory_id() -> MemoryId;
    fn storage() -> CellStorageRef<V>;

    fn get() -> Result<V, ApiError> {
        Self::storage()
            .with(|data| data.borrow().get().clone())
            .ok_or_else(|| {
                ApiError::unexpected()
                    .add_message(&format!("Failed to get {}, not initialized", Self::NAME))
            })
    }

    fn set(value: V) -> Result<V, ApiError> {
        Self::storage()
            .with(|data| data.borrow_mut().set(Some(value.clone())))
            .map_err(|_| {
                ApiError::unexpected().add_message(&format!("Failed to set {}", Self::NAME))
            })?;
        Ok(value)
    }
}
