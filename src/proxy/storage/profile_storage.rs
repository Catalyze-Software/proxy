use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageQueryable, StorageUpdateable, PROFILES,
        PROFILES_MEMORY_ID,
    },
    StorageInsertableByKey,
};
use candid::Principal;
use canister_types::models::profile::Profile;

use ic_stable_structures::memory_manager::MemoryId;

pub struct ProfileStore;

impl Storage<Principal, Profile> for ProfileStore {
    const NAME: &'static str = "profiles";

    fn storage() -> StaticStorageRef<Principal, Profile> {
        &PROFILES
    }

    fn memory_id() -> MemoryId {
        PROFILES_MEMORY_ID
    }
}

impl StorageQueryable<Principal, Profile> for ProfileStore {}
impl StorageUpdateable<Principal, Profile> for ProfileStore {}
impl StorageInsertableByKey<Principal, Profile> for ProfileStore {}
