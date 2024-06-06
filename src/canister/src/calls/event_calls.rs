/// # Event methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Group) role based authentication

/// # Questions
/// * Should the `get_events` give back a more summier response per group compared to the `get_event` function?
/// And what about the public / private access of these calls?
///
use candid::Principal;
use catalyze_shared::api_error::ApiError;
use ic_cdk::{query, update};

use canister_types::models::{
    attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
    event::{EventFilter, EventResponse, EventsCount, EventSort, PostEvent, UpdateEvent},
    paged_response::PagedResponse,
    permission::PermissionType,
    profile::ProfileResponse,
};

use crate::{
    helpers::{
        group_permission::{can_delete, can_edit},
        guards::has_access,
    },
    logic::event_logic::EventCalls,
};

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
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group
/// # Returns
/// * `EventResponse` - The event
/// # Errors
/// * `ApiError` - If something went wrong while getting the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query]
pub fn get_event(event_id: u64) -> Result<EventResponse, ApiError> {
    EventCalls::get_event(event_id)
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
    filters: Vec<EventFilter>,
) -> Result<PagedResponse<EventResponse>, ApiError> {
    EventCalls::get_events(limit, page, sort, filters)
}

/// Get events count - [`[query]`](query)
/// # Arguments
/// * `group_ids` - Optional group ids to filter the events count
/// * `query` - Optional query to filter the events
/// # Returns
/// * `EventsCount` - The events in a paged response
#[query]
fn get_event_count(group_ids: Option<Vec<u64>>, query: Option<String>) -> EventsCount {
    EventCalls::get_events_count(group_ids, query)
}

/// edit an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group
/// * `update_event` - The event to update
/// # Returns
/// * `EventResponse` - The updated event
/// # Errors
/// * `ApiError` - If something went wrong while updating the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn edit_event(
    event_id: u64,
    group_id: u64,
    update_event: UpdateEvent,
) -> Result<EventResponse, ApiError> {
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::edit_event(event_id, update_event, group_id)
}

/// Delete an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `()` - If the event was deleted
/// # Errors
/// * `ApiError` - If something went wrong while deleting the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn delete_event(event_id: u64, group_id: u64) -> Result<(), ApiError> {
    can_delete(group_id, PermissionType::Event(None))?;
    EventCalls::delete_event(event_id, group_id)
}

/// Cancel an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `reason` - The reason why the event was cancelled
/// # Returns
/// * `()` - If the event was cancelled
/// # Errors
/// * `ApiError` - If something went wrong while cancelling the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn cancel_event(event_id: u64, group_id: u64, reason: String) -> Result<(), ApiError> {
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::cancel_event(event_id, reason, group_id)
}

// Attendee methods

/// Join an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `JoinedAttendeeResponse` - the event join details
/// # Errors
/// * `ApiError` - If something went wrong while joining the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn join_event(event_id: u64) -> Result<JoinedAttendeeResponse, ApiError> {
    EventCalls::join_event(event_id)
}

/// Invite a user to an event - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to invite
/// # Returns
/// * `InviteAttendeeResponse` - The event invite details
/// # Errors
/// * `ApiError` - If something went wrong while inviting the user to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn invite_to_event(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> Result<InviteAttendeeResponse, ApiError> {
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::invite_to_event(event_id, attendee_principal, group_id)
}

/// Accept an user invite to an event as a admin - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to accept
/// # Returns
/// * `JoinedAttendeeResponse` - the event join details
/// # Errors
/// * `ApiError` - If something went wrong while accepting the user invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn accept_user_request_event_invite(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> Result<JoinedAttendeeResponse, ApiError> {
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::accept_or_decline_user_request_event_invite(
        event_id,
        attendee_principal,
        group_id,
        true,
    )
}

/// Decline an user invite to an event as a admin - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to accept
/// # Returns
/// * `JoinedAttendeeResponse` - the event join details
/// # Errors
/// * `ApiError` - If something went wrong while accepting the user invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn decline_user_request_event_invite(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> Result<JoinedAttendeeResponse, ApiError> {
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::accept_or_decline_user_request_event_invite(
        event_id,
        attendee_principal,
        group_id,
        false,
    )
}

