use candid::Principal;
use ic_stable_structures::memory_manager::MemoryId;

use super::{
    storage_api::{HISTORY_CANISTER, HISTORY_CANISTER_MEMORY_ID},
    CellStorage, CellStorageRef,
};

pub struct HistoryCanisterStorage;

impl CellStorage<Principal> for HistoryCanisterStorage {
    const NAME: &'static str = "history_canister";

    fn storage() -> CellStorageRef<Principal> {
        &HISTORY_CANISTER
    }

    fn memory_id() -> MemoryId {
        HISTORY_CANISTER_MEMORY_ID
    }
}
