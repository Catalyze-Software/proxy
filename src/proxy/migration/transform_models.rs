use std::collections::HashMap;

use super::{old_models::group_model::GroupRole, read_stores::OldData};
use candid::Principal;

use canister_types::models::{
    attendee::{Attendee, AttendeeInvite, AttendeeJoin},
    event::Event,
    group::Group,
    identifier::Identifier,
    invite_type::InviteType,
    member::{Join, Member, MemberInvite},
    permission::Permission,
    profile::Profile,
    role::Role,
    subject::Subject,
};
use ic_cdk::trap;

use super::old_models::attendee_model::{
    Invite as OldAttendeeInvite, InviteType as OldAttendeeInviteType,
};
use super::old_models::member_model::{Invite as OldInvite, InviteType as OldInviteType};

pub struct NewData {
    pub new_profiles: Vec<(Principal, Profile)>,
    pub new_members: Vec<(Principal, Member)>,
    pub new_attendees: Vec<(Principal, Attendee)>,

    pub new_groups: Vec<(u64, Group)>,
    pub new_events: Vec<(u64, Event)>,
    // pub new_reports: Vec<(u64, Report)>,

    // pub new_friend_requests: Vec<(u64, FriendRequest)>,
    // pub new_boosted: Vec<(u64, Boost)>,
    // pub new_notifications: Vec<(u64, Notification)>,
    // pub new_user_notifications: Vec<(Principal, UserNotifications)>,
}

pub fn transform_models(old_data: &OldData) -> NewData {
    let new_profiles = profiles_from_old(old_data);
    let new_members = members_from_old(old_data);
    let new_attendees = attendees_from_old(old_data);

    let new_groups = groups_from_old(old_data);
    let new_events = events_from_old(old_data);
    // let new_reports = reports_from_old(&old_data);

    // let new_friend_requests = friend_requests_from_old(&old_data);
    // let new_boosted = boosted_from_old(&old_data);

    // let new_notifications = notifications_from_old(&old_data);
    // let new_user_notifications = user_notifications_from_old(&old_data);

    NewData {
        new_profiles,
        new_members,
        new_attendees,
        new_groups,
        new_events,
        // new_reports,
        // new_friend_requests,
        // new_boosted,
        // new_notifications,
        // new_user_notifications,
    }
}

fn profiles_from_old(old_data: &OldData) -> Vec<(Principal, Profile)> {
    let mut new_profiles = Vec::new();

    for (_, old_profile) in old_data.old_profiles.iter() {
        let principal: Principal = old_profile.principal;

        let new_starred: Vec<Subject> = old_profile
            .starred
            .clone()
            .into_keys()
            .map(|identifier| {
                let x = Identifier::from(identifier);
                match x.kind().as_str() {
                    "grp" => Subject::Group(x.id()),
                    "evt" => Subject::Event(x.id()),
                    // should not have any other kind
                    _ => trap(format!("Unknown identifier kind: {}", x.kind()).as_str()),
                }
            })
            .collect();

        let profile = Profile {
            username: old_profile.username.clone(),
            display_name: old_profile.display_name.clone(),
            application_role: old_profile.application_role.clone(),
            first_name: old_profile.first_name.clone(),
            last_name: old_profile.last_name.clone(),

            privacy: old_profile.privacy.clone(),

            about: old_profile.about.clone(),
            email: old_profile.email.clone(),
            date_of_birth: old_profile.date_of_birth,
            city: old_profile.city.clone(),
            state_or_province: old_profile.state_or_province.clone(),
            country: old_profile.country.clone(),
            profile_image: old_profile.profile_image.clone(),
            banner_image: old_profile.banner_image.clone(),
            skills: old_profile.skills.clone(),
            interests: old_profile.interests.clone(),
            causes: old_profile.causes.clone(),
            website: old_profile.website.clone(),

            code_of_conduct: Some(old_profile.code_of_conduct.clone()),
            privacy_policy: old_profile.privacy_policy.clone(),
            terms_of_service: old_profile.terms_of_service.clone(),
            wallets: old_profile.wallets.clone(),

            starred: new_starred,
            relations: old_profile.relations.clone(),
            extra: old_profile.extra.clone(),
            updated_on: old_profile.updated_on,
            created_on: old_profile.created_on,
            notification_id: None,
            pinned: vec![],
        };

        new_profiles.push((principal, profile));
    }

    new_profiles
}

fn members_from_old(old_data: &OldData) -> Vec<(Principal, Member)> {
    let mut new_members = Vec::new();

    for (_, old_member) in old_data.old_members.iter() {
        let principal: Principal = old_member.principal;

        let joined: HashMap<u64, Join> = old_member
            .joined
            .iter()
            .map(|(identifier, join)| {
                let id = Identifier::from(*identifier).id();
                let join = Join {
                    roles: join.roles.clone(),
                    updated_at: join.updated_at,
                    created_at: join.created_at,
                };
                (id, join)
            })
            .collect::<HashMap<u64, Join>>();

        let invites: HashMap<u64, MemberInvite> = old_member
            .invites
            .iter()
            .map(|(identifier, invite)| {
                let id = Identifier::from(*identifier).id();
                let invite = MemberInvite::from(invite.clone());
                (id, invite)
            })
            .collect::<HashMap<u64, MemberInvite>>();

        let member = Member { joined, invites };

        new_members.push((principal, member));
    }

    new_members
}

