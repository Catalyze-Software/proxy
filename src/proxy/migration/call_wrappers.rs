use candid::Principal;
use ic_cdk::trap;

use super::old_models::attendee_model::Attendee as OldAttendee;
use super::old_models::event_models::Event as OldEvent;
use super::old_models::group_model::Group as OldGroup;
use super::old_models::member_model::Member as OldMember;
use super::old_models::profile_models::Profile as OldProfile;

// child profiles
pub async fn read_profile_entries(env: String) -> Vec<(String, OldProfile)> {
    let canister_id = match env.as_str() {
        "production" => super::principals::PRODUCTION_CHILD_PROFILES,
        "staging" => super::principals::STAGING_CHILD_PROFILES,
        "development" => super::principals::DEVELOPMENT_CHILD_PROFILES,
        _ => trap("Invalid environment"),
    };
    let canister_id = Principal::from_text(canister_id).unwrap();

    ic_cdk::call::<(), (Vec<(String, OldProfile)>,)>(canister_id, "read_profile_entries", ())
        .await
        .expect("Failed to call profile canister")
        .0
}

// child members
pub async fn read_members_entries(env: String) -> Vec<(String, OldMember)> {
    let canister_id = match env.as_str() {
        "production" => super::principals::PRODUCTION_CHILD_MEMBERS,
        "staging" => super::principals::STAGING_CHILD_MEMBERS,
        "development" => super::principals::DEVELOPMENT_CHILD_MEMBERS,
        _ => trap("Invalid environment"),
    };
    let canister_id = Principal::from_text(canister_id).unwrap();

    ic_cdk::call::<(), (Vec<(String, OldMember)>,)>(canister_id, "read_members_entries", ())
        .await
        .expect("Failed to call member canister")
        .0
}

// child groups
pub async fn read_groups_entries(env: String) -> Vec<(String, OldGroup)> {
    let canister_id = match env.as_str() {
        "production" => super::principals::PRODUCTION_CHILD_GROUPS,
        "staging" => super::principals::STAGING_CHILD_GROUPS,
        "development" => super::principals::DEVELOPMENT_CHILD_GROUPS,
        _ => trap("Invalid environment"),
    };
    let canister_id = Principal::from_text(canister_id).unwrap();

    ic_cdk::call::<(), (Vec<(String, OldGroup)>,)>(canister_id, "read_groups_entries", ())
        .await
        .expect("Failed to call group canister")
        .0
}

// child events
pub async fn read_events_entries(env: String) -> Vec<(String, OldEvent)> {
    let canister_id = match env.as_str() {
        "production" => super::principals::PRODUCTION_CHILD_EVENTS,
        "staging" => super::principals::STAGING_CHILD_EVENTS,
        "development" => super::principals::DEVELOPMENT_CHILD_EVENTS,
        _ => trap("Invalid environment"),
    };
    let canister_id = Principal::from_text(canister_id).unwrap();

    ic_cdk::call::<(), (Vec<(String, OldEvent)>,)>(canister_id, "read_events_entries", ())
        .await
        .expect("Failed to call event canister")
        .0
}

// child attendees
pub async fn read_attendees_entries(env: String) -> Vec<(String, OldAttendee)> {
    let canister_id = match env.as_str() {
        "production" => super::principals::PRODUCTION_CHILD_EVENT_ATTENDEES,
        "staging" => super::principals::STAGING_CHILD_EVENT_ATTENDEES,
        "development" => super::principals::DEVELOPMENT_CHILD_EVENT_ATTENDEES,
        _ => trap("Invalid environment"),
    };
    let canister_id = Principal::from_text(canister_id).unwrap();

    ic_cdk::call::<(), (Vec<(String, OldAttendee)>,)>(canister_id, "read_attendees_entries", ())
        .await
        .expect("Failed to call attendee canister")
        .0
}

// notifications
// pub async fn read_notifications_entries(env: String) -> Vec<(u32, String)> {
//     let canister_id = match env.as_str() {
//         "production" => super::principals::PRODUCTION_NOTIFICATIONS,
//         "staging" => super::principals::STAGING_NOTIFICATIONS,
//         "development" => super::principals::DEVELOPMENT_NOTIFICATIONS,
//         _ => trap("Invalid environment"),
//     };
//     let canister_id = Principal::from_text(canister_id).unwrap();

//     ic_cdk::call::<(), (Vec<(u32, String)>,)>(canister_id, "read_notifications_entries", ())
//         .await
//         .expect("Failed to call notification canister")
//         .0
// }
