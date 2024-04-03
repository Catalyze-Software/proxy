#![allow(unused)]
use crate::{ENV, SENDER};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
    event::{EventFilter, EventResponse, EventSort, PostEvent, UpdateEvent},
    filter_type::FilterType,
    paged_response::PagedResponse,
};
use pocket_ic::{query_candid_as, update_candid_as};

pub fn add_event(
    value: PostEvent,
    group_identifier: Principal,
    member_identifier: Principal,
    event_attendee_canister: Principal,
) -> EventResponse {
    let event_response: EventResponse = update_candid_as::<
        (PostEvent, Principal, Principal, Principal),
        (Result<EventResponse, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "add_event",
        (
            value,
            group_identifier,
            member_identifier,
            event_attendee_canister,
        ),
    )
    .expect("Failed to call add_event from pocket ic")
    .0
    .expect("Failed to call add_event");

    event_response
}

pub fn get_event(event_id: u64, group_id: u64) -> Result<EventResponse, ApiError> {
    query_candid_as::<(u64, u64), (Result<EventResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_event",
        (event_id, group_id),
    )
    .expect("Failed to call get_event from pocket ic")
    .0
}

// deprecated
// pub fn get_event_privacy_and_owner(
//     identifier: Principal,
//     group_identifier: Principal,
// ) -> Result<(Principal, Privacy), ApiError>

pub fn get_events(
    limit: usize,
    page: usize,
    sort: EventSort,
    filter: Vec<EventFilter>,
    filter_type: Vec<FilterType<EventFilter>>,
    group_identifier: Option<Principal>,
) -> PagedResponse<EventResponse> {
    let paged_response: PagedResponse<EventResponse> = query_candid_as::<
        (
            usize,
            usize,
            EventSort,
            Vec<EventFilter>,
            Vec<FilterType<EventFilter>>,
            Option<Principal>,
        ),
        (Result<PagedResponse<EventResponse>, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_events",
        (limit, page, sort, filter, filter_type, group_identifier),
    )
    .expect("Failed to call get_events from pocket ic")
    .0
    .expect("Failed to call get_events");

    paged_response
}

pub fn get_events_count(group_ids: Vec<u64>) -> Vec<(Principal, u64)> {
    query_candid_as::<(Vec<u64>,), (Vec<(Principal, u64)>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_events_count",
        (group_ids,),
    )
    .expect("Failed to call get_events_count from pocket ic")
    .0
}

pub fn edit_event(
    event_id: u64,
    group_id: u64,
    update_event: UpdateEvent,
) -> Result<EventResponse, ApiError> {
    update_candid_as::<(u64, u64, UpdateEvent), (Result<EventResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "edit_event",
        (event_id, group_id, update_event),
    )
    .expect("Failed to call edit_event from pocket ic")
    .0
}

pub fn delete_event(event_id: u64, group_id: u64) -> Result<(), ApiError> {
    update_candid_as::<(u64, u64), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "delete_event",
        (event_id, group_id),
    )
    .expect("Failed to call delete_event from pocket ic")
    .0
}

pub fn cancel_event(event_id: u64, group_id: u64, reason: String) -> Result<(), ApiError> {
    update_candid_as::<(u64, u64), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "cancel_event",
        (event_id, group_id),
    )
    .expect("Failed to call cancel_event from pocket ic")
    .0
}

// deprecated
// pub fn update_attendee_count_on_event(
//     event_identifier: Principal,
//     event_attendee_canister: Principal,
//     attendee_count: usize,
// ) -> Result<(), bool>

pub fn join_event(event_id: u64, group_id: u64) -> Result<JoinedAttendeeResponse, ApiError> {
    update_candid_as::<(u64, u64), (Result<JoinedAttendeeResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "join_event",
        (event_id, group_id),
    )
    .expect("Failed to call join_event from pocket ic")
    .0
}

pub fn invite_to_event(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> Result<InviteAttendeeResponse, ApiError> {
    update_candid_as::<(u64, u64, Principal), (Result<InviteAttendeeResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "invite_to_event",
        (event_id, group_id, attendee_principal),
    )
    .expect("Failed to call invite_to_event from pocket ic")
    .0
}

pub fn accept_user_request_event_invite(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> Result<JoinedAttendeeResponse, ApiError> {
    update_candid_as::<(u64, u64, Principal), (Result<JoinedAttendeeResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "accept_user_request_event_invite",
        (event_id, group_id, attendee_principal),
    )
    .expect("Failed to call accept_user_request_event_invite from pocket ic")
    .0
}

pub fn accept_owner_request_event_invite(event_id: u64) -> Result<Attendee, ApiError> {
    update_candid_as::<(u64,), (Result<Attendee, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "accept_owner_request_event_invite",
        (event_id,),
    )
    .expect("Failed to call accept_owner_request_event_invite from pocket ic")
    .0
}

// deprecated
// pub fn get_event_attendees_count(event_identifiers: Vec<Principal>) -> Vec<(Principal, usize)>

// deprecated
// pub fn get_event_invites_count(event_identifiers: Vec<Principal>) -> Vec<(Principal, usize)>

pub fn get_event_attendees(event_id: u64) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
    query_candid_as::<(u64,), (Result<Vec<JoinedAttendeeResponse>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_event_attendees",
        (event_id,),
    )
    .expect("Failed to call get_event_attendees from pocket ic")
    .0
}

pub fn get_self_events() -> (Principal, Attendee) {
    query_candid_as::<(), (Result<(Principal, Attendee), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_self_events",
        (),
    )
    .expect("Failed to call get_self_events from pocket ic")
    .0
    .expect("Failed to call get_self_events")
}

pub fn get_attending_from_principal(principal: Principal) -> Vec<JoinedAttendeeResponse> {
    query_candid_as::<(Principal,), (Result<Vec<JoinedAttendeeResponse>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_attending_from_principal",
        (principal,),
    )
    .expect("Failed to call get_attending_from_principal from pocket ic")
    .0
    .expect("Failed to call get_attending_from_principal")
}

pub fn leave_event(event_id: u64) -> Result<(), ApiError> {
    update_candid_as::<(u64,), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "leave_event",
        (event_id,),
    )
    .expect("Failed to call leave_event from pocket ic")
    .0
}

pub fn remove_event_invite(event_id: u64) -> Result<(), ApiError> {
    update_candid_as::<(u64,), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_event_invite",
        (event_id,),
    )
    .expect("Failed to call remove_event_invite from pocket ic")
    .0
}

pub fn remove_attendee_from_event(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> Result<(), ApiError> {
    update_candid_as::<(u64, u64, Principal), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_attendee_from_event",
        (event_id, group_id, attendee_principal),
    )
    .expect("Failed to call remove_attendee_from_event from pocket ic")
    .0
}

pub fn remove_attendee_invite_from_event(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> Result<(), ApiError> {
    update_candid_as::<(u64, u64, Principal), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_attendee_invite_from_event",
        (event_id, group_id, attendee_principal),
    )
    .expect("Failed to call remove_attendee_invite_from_event from pocket ic")
    .0
}

pub fn get_event_invites(
    event_id: u64,
    group_id: u64,
) -> Result<Vec<InviteAttendeeResponse>, ApiError> {
    query_candid_as::<(u64, u64), (Result<Vec<InviteAttendeeResponse>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_event_invites",
        (event_id, group_id),
    )
    .expect("Failed to call get_event_invites from pocket ic")
    .0
}

// deprecated
// pub fn add_owner_as_attendee(
//     user_principal: Principal,
//     event_identifier: Principal,
//     group_identifier: Principal,
// ) -> Result<(), bool>