impl From<OldInvite> for MemberInvite {
    fn from(old_invite: OldInvite) -> Self {
        let invite_type = match old_invite.invite_type {
            OldInviteType::OwnerRequest => InviteType::OwnerRequest,
            OldInviteType::UserRequest => InviteType::UserRequest,
        };
        MemberInvite {
            invite_type,
            updated_at: old_invite.updated_at,
            created_at: old_invite.created_at,
            notification_id: None,
        }
    }
}

fn attendees_from_old(old_data: &OldData) -> Vec<(Principal, Attendee)> {
    let mut new_attendees = Vec::new();

    for (_, old_attendee) in old_data.old_event_attendees.iter() {
        let principal: Principal = old_attendee.principal;

        let joined: HashMap<u64, AttendeeJoin> = old_attendee
            .joined
            .iter()
            .map(|(identifier, join)| {
                let id = Identifier::from(*identifier).id();
                let join = AttendeeJoin {
                    group_id: Identifier::from(join.group_identifier).id(),
                    updated_at: join.updated_at,
                    created_at: join.created_at,
                };
                (id, join)
            })
            .collect::<HashMap<u64, AttendeeJoin>>();

        let invites: HashMap<u64, AttendeeInvite> = old_attendee
            .invites
            .iter()
            .map(|(identifier, invite)| {
                let id = Identifier::from(*identifier).id();
                let invite = AttendeeInvite::from(invite.clone());
                (id, invite)
            })
            .collect::<HashMap<u64, AttendeeInvite>>();

        let attendee = Attendee { joined, invites };

        new_attendees.push((principal, attendee));
    }

    new_attendees
}

impl From<OldAttendeeInvite> for AttendeeInvite {
    fn from(old_invite: OldAttendeeInvite) -> Self {
        let invite_type: InviteType = match old_invite.invite_type {
            OldAttendeeInviteType::OwnerRequest => InviteType::OwnerRequest,
            OldAttendeeInviteType::UserRequest => InviteType::UserRequest,
            OldAttendeeInviteType::None => panic!("OldAttendeeInviteType::None is not allowed"),
        };
        AttendeeInvite {
            group_id: Identifier::from(old_invite.group_identifier).id(),
            invite_type,
            updated_at: old_invite.updated_at,
            created_at: old_invite.created_at,
            notification_id: None,
        }
    }
}

fn groups_from_old(old_data: &OldData) -> Vec<(u64, Group)> {
    let mut new_groups = Vec::new();

    for (identifier, old_group) in old_data.old_groups.iter() {
        let id = Identifier::from(Principal::from_text(identifier).unwrap()).id();

        let group = Group {
            name: old_group.name.clone(),
            description: old_group.description.clone(),
            privacy: old_group.privacy.clone(),
            tags: old_group.tags.clone(),
            location: old_group.location.clone(),
            banner_image: old_group.banner_image.clone(),
            updated_on: old_group.updated_on,
            created_on: old_group.created_on,
            website: old_group.website.clone(),
            owner: old_group.owner,
            created_by: old_group.created_by,
            matrix_space_id: old_group.matrix_space_id.clone(),
            image: old_group.image.clone(),
            privacy_gated_type_amount: old_group.privacy_gated_type_amount,
            roles: old_group
                .roles
                .iter()
                .map(|role| Role::from(role.clone()))
                .collect(),
            is_deleted: old_group.is_deleted,
            wallets: old_group.wallets.clone(),
            notification_id: None,
            special_members: HashMap::new(),
        };

        new_groups.push((id, group));
    }

    // sort by ascending u64 in Vec<(u64, Group)>
    new_groups.sort_by(|a, b| a.0.cmp(&b.0));

    new_groups
}

impl From<GroupRole> for Role {
    fn from(group_role: GroupRole) -> Self {
        Self {
            name: group_role.name,
            protected: group_role.protected,
            permissions: group_role
                .permissions
                .into_iter()
                .map(Permission::from)
                .collect(),
            color: group_role.color,
            index: group_role.index,
        }
    }
}

fn events_from_old(old_data: &OldData) -> Vec<(u64, Event)> {
    let mut new_events = Vec::new();

    for (identifier, old_event) in old_data.old_events.iter() {
        let id = Identifier::from(Principal::from_text(identifier).unwrap()).id();

        let event = Event {
            name: old_event.name.clone(),
            description: old_event.description.clone(),
            privacy: old_event.privacy.clone(),
            tags: old_event.tags.clone(),
            location: old_event.location.clone(),
            banner_image: old_event.banner_image.clone(),
            updated_on: old_event.updated_on,
            created_on: old_event.created_on,
            website: old_event.website.clone(),
            owner: old_event.owner,
            created_by: old_event.created_by,
            group_id: Identifier::from(old_event.group_identifier).id(),
            is_canceled: old_event.is_canceled.clone(),
            metadata: old_event.metadata.clone(),
            date: old_event.date.clone(),
            image: old_event.image.clone(),
            is_deleted: old_event.is_deleted,
        };

        new_events.push((id, event));
    }

    // sort by ascending u64 in Vec<(u64, Event)>
    new_events.sort_by(|a, b| a.0.cmp(&b.0));

    new_events
}

// fn reports_from_old(old_data: &OldData) -> Vec<(u64, Report)> {
//     todo!()
// }

// fn friend_requests_from_old(old_data: &OldData) -> Vec<(u64, FriendRequest)> {
//     todo!()
// }

// fn boosted_from_old(old_data: &OldData) -> Vec<(u64, Boost)> {
//     todo!()
// }

// fn notifications_from_old(old_data: &OldData) -> Vec<(u64, Notification)> {
//     todo!()
// }

// fn user_notifications_from_old(old_data: &OldData) -> Vec<(Principal, UserNotifications)> {
//     todo!()
// }
