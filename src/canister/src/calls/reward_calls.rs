use crate::{
    helpers::guards::is_developer,
    logic::reward_buffer_logic::send_reward_data,
    storage::{
        reward_canister_storage::RewardCanisterStorage, CellStorage, GroupStore, ProfileStore,
        RewardBufferStore, RewardTimerStore, StorageQueryable,
    },
};
use candid::Principal;
use canister_types::models::{api_error::ApiError, reward::RewardableActivityResponse};
use ic_cdk::{query, update};

#[update(guard = "is_developer")]
fn _dev_set_reward_canister(principal: Principal) -> Result<Principal, ApiError> {
    RewardCanisterStorage::set(principal)
}

#[query(guard = "is_developer")]
fn _dev_get_reward_canister() -> Result<Principal, ApiError> {
    RewardCanisterStorage::get()
}

#[query]
fn reward_timer_next_trigger() -> Option<u64> {
    RewardTimerStore::next_trigger()
}

#[query]
fn read_reward_buffer() -> Vec<RewardableActivityResponse> {
    RewardBufferStore::get_all()
        .into_iter()
        .map(|(_, v)| v.into())
        .collect()
}

// testers
#[update(guard = "is_developer")]
fn fill_buffer() {
    for (id, _) in GroupStore::get_all() {
        RewardBufferStore::notify_group_member_count_changed(id);
    }

    for (id, _) in ProfileStore::get_all() {
        RewardBufferStore::notify_active_user(id);
    }
}

#[update(guard = "is_developer")]
async fn _dev_send_reward_data() {
    send_reward_data().await
}
