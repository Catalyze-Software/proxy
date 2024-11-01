use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdateable, GROUPS,
        GROUPS_MEMORY_ID,
    },
    ID_KIND_GROUPS,
};
use canister_types::models::group::Group;
use ic_stable_structures::memory_manager::MemoryId;

pub struct GroupStore;

impl Storage<u64, Group> for GroupStore {
    const NAME: &'static str = ID_KIND_GROUPS;

    fn storage() -> StaticStorageRef<u64, Group> {
        &GROUPS
    }

    fn memory_id() -> MemoryId {
        GROUPS_MEMORY_ID
    }
}

impl StorageQueryable<u64, Group> for GroupStore {}
impl StorageUpdateable<u64, Group> for GroupStore {}
impl StorageInsertable<Group> for GroupStore {}
