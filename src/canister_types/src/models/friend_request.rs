use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::time;
use serde::Deserialize;

use crate::impl_storable_for;

impl_storable_for!(FriendRequest);

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FriendRequest {
    pub requested_by: Principal,
    pub message: String,
    pub to: Principal,
    pub created_at: u64,
}

impl FriendRequest {
    pub fn new(requested_by: Principal, to: Principal, message: String) -> Self {
        Self {
            requested_by,
            message,
            to,
            created_at: time(),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FriendRequestResponse {
    pub id: u64,
    pub requested_by: Principal,
    pub message: String,
    pub to: Principal,
    pub created_at: u64,
}