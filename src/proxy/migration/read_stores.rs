use super::old_models::attendee_model::Attendee as OldAttendee;
use super::old_models::event_models::Event as OldEvent;
use super::old_models::group_model::Group as OldGroup;
use super::old_models::member_model::Member as OldMember;
use super::old_models::profile_models::Profile as OldProfile;

use super::call_wrappers::{
    read_attendees_entries, read_events_entries, read_groups_entries, read_members_entries,
    read_profile_entries,
};

use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct OldData {
    pub old_members: Vec<(String, OldMember)>,
    pub old_groups: Vec<(String, OldGroup)>,
    pub old_profiles: Vec<(String, OldProfile)>,
    pub old_events: Vec<(String, OldEvent)>,
    pub old_event_attendees: Vec<(String, OldAttendee)>,
    // pub old_notifications: Vec<(u32, String)>,
}

pub async fn read_old_data(env: String) -> OldData {
    let old_members = read_members_entries(env.clone()).await;
    let old_groups = read_groups_entries(env.clone()).await;
    let old_profiles = read_profile_entries(env.clone()).await;
    let old_events = read_events_entries(env.clone()).await;
    let old_event_attendees = read_attendees_entries(env.clone()).await;
    // let old_notifications = read_notifications_entries(env.clone()).await;

    OldData {
        old_members,
        old_groups,
        old_profiles,
        old_events,
        old_event_attendees,
        // old_notifications,
    }
}