/// Accept an owner invite to an event as a user - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// # Returns
/// * `Attendee` - The attendee
/// # Errors
/// * `ApiError` - If something went wrong while accepting the owner invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn accept_owner_request_event_invite(event_id: u64) -> Result<Attendee, ApiError> {
    EventCalls::accept_or_decline_owner_request_event_invite(event_id, true)
}

/// Decline an owner invite to an event as a user - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// # Returns
/// * `Attendee` - The attendee
/// # Errors
/// * `ApiError` - If something went wrong while accepting the owner invite to the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn decline_owner_request_event_invite(event_id: u64) -> Result<Attendee, ApiError> {
    EventCalls::accept_or_decline_owner_request_event_invite(event_id, false)
}

/// Get the attendees for an event - [`[query]`](query)
/// # Arguments
/// * `event_id` - The event identifier to get the attendees from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The attendees for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendees for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_event_attendees(event_id: u64) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
    EventCalls::get_event_attendees(event_id)
}

/// Get the attendees for an event - [`[query]`](query)
/// # Arguments
/// * `event_id` - The event identifier to get the attendees from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The attendees for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendees for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_event_attendees_profiles_and_roles(
    event_id: u64,
) -> Result<Vec<(ProfileResponse, Vec<String>)>, ApiError> {
    EventCalls::get_event_attendees_profiles_and_roles(event_id)
}

/// Get the attendees for an event with their profiles - [`[query]`](query)
/// # Arguments
/// * `event_id` - The event identifier to get the attendees from
/// # Returns
/// * `Vec<JoinedAttendeeResponse>` - The attendees for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the attendees for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_event_invites_with_profiles(
    event_id: u64,
) -> Result<Vec<(ProfileResponse, InviteAttendeeResponse)>, ApiError> {
    EventCalls::get_event_invites_with_profiles(event_id)
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
pub fn get_self_attendee() -> Result<Attendee, ApiError> {
    EventCalls::get_self_attendee()
}
/// Get the caller joined groups - [`[query]`](query)
/// # Returns
/// * `Vec<GroupResponse>` - All groups the user is part of
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_self_events() -> Vec<EventResponse> {
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
/// * `event_id` - The identifier of the event
/// # Returns
/// * `()` - If the user left the event
/// # Errors
/// * `ApiError` - If something went wrong while leaving the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn leave_event(event_id: u64) -> Result<(), ApiError> {
    EventCalls::leave_event(event_id)
}

/// Remove an event invite as a user - [`[update]`](update)
/// # Change
/// * was `remove_event`
/// # Arguments
/// * `event_id` - The identifier of the event
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_event_invite(event_id: u64) -> Result<(), ApiError> {
    EventCalls::remove_event_invite(event_id)
}

/// Remove an event attendee as a admin - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to remove
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_attendee_from_event(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> Result<(), ApiError> {
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::remove_attendee_from_event(attendee_principal, event_id)
}

/// Remove an event invite as a admin - [`[update]`](update)
/// # Arguments
/// * `event_id` - The identifier of the event
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// * `attendee_principal` - The principal of the user to remove
/// # Returns
/// * `()` - If the user removed the event
/// # Errors
/// * `ApiError` - If something went wrong while removing the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_attendee_invite_from_event(
    event_id: u64,
    group_id: u64,
    attendee_principal: Principal,
) -> Result<(), ApiError> {
    can_edit(group_id, PermissionType::Event(None))?;
    EventCalls::remove_attendee_invite_from_event(attendee_principal, event_id)
}

/// Get the invites for an event - [`[query]`](query)
/// # Arguments
/// * `event_id` - The event identifier to get the invites from
/// * `group_id` - Used to check if the user has access to the group the event belongs to
/// # Returns
/// * `Vec<InviteAttendeeResponse>` - The invites for the event
/// # Errors
/// * `ApiError` - If something went wrong while getting the invites for the event
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[query(guard = "has_access")]
pub fn get_event_invites(
    event_id: u64,
    group_id: u64,
) -> Result<Vec<InviteAttendeeResponse>, ApiError> {
    EventCalls::get_event_invites(event_id, group_id)
}
