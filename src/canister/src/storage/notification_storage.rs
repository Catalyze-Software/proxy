use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageQueryable, StorageUpdatable, NOTIFICATIONS,
        NOTIFICATIONS_MEMORY_ID,
    },
    StorageInsertable,
};
use canister_types::models::notification::Notification;
use ic_stable_structures::memory_manager::MemoryId;

pub struct NotificationStore;

impl Storage<u64, Notification> for NotificationStore {
    const NAME: &'static str = "notifications";

    fn storage() -> StaticStorageRef<u64, Notification> {
        &NOTIFICATIONS
    }

    fn memory_id() -> MemoryId {
        NOTIFICATIONS_MEMORY_ID
    }
}

impl StorageQueryable<u64, Notification> for NotificationStore {}
impl StorageUpdatable<u64, Notification> for NotificationStore {}
impl StorageInsertable<Notification> for NotificationStore {}
