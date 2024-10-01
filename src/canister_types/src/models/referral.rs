use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(Referral);

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Referral {
    pub referred_by: Principal,
    pub created_at: u64,
}

impl Default for Referral {
    fn default() -> Self {
        Self {
            created_at: time(),
            referred_by: Principal::anonymous(),
        }
    }
}

impl Referral {
    pub fn new(referred_by: Principal) -> Self {
        Self {
            referred_by,
            created_at: time(),
        }
    }
}
