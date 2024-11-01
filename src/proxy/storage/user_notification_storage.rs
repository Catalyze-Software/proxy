use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageQueryable, StorageUpdateable, USER_NOTIFICATIONS,
        USER_NOTIFICATIONS_MEMORY_ID,
    },
    StorageInsertableByKey,
};
use candid::Principal;
use canister_types::models::user_notifications::UserNotifications;
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
