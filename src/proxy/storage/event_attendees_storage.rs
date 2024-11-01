use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageInsertableByKey, StorageQueryable, StorageUpdateable,
        EVENT_ATTENDEES, EVENT_ATTENDEES_MEMORY_ID,
    },
    ID_KIND_EVENT_ATTENDEES,
};
use canister_types::models::member_collection::MemberCollection;
use ic_stable_structures::memory_manager::MemoryId;

pub struct EventAttendeeStore;

impl Storage<u64, MemberCollection> for EventAttendeeStore {
    const NAME: &'static str = ID_KIND_EVENT_ATTENDEES;

    fn storage() -> StaticStorageRef<u64, MemberCollection> {
        &EVENT_ATTENDEES
    }

    fn memory_id() -> MemoryId {
        EVENT_ATTENDEES_MEMORY_ID
    }
}

impl StorageQueryable<u64, MemberCollection> for EventAttendeeStore {}
impl StorageUpdateable<u64, MemberCollection> for EventAttendeeStore {}
impl StorageInsertableByKey<u64, MemberCollection> for EventAttendeeStore {}
