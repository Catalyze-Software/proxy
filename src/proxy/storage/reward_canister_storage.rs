use candid::Principal;
use ic_stable_structures::memory_manager::MemoryId;

use super::{
    storage_api::{REWARD_CANISTER, REWARD_CANISTER_MEMORY_ID},
    CellStorage, CellStorageRef,
};

pub struct RewardCanisterStorage;

impl CellStorage<Principal> for RewardCanisterStorage {
    const NAME: &'static str = "reward_canister";

    fn storage() -> CellStorageRef<Principal> {
        &REWARD_CANISTER
    }

    fn memory_id() -> MemoryId {
        REWARD_CANISTER_MEMORY_ID
    }
}
