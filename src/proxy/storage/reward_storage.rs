use super::{
    storage_api::{StaticStorageRef, Storage, REWARD_BUFFER, REWARD_BUFFER_MEMORY_ID},
    StorageInsertable, StorageQueryable, StorageUpdateable, ID_KIND_REWARDS_BUFFER,
};
use crate::logic::reward_buffer_logic::send_reward_data;
use candid::Principal;
use canister_types::models::reward::{Activity, RewardableActivity};
use ic_cdk::{api::time, spawn};
use ic_cdk_timers::set_timer_interval;
use ic_stable_structures::memory_manager::MemoryId;
use std::{cell::RefCell, time::Duration};

// Interval for sending reward activities to Reward Canister
pub const DAY_IN_NANOS: u64 = 86400 * 1_000_000_000;

// timer to periodically process the reward buffer
thread_local! {
   pub static REWARD_TIMER: RefCell<Option<u64>> = const { RefCell::new(None) };
}

pub struct RewardTimerStore;

impl RewardTimerStore {
    pub fn start() {
        let _ = set_timer_interval(Duration::from_nanos(DAY_IN_NANOS), move || {
            spawn(send_reward_data());
        });

        let next_trigger = time() + DAY_IN_NANOS;

        REWARD_TIMER.with(|t| *t.borrow_mut() = Some(next_trigger));
    }

    pub fn next_trigger() -> Option<u64> {
        REWARD_TIMER.with(|t| *t.borrow())
    }

    pub fn set_next_trigger() {
        REWARD_TIMER.with(|t| {
            let next_trigger = time() + DAY_IN_NANOS;
            *t.borrow_mut() = Some(next_trigger);
        });
    }
}

pub struct RewardBufferStore;

impl Storage<u64, RewardableActivity> for RewardBufferStore {
    const NAME: &'static str = ID_KIND_REWARDS_BUFFER;

    fn storage() -> StaticStorageRef<u64, RewardableActivity> {
        &REWARD_BUFFER
    }

    fn memory_id() -> MemoryId {
        REWARD_BUFFER_MEMORY_ID
    }
}

impl StorageQueryable<u64, RewardableActivity> for RewardBufferStore {}
impl StorageUpdateable<u64, RewardableActivity> for RewardBufferStore {}
impl StorageInsertable<RewardableActivity> for RewardBufferStore {}

impl RewardBufferStore {
    pub fn notify_group_member_count_changed(group_id: u64) {
        let activity = RewardableActivity::new(Activity::GroupMemberCount(group_id));
        let _ = RewardBufferStore::insert(activity);
    }

    pub fn notify_referral_used(principal: Principal) {
        let activity = RewardableActivity::new(Activity::UserReferral(principal));
        let _ = RewardBufferStore::insert(activity);
    }

    pub fn notify_profile_filled(principal: Principal) {
        let activity = RewardableActivity::new(Activity::UserProfileFilled(principal));
        let _ = RewardBufferStore::insert(activity);
    }

    pub fn notify_first_group_joined(principal: Principal) {
        let activity: RewardableActivity =
            RewardableActivity::new(Activity::FirstGroupJoined(principal));
        let _ = RewardBufferStore::insert(activity);
    }
}
