use canister_types::models::{profile::PostProfile, profile_privacy::ProfilePrivacy};

use crate::{
    calls,
    utils::{random_principal, setup, FallibleCall},
};

#[test]
fn test_add_profile() {
    let ctx = setup();
    let sender = random_principal();

    let input = (
        PostProfile {
            username: "alice".to_string(),
            display_name: "Alice".to_string(),
            first_name: "Alice".to_string(),
            last_name: "Smith".to_string(),
            privacy: ProfilePrivacy::default(),
            extra: Default::default(),
        },
        sender,
    );

    let resp = calls::add_profile(&ctx, sender, input).assert_success();
    assert_eq!(resp.username, "alice");
}
