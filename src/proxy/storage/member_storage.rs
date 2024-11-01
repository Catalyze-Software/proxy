use super::storage_api::{
    StaticStorageRef, Storage, StorageInsertableByKey, StorageQueryable, StorageUpdateable,
    MEMBERS, MEMBERS_MEMORY_ID,
};
use candid::Principal;
use canister_types::models::member::Member;
use ic_stable_structures::memory_manager::MemoryId;

pub struct MemberStore;

impl Storage<Principal, Member> for MemberStore {
    const NAME: &'static str = "members";

    fn storage() -> StaticStorageRef<Principal, Member> {
        &MEMBERS
    }

    fn memory_id() -> MemoryId {
        MEMBERS_MEMORY_ID
    }
}

impl StorageQueryable<Principal, Member> for MemberStore {}
impl StorageUpdateable<Principal, Member> for MemberStore {}
impl StorageInsertableByKey<Principal, Member> for MemberStore {}
