use crate::storage::{
    reward_canister_storage::RewardCanisterStorage, CellStorage, GroupMemberStore, GroupStore,
    RewardBufferStore, RewardTimerStore, StorageQueryable, StorageUpdateable,
};

use candid::Principal;
use canister_types::models::reward::{
    Activity, GroupReward, RewardDataPackage, RewardableActivity,
};
use ic_cdk::call;

pub fn process_buffer() -> RewardDataPackage {
    let rewardables: Vec<RewardableActivity> = RewardBufferStore::get_all()
        .into_iter()
        .map(|(_, activity)| activity)
        .collect();

    let mut group_member_counts: Vec<GroupReward> = Vec::new();
    let mut user_referrals: Vec<Principal> = Vec::new();
    let mut filled_profiles: Vec<Principal> = Vec::new();
    let mut first_group_joined: Vec<Principal> = Vec::new();

    for rewardable in rewardables.iter() {
        match rewardable.get_activity() {
            Activity::GroupMemberCount(id) => {
                // collect owner, group id and member count
                if let Ok((_, group)) = GroupStore::get(id) {
                    let (_, members) = GroupMemberStore::get(id).unwrap_or_default();

                    group_member_counts.push(GroupReward::new(
                        group.owner,
                        id,
                        members.get_member_count(),
                    ));
                }
            }
            Activity::UserReferral(principal) => user_referrals.push(principal),
            Activity::UserProfileFilled(principal) => filled_profiles.push(principal),
            Activity::FirstGroupJoined(principal) => first_group_joined.push(principal),
            Activity::None => {}
        }
    }

    RewardDataPackage {
        group_member_counts,
        user_referrals,
        filled_profiles,
        first_group_joined,
    }
}

pub async fn send_reward_data() {
    RewardTimerStore::set_next_trigger();

    let reward_canister = RewardCanisterStorage::get().expect("Reward canister not set");

    let reward_data = process_buffer();

    let _ =
        call::<(RewardDataPackage,), ()>(reward_canister, "process_buffer", (reward_data,)).await;

    // clear buffer
    RewardBufferStore::clear();
}
