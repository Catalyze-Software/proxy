use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageQueryable, StorageUpdateable, GROUP_TRANSFER_REQUESTS,
        GROUP_TRANSFER_REQUESTS_MEMORY_ID,
    },
    StorageInsertableByKey,
};
use canister_types::models::group_transfer_request::GroupTransferRequest;
use ic_stable_structures::memory_manager::MemoryId;

pub struct GroupTransferRequestStore;

impl Storage<u64, GroupTransferRequest> for GroupTransferRequestStore {
    const NAME: &'static str = "group_transfer_requests";

    fn storage() -> StaticStorageRef<u64, GroupTransferRequest> {
        &GROUP_TRANSFER_REQUESTS
    }

    fn memory_id() -> MemoryId {
        GROUP_TRANSFER_REQUESTS_MEMORY_ID
    }
}

impl StorageQueryable<u64, GroupTransferRequest> for GroupTransferRequestStore {}
impl StorageUpdateable<u64, GroupTransferRequest> for GroupTransferRequestStore {}
impl StorageInsertableByKey<u64, GroupTransferRequest> for GroupTransferRequestStore {}
