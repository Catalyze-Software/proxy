use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdateable, BOOSTED,
        BOOSTED_MEMORY_ID,
    },
    ID_KIND_BOOSTED,
};
use canister_types::models::boosted::Boost;
use ic_stable_structures::memory_manager::MemoryId;

pub struct BoostedStore;

impl Storage<u64, Boost> for BoostedStore {
    const NAME: &'static str = ID_KIND_BOOSTED;

    fn storage() -> StaticStorageRef<u64, Boost> {
        &BOOSTED
    }

    fn memory_id() -> MemoryId {
        BOOSTED_MEMORY_ID
    }
}

impl StorageQueryable<u64, Boost> for BoostedStore {}
impl StorageUpdateable<u64, Boost> for BoostedStore {}
impl StorageInsertable<Boost> for BoostedStore {}
