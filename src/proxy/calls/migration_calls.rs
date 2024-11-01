use candid::Principal;
use canister_types::models::{
    attendee::Attendee, boosted::Boost, event::Event, event_collection::EventCollection,
    friend_request::FriendRequest, group::Group, member::Member,
    member_collection::MemberCollection, notification::Notification, profile::Profile,
    report::Report, user_notifications::UserNotifications,
};
use ic_cdk::query;

use crate::{
    helpers::guards::is_developer,
    storage::{
        AttendeeStore, BoostedStore, CategoryStore, EventAttendeeStore, EventStore,
        FriendRequestStore, GroupEventsStore, GroupMemberStore, GroupStore, MemberStore,
        NotificationStore, ProfileStore, ReportStore, SkillStore, StorageQueryable, TagStore,
        UserNotificationStore,
    },
};

#[query(guard = "is_developer")]
pub fn mig_profiles_get_all() -> Vec<(Principal, Profile)> {
    ProfileStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_groups_get_all() -> Vec<(u64, Group)> {
    GroupStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_events_get_all() -> Vec<(u64, Event)> {
    EventStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_reports_get_all() -> Vec<(u64, Report)> {
    ReportStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_members_get_all() -> Vec<(Principal, Member)> {
    MemberStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_attendee_get_all() -> Vec<(Principal, Attendee)> {
    AttendeeStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_friend_requests_get_all() -> Vec<(u64, FriendRequest)> {
    FriendRequestStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_boosted_get_all() -> Vec<(u64, Boost)> {
    BoostedStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_notifications_get_all() -> Vec<(u64, Notification)> {
    NotificationStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_user_notifications_get_all() -> Vec<(Principal, UserNotifications)> {
    UserNotificationStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_group_members_get_all() -> Vec<(u64, MemberCollection)> {
    GroupMemberStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_event_attendees_get_all() -> Vec<(u64, MemberCollection)> {
    EventAttendeeStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_group_events_get_all() -> Vec<(u64, EventCollection)> {
    GroupEventsStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_tags_get_all() -> Vec<(u64, String)> {
    TagStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_categories_get_all() -> Vec<(u64, String)> {
    CategoryStore::get_all()
}

#[query(guard = "is_developer")]
pub fn mig_skills_get_all() -> Vec<(u64, String)> {
    SkillStore::get_all()
}
