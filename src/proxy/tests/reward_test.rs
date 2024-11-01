use std::time::Duration;

use candid::{encode_args, Principal};
use canister_types::models::{
    api_error::ApiError, asset::Asset, group::PostGroup, location::Location,
    member::JoinedMemberResponse, privacy::Privacy, profile::PostProfile,
    profile_privacy::ProfilePrivacy,
};
use test_helper::{context::Context, sender::Sender, utils::generate_principal};

#[test]
pub fn test_referrals() {
    let context = Context::new();

    // check if the reward canister is set correctly
    let reward_canister_on_proxy = context
        .proxy_query::<Result<Principal, ApiError>>(
            Sender::ProductionDeveloper,
            "_dev_get_reward_canister",
            None,
        )
        .expect("Failed to get reward canister");

    assert!(reward_canister_on_proxy.is_ok());
    assert!(reward_canister_on_proxy.unwrap() == context.reward_canister_id);

    let proxy_canister_on_reward_canister = context
        .reward_query::<Result<Principal, ApiError>>(
            Sender::ProductionDeveloper,
            "_dev_get_proxy",
            None,
        )
        .expect("Failed to get reward canister");

    assert!(proxy_canister_on_reward_canister.is_ok());
    assert!(proxy_canister_on_reward_canister.unwrap() == context.proxy_canister_id);

    for i in 0..10 {
        let new_principal = generate_principal();
        // create a new profile with referral and approve all documents
        let new_profile = context.create_profile_with_referral(
            new_principal,
            PostProfile {
                username: format!("test_{}", i),
                display_name: format!("test_{}", i),
                first_name: format!("test_{}", i),
                last_name: format!("test_{}", i),
                privacy: ProfilePrivacy::Public,
                extra: format!("test_{}", i),
            },
        );

        assert!(new_profile.principal == new_principal);

        // check if the referred_by principal is set correctly
        let referred_by = context.get_referred_by(new_principal).unwrap();

        assert!(referred_by == context.ref_principal);
    }

    // get the reward buffer from the proxy
    let reward_buffer = context.get_reward_buffer();

    assert!(!reward_buffer.is_empty());

    context.pic.advance_time(Duration::from_secs(86500)); // 1 day
    context.pic.tick();

    // get the reward buffer from the proxy
    let reward_buffer = context.get_reward_buffer();

    assert!(reward_buffer.is_empty());
    println!("new: {:?}", reward_buffer);

    let reward_result = context
        .reward_query::<Result<(Principal, u64), ApiError>>(
            Sender::ProductionDeveloper,
            "get_user_points",
            Some(encode_args((context.ref_principal,)).unwrap()),
        )
        .expect("Failed to get user points");

    assert!(reward_result.is_ok());
    assert!(reward_result.clone().unwrap().1 == 1000); // (10 x 100 points per ref) = 1000 points

    for i in 0..10 {
        let new_principal = generate_principal();
        // create a new profile with referral and approve all documents
        context.create_profile_with_referral(
            new_principal,
            PostProfile {
                username: format!("second_test_{}", i),
                display_name: format!("second_test_{}", i),
                first_name: format!("second_test_{}", i),
                last_name: format!("second_test_{}", i),
                privacy: ProfilePrivacy::Public,
                extra: format!("second_test_{}", i),
            },
        );
        context.pic.advance_time(Duration::from_secs(86500)); // 1 day between each profile creation
        context.pic.tick();
    }

    let reward_result = context
        .reward_query::<Result<(Principal, u64), ApiError>>(
            Sender::ProductionDeveloper,
            "get_user_points",
            Some(encode_args((context.ref_principal,)).unwrap()),
        )
        .expect("Failed to get user points");

    assert!(reward_result.is_ok());
    assert!(reward_result.clone().unwrap().1 == 2000); // 1000 + (10 x 100 points per ref) = 1000 points
}

#[test]
pub fn test_first_group_join() {
    let context = Context::new();

    context.create_default_profile_for_user();

    let group_response = context.create_group(
        context.user_principal,
        PostGroup {
            name: "test_group".to_string(),
            description: "test_group".to_string(),
            privacy: Privacy::Public,
            website: "test_group".to_string(),
            matrix_space_id: "test_group".to_string(),
            location: Location::None,
            privacy_gated_type_amount: None,
            image: Asset::None,
            banner_image: Asset::None,
            tags: vec![],
        },
    );

    let random_user = generate_principal();
    let _ = context.create_profile(
        random_user,
        PostProfile {
            username: "random_user".to_string(),
            display_name: "random_user".to_string(),
            first_name: "random_user".to_string(),
            last_name: "random_user".to_string(),
            privacy: ProfilePrivacy::Public,
            extra: "random_user".to_string(),
        },
    );

    let joined_group = context
        .proxy_update::<Result<JoinedMemberResponse, ApiError>>(
            Sender::Other(random_user),
            "join_group",
            Some(encode_args((group_response.id,)).unwrap()),
        )
        .expect("Failed to join group");

    assert!(joined_group.is_ok());

    let reward_buffer = context.get_reward_buffer();

    assert!(!reward_buffer.is_empty());

    context.pic.advance_time(Duration::from_secs(86500)); // 1 day between each profile creation
    context.pic.tick();

    let reward_buffer = context.get_reward_buffer();

    assert!(reward_buffer.is_empty());

    let reward_result = context
        .reward_query::<Result<(Principal, u64), ApiError>>(
            Sender::ProductionDeveloper,
            "get_user_points",
            Some(encode_args((random_user,)).unwrap()),
        )
        .expect("Failed to get user points");

    assert!(reward_result.is_ok());
    assert!(reward_result.clone().unwrap().1 == 200);

    let _ = context
        .proxy_update::<Result<(), ApiError>>(
            Sender::Other(random_user),
            "leave_group",
            Some(encode_args((group_response.id,)).unwrap()),
        )
        .expect("Failed to leave group");

    let _ = context
        .proxy_update::<Result<JoinedMemberResponse, ApiError>>(
            Sender::Other(random_user),
            "join_group",
            Some(encode_args((group_response.id,)).unwrap()),
        )
        .expect("Failed to leave group");

    let reward_result = context
        .reward_query::<Result<(Principal, u64), ApiError>>(
            Sender::ProductionDeveloper,
            "get_user_points",
            Some(encode_args((random_user,)).unwrap()),
        )
        .expect("Failed to get user points");

    assert!(reward_result.is_ok());
    assert!(reward_result.clone().unwrap().1 == 200);
}
