use crate::{helpers::guards::has_access, logic::boost_logic::BoostCalls, E8S_PER_DAY_BOOST_COST};
use canister_types::models::{api_error::ApiError, boosted::Boost, subject::Subject};
use ic_cdk::{query, update};

/// Returns the boosted groups
/// # Returns
/// * `Vec<(u64, Boost)>` - (boost id, boosted groups)
#[query]
fn get_boosted_groups() -> Vec<(u64, Boost)> {
    BoostCalls::get_boosts_by_subject(Subject::Group(0))
}

/// Returns the boosted events
/// # Returns
/// * `Vec<(u64, Boost)>` - (boost id, boosted groups)
#[query]
fn get_boosted_events() -> Vec<(u64, Boost)> {
    BoostCalls::get_boosts_by_subject(Subject::Event(0))
}

/// Returns the cost of boosting per day
/// # Returns
/// * `u64` - the cost of boosting per day
#[query]
fn get_e8s_per_day_boost_cost() -> u64 {
    E8S_PER_DAY_BOOST_COST
}

/// Boosts a group or event
/// # Arguments
/// * `identifier` - the identifier of the group or event
/// * `blockheight` - the blockheight of the ICP transaction
/// # Returns
/// * `u64` - the remaining boost time in seconds
/// # Errors
/// * `ApiError` if something went wrong boosting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// The identifier is used to determine if the group or event should be boosted.
#[update(guard = "has_access")]
async fn boost(boost_subject: Subject, blockheight: u64) -> Result<u64, ApiError> {
    use Subject::*;
    let subject = match boost_subject {
        Group(id) => Subject::Group(id),
        Event(id) => Subject::Event(id),
        _ => return Err(ApiError::bad_request().add_message("Invalid identifier")),
    };

    BoostCalls::boost(subject, blockheight).await
}

/// Returns the remaining boost time in seconds for a group or event
/// # Arguments
/// * `identifier` - the identifier of the group or event
/// # Returns
/// * `u64` - the remaining boost time in seconds
/// # Errors
/// * `ApiError` if something went wrong getting the remaining boost time
#[query]
fn get_remaining_boost_time_in_seconds(boost_subject: Subject) -> Result<u64, ApiError> {
    use Subject::*;
    let subject = match boost_subject {
        Group(id) => Subject::Group(id),
        Event(id) => Subject::Event(id),
        _ => return Err(ApiError::bad_request().add_message("Invalid identifier")),
    };

    let (id, _) = BoostCalls::get_boost_by_subject(subject)?;
    BoostCalls::get_seconds_left_for_boost(id)
}
