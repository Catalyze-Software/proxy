use candid::Principal;
use canister_types::models::identifier::Identifier;

use super::{read_stores::OldData, transform_models::NewData};

pub fn check_data_integrity(old_data: &OldData, new_data: &NewData) -> Vec<Vec<String>> {
    vec![
        check_store_sizes(old_data, new_data),
        check_unique_principals(old_data),
        check_unique_and_ascending_ids(old_data),
        check_joined_and_invites(old_data, new_data),
    ]
}

fn check_store_sizes(old_data: &OldData, new_data: &NewData) -> Vec<String> {
    assert_eq!(old_data.old_members.len(), new_data.new_members.len());
    assert_eq!(old_data.old_groups.len(), new_data.new_groups.len());
    assert_eq!(old_data.old_profiles.len(), new_data.new_profiles.len());
    assert_eq!(old_data.old_events.len(), new_data.new_events.len());
    assert_eq!(
        old_data.old_event_attendees.len(),
        new_data.new_attendees.len()
    );

    vec![
        format!("Profiles store size {}", new_data.new_profiles.len()),
        format!("Members store size {}", new_data.new_members.len()),
        format!("Attendees store size {}", new_data.new_attendees.len()),
        format!("Groups store size {}", new_data.new_groups.len()),
        format!("Events store size {}", new_data.new_events.len()),
    ]
}

fn check_unique_principals(old_data: &OldData) -> Vec<String> {
    let old_profiles_principals: Vec<Principal> = old_data
        .old_profiles
        .iter()
        .map(|(_, profile)| profile.principal)
        .collect();

    let old_members_principals: Vec<Principal> = old_data
        .old_members
        .iter()
        .map(|(_, member)| member.principal)
        .collect();

    let old_attendees_principals: Vec<Principal> = old_data
        .old_event_attendees
        .iter()
        .map(|(_, attendee)| attendee.principal)
        .collect();

    assert_eq!(
        old_profiles_principals.len(),
        old_profiles_principals
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        old_members_principals.len(),
        old_members_principals
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        old_attendees_principals.len(),
        old_attendees_principals
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    vec![
        format!(
            "Profiles store unique principals {}",
            old_profiles_principals.len()
        ),
        format!(
            "Members store unique principals {}",
            old_members_principals.len()
        ),
        format!(
            "Attendees store unique principals {}",
            old_attendees_principals.len()
        ),
    ]
}

fn check_unique_and_ascending_ids(old_data: &OldData) -> Vec<String> {
    let old_group_identifiers: Vec<String> = old_data
        .old_groups
        .iter()
        .map(|(id, _)| id.clone())
        .collect();

    let old_event_identifiers: Vec<String> = old_data
        .old_events
        .iter()
        .map(|(id, _)| id.clone())
        .collect();

    let mut old_group_ids = old_group_identifiers
        .iter()
        .map(|identifier| {
            let principal = Principal::from_text(identifier).unwrap();
            Identifier::from(principal).id()
        })
        .collect::<Vec<u64>>();
    old_group_ids.sort();

    let mut old_event_ids = old_event_identifiers
        .iter()
        .map(|identifier| {
            let principal = Principal::from_text(identifier).unwrap();
            Identifier::from(principal).id()
        })
        .collect::<Vec<u64>>();
    old_event_ids.sort();

    assert_eq!(
        old_group_ids.len(),
        old_group_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        old_event_ids.len(),
        old_event_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );

    assert_eq!(
        old_group_ids,
        (0..old_group_ids.len() as u64).collect::<Vec<u64>>()
    );

    assert_eq!(
        old_event_ids,
        (0..old_event_ids.len() as u64).collect::<Vec<u64>>()
    );

    vec![
        format!(
            "Groups store unique and ascending ids {}",
            old_group_ids.len()
        ),
        format!(
            "Events store unique and ascending ids {}",
            old_event_ids.len()
        ),
    ]
}

fn check_joined_and_invites(old_data: &OldData, new_data: &NewData) -> Vec<String> {
    let mut member_joined_count = 0;
    let mut member_invite_count = 0;

    let mut member_joined_counts = Vec::new();
    let mut member_invite_counts = Vec::new();

    let mut attendee_joined_count = 0;
    let mut attendee_invite_count = 0;

    let mut attendee_joined_counts = Vec::new();
    let mut attendee_invite_counts = Vec::new();

    for (_, member) in old_data.old_members.iter() {
        member_joined_count += member.joined.len();
        member_invite_count += member.invites.len();

        member_joined_counts.push(member.joined.len());
        member_invite_counts.push(member.invites.len());
    }

    for (_, attendee) in old_data.old_event_attendees.iter() {
        attendee_joined_count += attendee.joined.len();
        attendee_invite_count += attendee.invites.len();

        attendee_joined_counts.push(attendee.joined.len());
        attendee_invite_counts.push(attendee.invites.len());
    }

    let mut new_member_joined_count = 0;
    let mut new_member_invite_count = 0;

    let mut new_member_joined_counts = Vec::new();
    let mut new_member_invite_counts = Vec::new();

    let mut new_attendee_joined_count = 0;
    let mut new_attendee_invite_count = 0;

    let mut new_attendee_joined_counts = Vec::new();
    let mut new_attendee_invite_counts = Vec::new();

    for (_, member) in new_data.new_members.iter() {
        new_member_joined_count += member.joined.len();
        new_member_invite_count += member.invites.len();

        new_member_joined_counts.push(member.joined.len());
        new_member_invite_counts.push(member.invites.len());
    }

    for (_, attendee) in new_data.new_attendees.iter() {
        new_attendee_joined_count += attendee.joined.len();
        new_attendee_invite_count += attendee.invites.len();

        new_attendee_joined_counts.push(attendee.joined.len());
        new_attendee_invite_counts.push(attendee.invites.len());
    }

    member_joined_counts.sort();
    member_invite_counts.sort();
    new_member_joined_counts.sort();
    new_member_invite_counts.sort();

    attendee_joined_counts.sort();
    attendee_invite_counts.sort();
    new_attendee_joined_counts.sort();
    new_attendee_invite_counts.sort();

    assert_eq!(member_joined_count, new_member_joined_count);
    assert_eq!(member_invite_count, new_member_invite_count);
    assert_eq!(attendee_joined_count, new_attendee_joined_count);
    assert_eq!(attendee_invite_count, new_attendee_invite_count);

    assert_eq!(member_joined_counts, new_member_joined_counts);
    assert_eq!(member_invite_counts, new_member_invite_counts);
    assert_eq!(attendee_joined_counts, new_attendee_joined_counts);
    assert_eq!(attendee_invite_counts, new_attendee_invite_counts);

    let ok = "OK";
    vec![
        format!("Old members joined count {}", member_joined_count),
        format!("Old members invite count {}", member_invite_count),
        format!("Old attendees joined count {}", attendee_joined_count),
        format!("Old attendees invite count {}", attendee_invite_count),
        format!("New members joined count {}", new_member_joined_count),
        format!("New members invite count {}", new_member_invite_count),
        format!("New attendees joined count {}", new_attendee_joined_count),
        format!("New attendees invite count {}", new_attendee_invite_count),
        format!(
            "Old member joined sorted vector equals new member joined sorted vector {}",
            ok
        ),
        format!(
            "Old member invite sorted vector equals new member invite sorted vector {}",
            ok
        ),
        format!(
            "Old attendee joined sorted vector equals new attendee joined sorted vector {}",
            ok
        ),
        format!(
            "Old attendee invite sorted vector equals new attendee invite sorted vector {}",
            ok
        ),
    ]
}
