use super::storage_api::{
    StaticStorageRef, Storage, StorageInsertableByKey, StorageQueryable, StorageUpdateable,
    ATTENDEES, ATTENDEES_MEMORY_ID,
};
use candid::Principal;
use canister_types::models::attendee::Attendee;

use ic_stable_structures::memory_manager::MemoryId;

pub struct AttendeeStore;

impl Storage<Principal, Attendee> for AttendeeStore {
    const NAME: &'static str = "attendees";

    fn storage() -> StaticStorageRef<Principal, Attendee> {
        &ATTENDEES
    }

    fn memory_id() -> MemoryId {
        ATTENDEES_MEMORY_ID
    }
}

impl StorageQueryable<Principal, Attendee> for AttendeeStore {}
impl StorageUpdateable<Principal, Attendee> for AttendeeStore {}
impl StorageInsertableByKey<Principal, Attendee> for AttendeeStore {}
