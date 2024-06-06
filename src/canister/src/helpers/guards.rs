use candid::Principal;
use catalyze_shared::api_error::ApiError;
use ic_cdk::caller;

use canister_types::models::application_role::ApplicationRole;

use crate::storage::{ProfileStore, StorageQueryable};

/// Checks if the caller is an anonymous principal
/// # Returns
/// * `()` if the caller is not anonymous
/// # Errors
/// * `String` if the caller is anonymous
/// # Note
/// `Result<(), String>` type is required because of the usage as a guard in the `candid` attribute macro
pub fn is_not_anonymous() -> Result<(), String> {
    match caller() == Principal::anonymous() {
        true => Err(ApiError::unauthorized()
            .add_message("Anonymous principal")
            .to_string()),
        false => Ok(()),
    }
}

/// Checks if the caller is anonymous, has a profile and is not blocked or banned on the application level
/// # Returns
/// * `()` if the caller is not anonymous, has a profile and is not blocked or banned
/// # Errors
/// * `String` if the caller is anonymous, has no profile or is blocked or banned
/// # Note
/// `Result<(), String>` type is required because of the usage as a guard in the `candid` attribute macro
pub fn has_access() -> Result<(), String> {
    // Check if the caller is anonymous
    is_not_anonymous()?;

    // Get the caller's profile
    match ProfileStore::get(caller()) {
        Err(err) => Err(err.to_string()),
        Ok((_, profile)) => {
            // Check if the caller has a profile
            // Check if the caller is blocked or banned on the application level
            if [ApplicationRole::Blocked, ApplicationRole::Banned]
                .contains(&profile.application_role)
            {
                Err(ApiError::unauthorized()
                    .add_message("Blocked or banned")
                    .to_string())
            } else {
                Ok(())
            }
        }
    }
}

/// Checks if the caller is the monitor principal
pub fn is_monitor() -> Result<(), String> {
    // monitor principal
    let monitor_principal =
        Principal::from_text("6or45-oyaaa-aaaap-absua-cai").expect("Invalid principal");
    if caller() == monitor_principal {
        Ok(())
    } else {
        Err(ApiError::unauthorized()
            .add_message("Unauthorized")
            .to_string())
    }
}

pub fn is_prod_developer() -> Result<(), String> {
    let developers = [
        // production
        "ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe",
    ];

    if developers.contains(&caller().to_text().as_str()) {
        Ok(())
    } else {
        Err(ApiError::unauthorized()
            .add_message("Unauthorized")
            .to_string())
    }
}

// Check if the caller is the Catalyze developer principal
pub fn is_developer() -> Result<(), String> {
    let developers = [
        // production
        "ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe",
        // staging
        "syzio-xu6ca-burmx-4afo2-ojpcw-e75j3-m67o5-s5bes-5vvsv-du3t4-wae",
    ];

    if developers.contains(&caller().to_text().as_str()) {
        Ok(())
    } else {
        Err(ApiError::unauthorized()
            .add_message("Unauthorized")
            .to_string())
    }
}

// TODO: add guards for group role based access
// https://forum.dfinity.org/t/rust-guard-access-arguments/22229?u=rmcs
// https://docs.rs/ic-cdk/latest/ic_cdk/api/call/fn.arg_data.html
