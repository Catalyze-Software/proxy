use candid::Principal;
use canister_types::models::api_error::ApiError;
use ic_cdk::{query, update};

use crate::{
    helpers::guards::is_developer,
    storage::{CellStorage, HistoryCanisterStorage, HistoryPointStorage},
};

#[query]
fn get_history_point() -> Result<u64, ApiError> {
    HistoryPointStorage::get()
}

#[update(guard = "is_developer")]
fn _dev_set_history_canister(principal: Principal) -> Result<Principal, ApiError> {
    HistoryCanisterStorage::set(principal)
}

#[query(guard = "is_developer")]
fn _dev_get_history_canister() -> Result<Principal, ApiError> {
    HistoryCanisterStorage::get()
}
