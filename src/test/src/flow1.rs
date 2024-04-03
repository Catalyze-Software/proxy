// Flow 1

use crate::{
    calls::{
        group_calls::add_group,
        profile_calls::{
            add_profile, approve_code_of_conduct, approve_privacy_policy, approve_terms_of_service,
        },
    },
    mocks::{
        models::{mock_post_group, mock_post_profile},
        principals::{canister_test_id, member_test_id},
    },
    GROUP_ID, SENDER,
};
use candid::Principal;
use canister_types::models::{
    group::{GroupResponse, PostGroup},
    profile::{PostProfile, ProfileResponse},
};

pub fn flow1() {
    // Set sender principal
    SENDER.with(|s| *s.borrow_mut() = Some(member_test_id()));

    /*
     * Add profile
     */
    let post_profile: PostProfile = mock_post_profile();

    let profile_response: ProfileResponse = add_profile(post_profile);

    // The `principal` field of the response should be the same as the sender.
    assert_eq!(
        profile_response.principal,
        SENDER.with(|s| s.borrow().unwrap())
    );

    /*
     * Approve code of conduct
     */
    let version: u64 = 1;

    let code_of_conduct_approved: bool = approve_code_of_conduct(version);

    assert_eq!(code_of_conduct_approved, true);

    /*
     * Approve privacy policy
     */
    let version: u64 = 1;

    let privacy_policy_approved: bool = approve_privacy_policy(version);

    assert_eq!(privacy_policy_approved, true);

    /*
     * Approve terms of service
     */
    let version: u64 = 1;

    let terms_of_service_approved: bool = approve_terms_of_service(version);

    assert_eq!(terms_of_service_approved, true);

    /*
     * Add group
     */
    let post_group: PostGroup = mock_post_group();

    // TODO:
    let group_response: GroupResponse = add_group(post_group);
    GROUP_ID.with(|g| *g.borrow_mut() = Some(group_response.id));
}
