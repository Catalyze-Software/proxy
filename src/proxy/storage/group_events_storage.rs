use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageInsertableByKey, StorageQueryable, StorageUpdateable,
        GROUP_EVENTS, GROUP_EVENTS_MEMORY_ID,
    },
    ID_KIND_GROUP_EVENTS,
};
use canister_types::models::event_collection::EventCollection;
use ic_stable_structures::memory_manager::MemoryId;

pub struct GroupEventsStore;

impl Storage<u64, EventCollection> for GroupEventsStore {
    const NAME: &'static str = ID_KIND_GROUP_EVENTS;

    fn storage() -> StaticStorageRef<u64, EventCollection> {
        &GROUP_EVENTS
    }

    fn memory_id() -> MemoryId {
        GROUP_EVENTS_MEMORY_ID
    }
}

impl StorageQueryable<u64, EventCollection> for GroupEventsStore {}
impl StorageUpdateable<u64, EventCollection> for GroupEventsStore {}
impl StorageInsertableByKey<u64, EventCollection> for GroupEventsStore {}
