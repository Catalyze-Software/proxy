use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageInsertableByKey, StorageQueryable, StorageUpdateable,
        GROUP_MEMBERS, GROUP_MEMBERS_MEMORY_ID,
    },
    ID_KIND_GROUP_MEMBERS,
};
use canister_types::models::member_collection::MemberCollection;
use ic_stable_structures::memory_manager::MemoryId;

pub struct GroupMemberStore;

impl Storage<u64, MemberCollection> for GroupMemberStore {
    const NAME: &'static str = ID_KIND_GROUP_MEMBERS;

    fn storage() -> StaticStorageRef<u64, MemberCollection> {
        &GROUP_MEMBERS
    }

    fn memory_id() -> MemoryId {
        GROUP_MEMBERS_MEMORY_ID
    }
}

impl StorageQueryable<u64, MemberCollection> for GroupMemberStore {}
impl StorageUpdateable<u64, MemberCollection> for GroupMemberStore {}
impl StorageInsertableByKey<u64, MemberCollection> for GroupMemberStore {}
