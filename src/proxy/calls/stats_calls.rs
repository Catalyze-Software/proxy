use crate::helpers::guards::is_monitor;
use crate::storage::*;
use ic_cdk::query;

#[query(guard = "is_monitor")]
fn store_stats() -> Vec<String> {
    let profile_store_size = ProfileStore::size();
    let friend_request_store_size = FriendRequestStore::size();
    let groups_store_size = GroupStore::size();
    let members_store_size = MemberStore::size();
    let events_store_size = EventStore::size();
    let attendees_store_size = AttendeeStore::size();
    let reports_store_size = ReportStore::size();
    let boosted_store_size = BoostedStore::size();
    let notifications_store_size = NotificationStore::size();
    let user_notifications_store_size = UserNotificationStore::size();
    let logs_store_size = LoggerStore::size();
    let group_members_store_size = GroupMemberStore::size();
    let group_events_store_size = GroupEventsStore::size();
    let event_attendees_store_size = EventAttendeeStore::size();
    let tags_store_size = TagStore::size();
    let category_store_size = CategoryStore::size();
    let skills_store_size = SkillStore::size();

    vec![
        format!("ProfileStore: {}", profile_store_size),
        format!("FriendRequestStore: {}", friend_request_store_size),
        format!("GroupStore: {}", groups_store_size),
        format!("MemberStore: {}", members_store_size),
        format!("EventStore: {}", events_store_size),
        format!("AttendeeStore: {}", attendees_store_size),
        format!("ReportStore: {}", reports_store_size),
        format!("BoostStore: {}", boosted_store_size),
        format!("NotificationStore: {}", notifications_store_size),
        format!("UserNotificationsStore: {}", user_notifications_store_size),
        format!("LoggerStore: {}", logs_store_size),
        format!("GroupMembersStore: {}", group_members_store_size),
        format!("GroupEventsStore: {}", group_events_store_size),
        format!("EventAttendeesStore: {}", event_attendees_store_size),
        format!("TagStore: {}", tags_store_size),
        format!("CategoryStore: {}", category_store_size),
        format!("SkillStore: {}", skills_store_size),
    ]
}
