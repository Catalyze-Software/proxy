use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(GroupTransferRequest);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupTransferRequest {
    pub from: Principal,
    pub to: Principal,
    pub created_on: u64,
}

impl GroupTransferRequest {
    pub fn new(from: Principal, to: Principal) -> Self {
        Self {
            from,
            to,
            created_on: time(),
        }
    }
}
