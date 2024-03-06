use crate::{
    helpers::{
        group_permission::{can_delete, can_edit},
        guards::has_access,
    },
    logic::event_logic::EventCalls,
};
/// # Event methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Group) role based authentication

/// # Questions
/// * Should the `get_events` give back a more summier response per group compared to the `get_event` function?
/// And what about the public / private access of these calls?
///
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
    event::{EventFilter, EventResponse, EventSort, PostEvent, UpdateEvent},
    filter_type::FilterType,
    identifier::Identifier,
    paged_response::PagedResponse,
    permission::PermissionType,
};
use ic_cdk::{query, update};

/// Add an event - [`[update]`](update)
/// # Arguments
/// * `post_event` - The event to add
/// # Returns
/// * `EventResponse` - The added event
/// # Errors
/// * `ApiError` - If something went wrong while adding the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_event(post_event: PostEvent) -> Result<EventResponse, ApiError> {
    can_edit(post_event.group_id, PermissionType::Event(None))?;
    EventCalls::add_event(post_event)
}

/// Get an event - [`[query]`](query)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group
/// # Returns
/// * `EventResponse` - The event
/// # Errors
/// * `ApiError` - If something went wrong while getting the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query]
pub fn get_event(
    event_identifier: Principal,
    group_identifier: Principal,
) -> Result<EventResponse, ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    EventCalls::get_event(event_id, group_id)
}

/// Get paged events - [`[query]`](query)
/// # Arguments
/// * `limit` - The maximum number of events to return per page
/// * `page` - The page number
/// * `sort` - The sort direction
/// * `filters` - The filters to apply
/// # Returns
/// * `PagedResponse<EventResponse>` - The events in a paged response
/// # Errors
/// * `ApiError` - If something went wrong while getting the events
#[query]
fn get_events(
    limit: usize,
    page: usize,
    sort: EventSort,
    filters: Vec<FilterType<EventFilter>>,
) -> Result<PagedResponse<EventResponse>, ApiError> {
    EventCalls::get_events(limit, page, sort, filters)
}

/// Get the number of events per group - [`[query]`](query)
/// # Arguments
/// * `group_identifiers` - The group identifiers to get the events count from
/// # Returns
/// * `Vec<(Principal, usize)>` - The events count per group
#[query]
pub fn get_events_count(group_identifiers: Vec<Principal>) -> Vec<(Principal, u64)> {
    let _group_identifiers = group_identifiers
        .into_iter()
        .map(|identifier| Identifier::from(identifier).id())
        .collect();
    EventCalls::get_events_count(_group_identifiers)
}

/// edit an event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// * `update_event` - The event to update
/// * `group_identifier` - Used to check if the user has access to the group
/// # Returns
/// * `EventResponse` - The updated event
/// # Errors
/// * `ApiError` - If something went wrong while updating the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn edit_event(
    event_identifier: Principal,
    update_event: UpdateEvent,
    group_identifier: Principal,
) -> Result<EventResponse, ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::edit_event(event_id, update_event, group_id)
}

/// Delete an event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `()` - If the event was deleted
/// # Errors
/// * `ApiError` - If something went wrong while deleting the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn delete_event(
    event_identifier: Principal,
    group_identifier: Principal,
) -> Result<(), ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    can_delete(group_id, PermissionType::Event(None))?;
    EventCalls::delete_event(event_id, group_id)
}

/// Cancel an event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// * `reason` - The reason why the event was cancelled
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `()` - If the event was cancelled
/// # Errors
/// * `ApiError` - If something went wrong while cancelling the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn cancel_event(
    event_identifier: Principal,
    reason: String,
    group_identifier: Principal,
) -> Result<(), ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::cancel_event(event_id, reason, group_id)
}

// Attendee methods

/// Join an event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `JoinedAttendeeResponse` - the event join details
/// # Errors
/// * `ApiError` - If something went wrong while joining the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn join_event(
    event_identifier: Principal,
    group_identifier: Principal,
) -> Result<JoinedAttendeeResponse, ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    EventCalls::join_event(event_id, group_id)
}

