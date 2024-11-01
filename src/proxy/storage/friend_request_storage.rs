use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdateable,
        FRIEND_REQUEST, FRIEND_REQUESTS_MEMORY_ID,
    },
    ID_KIND_FRIEND_REQUESTS,
};
use canister_types::models::friend_request::FriendRequest;
use ic_stable_structures::memory_manager::MemoryId;

pub struct FriendRequestStore;

impl Storage<u64, FriendRequest> for FriendRequestStore {
    const NAME: &'static str = ID_KIND_FRIEND_REQUESTS;

    fn storage() -> StaticStorageRef<u64, FriendRequest> {
        &FRIEND_REQUEST
    }

    fn memory_id() -> MemoryId {
        FRIEND_REQUESTS_MEMORY_ID
    }
}

impl StorageQueryable<u64, FriendRequest> for FriendRequestStore {}
impl StorageUpdateable<u64, FriendRequest> for FriendRequestStore {}
impl StorageInsertable<FriendRequest> for FriendRequestStore {}
