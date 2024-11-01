use std::collections::{HashMap, HashSet};

use canister_types::models::{
    event_collection::EventCollection, member_collection::MemberCollection,
};
use ic_cdk::query;

use crate::storage::{
    storage_api::{ATTENDEES, EVENTS, EVENT_ATTENDEES, GROUP_EVENTS, GROUP_MEMBERS, MEMBERS},
    AttendeeStore, EventAttendeeStore, EventStore, GroupEventsStore, GroupMemberStore, GroupStore,
    MemberStore, ProfileStore, StorageInsertable, StorageInsertableByKey, StorageUpdateable,
};

use super::{
    read_stores::read_old_data, sanity_check::check_data_integrity,
    transform_models::transform_models,
};

#[ic_cdk::update]
async fn migrate() -> Vec<Vec<String>> {
    let old_data = read_old_data("staging".to_string()).await;

    let new_data = transform_models(&old_data);

    let mut results = check_data_integrity(&old_data, &new_data);

    for (key, profile) in new_data.new_profiles {
        ProfileStore::insert_by_key(key, profile).expect("Failed to insert profile");
    }

    for (group_id, group) in new_data.new_groups {
        GroupStore::insert(group).expect("Failed to insert group");
        GroupMemberStore::insert_by_key(group_id, MemberCollection::new())
            .expect("Failed to insert group members");
        GroupEventsStore::insert_by_key(group_id, EventCollection::new())
            .expect("Failed to insert group events");
    }

    // EVENTS
    let mut event_collections: HashMap<u64, EventCollection> = HashMap::new();

    for (event_id, event) in new_data.new_events {
        EventStore::insert(event.clone()).expect("Failed to insert event");
        EventAttendeeStore::insert_by_key(event_id, MemberCollection::new())
            .expect("Failed to insert event attendees");

        match event_collections.get(&event.group_id) {
            Some(collection) => {
                let mut new_collection = collection.clone();
                new_collection.add_event(event_id);
                event_collections.insert(event.group_id, new_collection);
            }
            None => {
                let mut new_collection = EventCollection::new();
                new_collection.add_event(event_id);
                event_collections.insert(event.group_id, new_collection);
            }
        }
    }

    let group_events_hm_size = event_collections.len();

    for (group_id, collection) in event_collections {
        GroupEventsStore::update(group_id, collection).expect("Failed to insert group events");
    }

    // MEMBERS
    let mut member_collections: HashMap<u64, MemberCollection> = HashMap::new();

    for (key, member) in new_data.new_members {
        MemberStore::insert_by_key(key, member.clone()).expect("Failed to insert member");

        for (group_id, _) in member.joined {
            match member_collections.get(&group_id) {
                Some(collection) => {
                    let mut new_collection = collection.clone();
                    new_collection.add_member(key);
                    member_collections.insert(group_id, new_collection);
                }
                None => {
                    let mut new_collection = MemberCollection::new();
                    new_collection.add_member(key);
                    member_collections.insert(group_id, new_collection);
                }
            }
        }

        for (group_id, _) in member.invites {
            match member_collections.get(&group_id) {
                Some(collection) => {
                    let mut new_collection = collection.clone();
                    new_collection.add_invite(key);
                    member_collections.insert(group_id, new_collection);
                }
                None => {
                    let mut new_collection = MemberCollection::new();
                    new_collection.add_invite(key);
                    member_collections.insert(group_id, new_collection);
                }
            }
        }
    }

    let group_members_hm_size = member_collections.len();

    for (group_id, collection) in member_collections {
        GroupMemberStore::update(group_id, collection).expect("Failed to insert group members");
    }

    // ATTENDEES
    let mut attendee_collections: HashMap<u64, MemberCollection> = HashMap::new();
    for (key, attendee) in new_data.new_attendees {
        AttendeeStore::insert_by_key(key, attendee.clone()).expect("Failed to insert attendee");

        for (event_id, _) in attendee.joined {
            match attendee_collections.get(&event_id) {
                Some(collection) => {
                    let mut new_collection = collection.clone();
                    new_collection.add_member(key);
                    attendee_collections.insert(event_id, new_collection);
                }
                None => {
                    let mut new_collection = MemberCollection::new();
                    new_collection.add_member(key);
                    attendee_collections.insert(event_id, new_collection);
                }
            }
        }

        for (event_id, _) in attendee.invites {
            match attendee_collections.get(&event_id) {
                Some(collection) => {
                    let mut new_collection = collection.clone();
                    new_collection.add_invite(key);
                    attendee_collections.insert(event_id, new_collection);
                }
                None => {
                    let mut new_collection = MemberCollection::new();
                    new_collection.add_invite(key);
                    attendee_collections.insert(event_id, new_collection);
                }
            }
        }
    }

    let event_attendees_hm_size = attendee_collections.len();

    for (event_id, collection) in attendee_collections {
        EventAttendeeStore::update(event_id, collection).expect("Failed to insert event attendees");
    }

    // Check new store sizes
    let group_members_size = GROUP_MEMBERS.with(|store| store.borrow().len());
    let group_events_size = GROUP_EVENTS.with(|store| store.borrow().len());
    let event_attendees_size = EVENT_ATTENDEES.with(|store| store.borrow().len());

    results.push(vec![
        format!("Group members: {}", group_members_size),
        format!("Group events: {}", group_events_size),
        format!("Event attendees: {}", event_attendees_size),
    ]);

    results.push(vec![
        format!("Group members hashmap: {}", group_members_hm_size),
        format!("Group events hashmap: {}", group_events_hm_size),
        format!("Event attendees hashmap: {}", event_attendees_hm_size),
    ]);

    results
}

#[query]
fn check_new_stores() -> Vec<String> {
    // Check new store sizes
    let group_members_size = GROUP_MEMBERS.with(|store| store.borrow().len());
    let group_events_size = GROUP_EVENTS.with(|store| store.borrow().len());
    let event_attendees_size = EVENT_ATTENDEES.with(|store| store.borrow().len());

    let mut group_members: HashSet<u64> = HashSet::new();
    let mut group_events: HashSet<u64> = HashSet::new();
    let mut event_attendees: HashSet<u64> = HashSet::new();

    MEMBERS.with(|store| {
        for (_, member) in store.borrow().iter() {
            for (group_id, _) in member.joined.iter() {
                group_members.insert(*group_id);
            }

            for (group_id, _) in member.invites.iter() {
                group_members.insert(*group_id);
            }
        }
    });

    EVENTS.with(|store| {
        for (_, event) in store.borrow().iter() {
            group_events.insert(event.group_id);
        }
    });

    ATTENDEES.with(|store| {
        for (_, attendee) in store.borrow().iter() {
            for group_id in attendee.joined.iter() {
                event_attendees.insert(*group_id.0);
            }
            for group_id in attendee.invites.iter() {
                event_attendees.insert(*group_id.0);
            }
        }
    });

    vec![
        format!("Group members hashset: {}", group_members.len()),
        format!("Group events hashset: {}", group_events.len()),
        format!("Event attendees hashset: {}", event_attendees.len()),
        format!("Group members store: {}", group_members_size),
        format!("Group events store: {}", group_events_size),
        format!("Event attendees store: {}", event_attendees_size),
    ]
}
