use canister_types::models::api_error::ApiError;
use ic_stable_structures::memory_manager::MemoryId;

use super::{
    storage_api::{HISTORY_POINT, HISTORY_POINT_MEMORY_ID},
    CellStorage, CellStorageRef,
};

pub struct HistoryPointStorage;

impl CellStorage<u64> for HistoryPointStorage {
    const NAME: &'static str = "history_point";

    fn storage() -> CellStorageRef<u64> {
        &HISTORY_POINT
    }

    fn memory_id() -> MemoryId {
        HISTORY_POINT_MEMORY_ID
    }
}

impl HistoryPointStorage {
    pub fn get_next() -> Result<u64, ApiError> {
        let current = Self::get().unwrap_or(1);
        let next = current + 1;
        Self::set(next)?;
        Ok(next)
    }
}
