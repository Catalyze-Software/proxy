use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    attendee::{Attendee, JoinedAttendeeResponse},
};

use crate::storage::{AttendeeStore, StorageMethods};

pub struct AttendeeCalls;
pub struct AttendeeMapper;

impl AttendeeCalls {
    pub fn create_empty_attendee(principal: Principal) -> Result<(Principal, Attendee), ApiError> {
        if let Ok(_) = AttendeeStore::get(principal) {
            return Err(ApiError::duplicate()
                .add_method_name("create_empty_attendee")
                .add_message("Attendee already exists"));
        }

        let new_attendee = Attendee::new();
        AttendeeStore::insert_by_key(principal, new_attendee)
    }
}

impl AttendeeMapper {
    pub fn to_joined_attendee_response(
        principal: Principal,
        attendee: Attendee,
        group_id: u64,
        event_id: u64,
    ) -> Result<JoinedAttendeeResponse, ApiError> {
        if !attendee.is_event_joined(&group_id) {
            return Err(ApiError::not_found()
                .add_method_name("to_joined_attendee_response")
                .add_message("Attendee not joined the group"));
        };
        Ok(JoinedAttendeeResponse::new(event_id, group_id, principal))
    }
}
