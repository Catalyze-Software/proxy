use std::fmt;

use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(RewardableActivity);

#[derive(Clone, Default, Debug, CandidType, Deserialize, Serialize)]
pub struct RewardableActivity {
    activity: Vec<u8>,
    timestamp: u64,
}

impl RewardableActivity {
    pub fn new(activity: Activity) -> Self {
        Self {
            activity: activity.encode(),
            timestamp: time(),
        }
    }

    pub fn get_activity(&self) -> Activity {
        Activity::decode(self.activity.clone())
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn before(&self, days: u64) -> bool {
        self.timestamp < time() - days * 24 * 60 * 60
    }
}

impl From<RewardableActivity> for RewardableActivityResponse {
    fn from(val: RewardableActivity) -> Self {
        RewardableActivityResponse {
            activity: val.get_activity(),
            timestamp: val.timestamp,
        }
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct RewardableActivityResponse {
    pub activity: Activity,
    pub timestamp: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum Activity {
    GroupMemberCount(u64),
    UserActivity(Principal),
    UserReferral(Principal),
    UserProfileFilled(Principal),
    FirstGroupJoined(Principal),
}

impl Activity {
    pub fn encode(&self) -> Vec<u8> {
        Encode!(self).unwrap()
    }

    pub fn decode(bytes: Vec<u8>) -> Self {
        Decode!(&bytes, Activity).unwrap()
    }
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Activity::GroupMemberCount(_) => write!(f, "group_member_count"),
            Activity::UserActivity(_) => write!(f, "user_activity"),
            Activity::UserReferral(_) => write!(f, "user_referral"),
            Activity::UserProfileFilled(_) => write!(f, "user_profile_filled"),
            Activity::FirstGroupJoined(_) => write!(f, "first_group_joined"),
        }
    }
}

#[derive(Deserialize, CandidType, Clone)]
pub struct GroupReward {
    pub owner: Principal,
    pub group_id: u64,
    pub group_member_count: u64,
}

impl GroupReward {
    pub fn new(owner: Principal, id: u64, count: u64) -> Self {
        Self {
            owner,
            group_id: id,
            group_member_count: count,
        }
    }
}

#[derive(Deserialize, CandidType, Clone)]
pub struct UserActivity {
    pub owner: Principal,
    pub timestamp: u64,
}

impl UserActivity {
    pub fn new(owner: Principal, timestamp: u64) -> Self {
        Self { owner, timestamp }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct RewardDataPackage {
    pub group_member_counts: Vec<GroupReward>,
    pub user_activity: Vec<UserActivity>,
    pub user_referrals: Vec<Principal>,
    pub filled_profiles: Vec<Principal>,
    pub first_group_joined: Vec<Principal>,
}
