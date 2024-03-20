use candid::Principal;
use canister_types::models::api_error::ApiError;
use canister_types::models::profile::{PostProfile, ProfileResponse};
use canister_types::models::profile_privacy::ProfilePrivacy;
use futures::future::join_all;
use ic_agent::identity::Secp256k1Identity;
use ic_agent::Agent;
use ic_utils::call::SyncCall;
use ic_utils::Canister;

const PROXY_CANISTER_ID: &str = "bwm3m-wyaaa-aaaag-qdiua-cai";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let identity = Secp256k1Identity::from_pem_file("../test/src/mocks/identity2.pem").unwrap();

    let agent = Agent::builder()
        .with_url("https://icp0.io")
        .with_identity(identity)
        .build()
        .expect("failed to create agent");

    // let mock_profile = mock_post_profile();

    let canister = Canister::builder()
        .with_agent(&agent)
        .with_canister_id(Principal::from_text(PROXY_CANISTER_ID).unwrap())
        .build()
        .expect("failed to create canister");

    // ADD PROFILE and save principal
    // let new_principal = canister
    //     .update("add_profile")
    //     .with_arg::<PostProfile>(mock_profile)
    //     .build::<(Result<ProfileResponse, ApiError>,)>()
    //     .call_and_wait()
    //     .await
    //     .expect("failed to call canister")
    //     .0
    //     .unwrap_or_else(|err| panic!("failed to add profile: {:?}", err))
    //     .principal;

    // std::fs::write("principal_for_identity2.txt", &new_principal.to_text())
    //     .expect("failed to write principal");

    // GET PROFILE
    let new_principal = Principal::from_text(
        std::fs::read_to_string("principal_for_identity2.txt")
            .expect("failed to read principal")
            .trim(),
    )
    .expect("failed to parse principal");

    // let profile_response = canister
    //     .query("get_profile")
    //     .with_arg::<Principal>(new_principal)
    //     .build::<(Result<ProfileResponse, ApiError>,)>()
    //     .call()
    //     .await
    //     .expect("failed to call canister")
    //     .0
    //     .unwrap_or_else(|err| panic!("failed to get profile: {:?}", err));

    // let size = std::mem::size_of_val(&profile_response);
    // println!("{:?}", size);

    loop {
        let now = std::time::Instant::now();
        let futures = (0..10)
            .map(|_| {
                canister
                    .query("get_profile")
                    .with_arg::<Principal>(new_principal)
                    .build::<(Result<ProfileResponse, ApiError>,)>()
                    .call()
            })
            .collect::<Vec<_>>();

        let results = join_all(futures.into_iter()).await;

        let elapsed = now.elapsed();

        let mut total_size = 0;
        for result in results {
            total_size += std::mem::size_of_val(&result);
        }

        println!("Elapsed: {:?}, Total size: {}", elapsed, total_size);
    }
}

fn mock_post_profile() -> PostProfile {
    PostProfile {
        username: "test_username".to_string(),
        display_name: "test_display_name".to_string(),
        first_name: "test_first_name".to_string(),
        last_name: "test_last_name".to_string(),
        privacy: ProfilePrivacy::Public,
        extra: "test_extra".to_string(),
    }
}
