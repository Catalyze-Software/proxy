use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageQueryable, StorageUpdateable, REFERRALS,
        REFERRAL_MEMORY_ID,
    },
    StorageInsertableByKey,
};
use candid::Principal;
use canister_types::models::referral::Referral;

use ic_stable_structures::memory_manager::MemoryId;

pub struct ReferralStore;

impl Storage<Principal, Referral> for ReferralStore {
    const NAME: &'static str = "referral";

    fn storage() -> StaticStorageRef<Principal, Referral> {
        &REFERRALS
    }

    fn memory_id() -> MemoryId {
        REFERRAL_MEMORY_ID
    }
}

impl StorageQueryable<Principal, Referral> for ReferralStore {}
impl StorageUpdateable<Principal, Referral> for ReferralStore {}
impl StorageInsertableByKey<Principal, Referral> for ReferralStore {}