/// Invite a user to an event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// * `attendee_principal` - The principal of the user to invite
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `InviteAttendeeResponse` - The event invite details
/// # Errors
/// * `ApiError` - If something went wrong while inviting the user to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn invite_to_event(
    event_identifier: Principal,
    attendee_principal: Principal,
    group_identifier: Principal,
) -> Result<InviteAttendeeResponse, ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::invite_to_event(event_id, attendee_principal, group_id)
}

/// Accept an user invite to an event as a admin - [`[update]`](update)
/// # Arguments
/// * `attendee_principal` - The principal of the user to accept
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `JoinedAttendeeResponse` - the event join details
/// # Errors
/// * `ApiError` - If something went wrong while accepting the user invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn accept_user_request_event_invite(
    attendee_principal: Principal,
    event_identifier: Principal,
    group_identifier: Principal,
) -> Result<JoinedAttendeeResponse, ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::accept_user_request_event_invite(event_id, attendee_principal, group_id)
}

/// Accept an owner invite to an event as a user - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// # Returns
/// * `Attendee` - The attendee
/// # Errors
/// * `ApiError` - If something went wrong while accepting the owner invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn accept_owner_request_event_invite(
    event_identifier: Principal,
) -> Result<Attendee, ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    EventCalls::accept_owner_request_event_invite(event_id)
}

/// Get the attendees for an event - [`[query]`](query)
/// # Arguments
/// * `event_identifier` - The event identifier to get the attendees from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The attendees for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendees for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_event_attendees(
    event_identifier: Principal,
) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    EventCalls::get_event_attendees(event_id)
}

/// Get the caller attendee entry - [`[query]`](query)
/// # Change
/// * was `get_self` but due to conflict with other methods it was renamed
/// # Returns
/// * `Attendee` - The attendee
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendee
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_self_events() -> Result<Attendee, ApiError> {
    EventCalls::get_self_events()
}

/// Get the joined events from a principal - [`[query]`](query)
/// # Arguments
/// * `principal` - The principal to get the joined events from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The joined events
/// # Errors
/// * `ApiError` - If something went wrong while getting the joined events
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_attending_from_principal(
    principal: Principal,
) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
    EventCalls::get_attending_from_principal(principal)
}

/// Leave an event - [`[update]`](update)
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// # Returns
/// * `()` - If the user left the event
/// # Errors
/// * `ApiError` - If something went wrong while leaving the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn leave_event(event_identifier: Principal) -> Result<(), ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    EventCalls::leave_event(event_id)
}

/// Remove an event invite as a user - [`[update]`](update)
/// # Change
/// * was `remove_event`
/// # Arguments
/// * `event_identifier` - The identifier of the event
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_event_invite(event_identifier: Principal) -> Result<(), ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    EventCalls::remove_event_invite(event_id)
}

/// Remove an event attendee as a admin - [`[update]`](update)
/// # Arguments
/// * `attendee_principal` - The principal of the user to remove
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_attendee_from_event(
    attendee_principal: Principal,
    event_identifier: Principal,
    group_identifier: Principal,
) -> Result<(), ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::remove_attendee_from_event(attendee_principal, event_id, group_id)
}

/// Remove an event invite as a admin - [`[update]`](update)
/// # Arguments
/// * `attendee_principal` - The principal of the user to remove
/// * `event_identifier` - The identifier of the event
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_attendee_invite_from_event(
    attendee_principal: Principal,
    event_identifier: Principal,
    group_identifier: Principal,
) -> Result<(), ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::remove_attendee_invite_from_event(attendee_principal, event_id)
}

/// Get the invites for an event - [`[query]`](query)
/// # Arguments
/// * `event_identifier` - The event identifier to get the invites from
/// * `group_identifier` - Used to check if the user has access to the group the event belongs to
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `Vec<InviteAttendeeResponse>` - The invites for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the invites for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[query(guard = "has_access")]
pub fn get_event_invites(
    event_identifier: Principal,
    group_identifier: Principal,
) -> Result<Vec<InviteAttendeeResponse>, ApiError> {
    let event_id = Identifier::from(event_identifier).id();
    let group_id = Identifier::from(group_identifier).id();
    EventCalls::get_event_invites(event_id, group_id)
}
