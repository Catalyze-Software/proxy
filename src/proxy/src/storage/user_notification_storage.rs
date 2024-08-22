use super::{
    storage_api::{
        Storage, StorageQueryable, StorageUpdateable, USER_NOTIFICATIONS,
        USER_NOTIFICATIONS_MEMORY_ID,
    },
    StorageInsertableByKey,
};
use candid::Principal;
use catalyze_shared::{user_notifications::UserNotifications, StaticStorageRef};
use ic_stable_structures::memory_manager::MemoryId;

pub struct UserNotificationStore;

impl Storage<Principal, UserNotifications> for UserNotificationStore {
    const NAME: &'static str = "user_notifications";

    fn storage() -> StaticStorageRef<Principal, UserNotifications> {
        &USER_NOTIFICATIONS
    }

    fn memory_id() -> MemoryId {
        USER_NOTIFICATIONS_MEMORY_ID
    }
}

impl StorageQueryable<Principal, UserNotifications> for UserNotificationStore {}
impl StorageUpdateable<Principal, UserNotifications> for UserNotificationStore {}
impl StorageInsertableByKey<Principal, UserNotifications> for UserNotificationStore {}
