use crate::{
    helpers::time_helper::hours_to_nanoseconds,
    storage::{
        AttendeeStore, BoostedStore, EventAttendeeStore, EventStore, GroupEventsStore, MemberStore,
        ProfileStore, StorageInsertable, StorageInsertableByKey, StorageQueryable,
        StorageUpdateable,
    },
};

use super::{
    boost_logic::BoostCalls, notification_logic::NotificationCalls, profile_logic::ProfileCalls,
};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    attendee::{Attendee, InviteAttendeeResponse, JoinedAttendeeResponse},
    boosted::Boost,
    date_range::DateRange,
    event::{
        Event, EventCallerData, EventFilter, EventResponse, EventSort, EventsCount, PostEvent,
        UpdateEvent,
    },
    event_collection::EventCollection,
    invite_type::InviteType,
    member_collection::MemberCollection,
    paged_response::PagedResponse,
    privacy::Privacy,
    profile::ProfileResponse,
    subject::{Subject, SubjectType},
};
use ic_cdk::{api::time, caller};
use std::collections::HashMap;

pub struct EventCalls;

impl EventCalls {
    pub fn add_event(post_event: PostEvent) -> Result<EventResponse, ApiError> {
        let (new_event_id, new_event) = EventStore::insert(Event::from(post_event.clone()))?;

        let (_, mut attendee) = AttendeeStore::get(caller())?;

        attendee.add_joined(new_event_id, new_event.group_id);
        AttendeeStore::update(caller(), attendee)?;

        // initialize attendees with the caller
        let mut attendees = MemberCollection::new();
        attendees.add_member(caller());
        EventAttendeeStore::insert_by_key(new_event_id, attendees)?;

        // initialize group events with the new event
        let mut group_events = EventCollection::new();
        group_events.add_event(new_event_id);
        GroupEventsStore::update(post_event.group_id, group_events)?;

        Ok(EventResponse::new(
            new_event_id,
            new_event.clone(),
            Self::get_boosted_event(new_event_id),
            Self::get_event_caller_data(new_event_id, new_event.group_id),
            Self::get_attendees_count(new_event_id),
        ))
    }

    pub fn get_event(event_id: u64) -> Result<EventResponse, ApiError> {
        let (_, event) = EventStore::get(event_id)?;

        if event.match_privacy(Privacy::InviteOnly) {
            let (_, caller_attendee) = AttendeeStore::get(caller())?;

            if caller_attendee.is_event_joined(&event_id) {
                Ok(EventResponse::new(
                    event_id,
                    event.clone(),
                    Self::get_boosted_event(event_id),
                    Self::get_event_caller_data(event_id, event.group_id),
                    Self::get_attendees_count(event_id),
                ))
            } else {
                Err(ApiError::unauthorized())
            }
        } else {
            Ok(EventResponse::new(
                event_id,
                event.clone(),
                Self::get_boosted_event(event_id),
                Self::get_event_caller_data(event_id, event.group_id),
                Self::get_attendees_count(event_id),
            ))
        }
    }

    pub fn get_events(
        limit: usize,
        page: usize,
        sort: EventSort,
        filters: Vec<EventFilter>,
    ) -> Result<PagedResponse<EventResponse>, ApiError> {
        // get all the events and filter them based on the privacy
        // exclude all InviteOnly events that the caller is not a attendee of
        let mut events = EventStore::filter(|event_id, event| {
            if event.match_privacy(Privacy::InviteOnly) {
                if let Ok((_, caller_attendee)) = AttendeeStore::get(caller()) {
                    return caller_attendee.is_event_joined(event_id);
                }
                return false;
            }
            true
        })
        .into_iter()
        .collect::<HashMap<u64, Event>>();

        for filter in filters {
            for (id, event) in &events.clone() {
                if !filter.is_match(id, event) {
                    events.remove(id);
                }
            }
        }

        let sorted_events = sort.sort(events);
        let result: Vec<EventResponse> = sorted_events
            .into_iter()
            .map(|data| {
                EventResponse::new(
                    data.0,
                    data.1.clone(),
                    Self::get_boosted_event(data.0),
                    Self::get_event_caller_data(data.0, data.1.group_id),
                    Self::get_attendees_count(data.0),
                )
            })
            .collect();

        Ok(PagedResponse::new(page, limit, result))
    }

    pub fn edit_event(
        event_id: u64,
        update_event: UpdateEvent,
        group_id: u64,
    ) -> Result<EventResponse, ApiError> {
        let (_, mut event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        event = event.update(update_event);
        EventStore::update(event_id, event.clone())?;

        Ok(EventResponse::new(
            event_id,
            event.clone(),
            Self::get_boosted_event(event_id),
            Self::get_event_caller_data(event_id, event.group_id),
            Self::get_attendees_count(event_id),
        ))
    }

    pub fn get_boosted_events() -> Vec<EventResponse> {
        BoostCalls::get_boosts_by_subject(SubjectType::Event)
            .into_iter()
            .map(|(_, boost)| Self::get_event(*boost.subject.get_id()).unwrap())
            .collect()
    }

    pub fn get_events_count(group_ids: Option<Vec<u64>>, query: Option<String>) -> EventsCount {
        let events = match group_ids {
            Some(ids) => EventStore::filter(|_, event| {
                ids.contains(&event.group_id)
                    && match &query {
                        Some(q) => event.name.to_lowercase().contains(&q.to_lowercase()),
                        None => true,
                    }
            }),
            None => EventStore::get_all(),
        };

        let (attending, invited) = match AttendeeStore::get(caller()) {
            Ok((_, attendee)) => {
                let joined = attendee.joined.keys().cloned().collect::<Vec<_>>();
                let invites = attendee.invites.keys().cloned().collect();
                (joined, invites)
            }
            Err(_) => (vec![], vec![]),
        };

        let new = events
            .iter()
            .filter(|(_, event)| {
                DateRange::new(time() - hours_to_nanoseconds(24), time())
                    .is_within(event.created_on)
            })
            .count() as u64;

        let future = events
            .iter()
            .filter(|(_, event)| event.date.is_after_start_date(time()));

        let past = events
            .iter()
            .filter(|(_, event)| event.date.is_before_start_date(time()))
            .count() as u64;

        let starred = ProfileCalls::get_starred_by_subject(SubjectType::Event).len() as u64;

        EventsCount {
            total: events.len() as u64,
            attending: future
                .clone()
                .filter(|(id, _)| attending.contains(id))
                .count() as u64,
            invited: future
                .clone()
                .filter(|(id, _)| invited.contains(id))
                .count() as u64,
            starred,
            new,
            future: future.count() as u64,
            past,
        }
    }

    pub fn delete_event(event_id: u64, group_id: u64) -> Result<(), ApiError> {
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        let _ = EventStore::remove(event_id);

        let event_attendees =
            EventAttendeeStore::get(event_id).map_or(MemberCollection::new(), |(_, m)| m);

        if let Some((boost_id, _)) =
            BoostedStore::find(|_, b| b.subject == Subject::Event(event_id))
        {
            BoostedStore::remove(boost_id);
        }

        // remove all groups from the members
        for member in event_attendees.get_member_principals() {
            // remove all pinned and starred from the profiles
            if let Ok((_, mut profile)) = ProfileStore::get(member) {
                let subject = Subject::Event(event_id);

                if profile.is_starred(&subject) || profile.is_pinned(&subject) {
                    profile.remove_starred(&subject);
                    profile.remove_pinned(&subject);
                    ProfileStore::update(member, profile).unwrap();
                }
            }

            if let Ok((principal, mut attendee)) = AttendeeStore::get(member) {
                attendee.remove_joined(group_id);
                AttendeeStore::update(principal, attendee).unwrap();
            }
        }

        // remove all invites from the members
        for member in event_attendees.get_invite_principals() {
            if let Ok((principal, mut attendee)) = AttendeeStore::get(member) {
                attendee.remove_invite(group_id);
                AttendeeStore::update(principal, attendee).unwrap();
            }
        }

        // remove attendees from the event
        EventAttendeeStore::remove(event_id);

        // remove event from group events
        let mut group_events =
            GroupEventsStore::get(group_id).map_or(EventCollection::new(), |(_, m)| m);
        group_events.remove_event(&event_id);
        GroupEventsStore::update(group_id, group_events)?;
        Ok(())
    }

    pub fn cancel_event(event_id: u64, reason: String, group_id: u64) -> Result<(), ApiError> {
        let (_, mut event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        event = event.cancel(reason);
        EventStore::update(event_id, event.clone())?;

        Ok(())
    }

    // Attendee methods
    pub fn join_event(event_id: u64) -> Result<JoinedAttendeeResponse, ApiError> {
        let (attendee_principal, mut attendee) = AttendeeStore::get(caller())?;
        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        let (_, event) = EventStore::get(event_id)?;

        match event.privacy {
            Privacy::Private => {
                let invite_attendee_response = InviteAttendeeResponse::new(
                    event_id,
                    event.group_id,
                    caller(),
                    InviteType::UserRequest,
                );
                let notification_id = NotificationCalls::notification_user_join_request_event(
                    vec![event.owner],
                    invite_attendee_response,
                )?;

                attendee.add_invite(
                    event_id,
                    event.group_id,
                    InviteType::UserRequest,
                    Some(notification_id),
                );
                attendees.add_invite(caller());
            }
            Privacy::Public => {
                NotificationCalls::notification_join_public_event(
                    vec![event.owner],
                    event.group_id,
                    event_id,
                );
                attendee.add_joined(event_id, event.group_id);
                attendees.add_member(caller());
            }
            _ => {
                return Err(ApiError::unsupported()
                    .add_message("This privacy type is not supported for events"));
            }
        }
        AttendeeStore::update(attendee_principal, attendee)?;
        EventAttendeeStore::update(event_id, attendees)?;

        Ok(JoinedAttendeeResponse::new(
            event_id,
            event.group_id,
            attendee_principal,
        ))
    }

    pub fn invite_to_event(
        event_id: u64,
        attendee_principal: Principal,
        group_id: u64,
    ) -> Result<InviteAttendeeResponse, ApiError> {
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        let (attendee_principal, mut attendee) = AttendeeStore::get(attendee_principal)?;

        let invite_attendee_response = InviteAttendeeResponse::new(
            event_id,
            group_id,
            attendee_principal,
            InviteType::OwnerRequest,
        );

        let notification_id = NotificationCalls::notification_owner_join_request_event(
            attendee_principal,
            invite_attendee_response.clone(),
            vec![],
        )?;

        attendee.add_invite(
            event_id,
            group_id,
            InviteType::OwnerRequest,
            Some(notification_id),
        );
        AttendeeStore::update(attendee_principal, attendee)?;

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.add_invite(attendee_principal);
        EventAttendeeStore::update(event_id, attendees)?;

        Ok(invite_attendee_response)
    }

    pub fn accept_or_decline_user_request_event_invite(
        event_id: u64,
        attendee_principal: Principal,
        group_id: u64,
        accept: bool,
    ) -> Result<JoinedAttendeeResponse, ApiError> {
        let (_, event) = EventStore::get(event_id)?;

        if !event.is_from_group(group_id) {
            return Err(ApiError::unauthorized());
        }

        let (attendee_principal, mut attendee) = AttendeeStore::get(attendee_principal)?;

        if !attendee.has_pending_join_request(event_id) {
            return Err(ApiError::not_found());
        }

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;

        if let Some(invite) = attendee.get_invite(&event_id) {
            if accept {
                attendee.turn_invite_into_joined(event_id);
                attendees.create_member_from_invite(attendee_principal);
            } else {
                attendee.remove_invite(event_id);
                attendees.remove_invite(&attendee_principal);
            }

            AttendeeStore::update(attendee_principal, attendee)?;
            EventAttendeeStore::update(event_id, attendees.clone())?;

            NotificationCalls::notification_user_join_request_event_accept_or_decline(
                attendee_principal,
                invite,
                attendees.get_member_principals(),
                accept,
            )?;
        }

        Ok(JoinedAttendeeResponse::new(
            event_id,
            group_id,
            attendee_principal,
        ))
    }

    pub fn accept_or_decline_owner_request_event_invite(
        event_id: u64,
        accept: bool,
    ) -> Result<Attendee, ApiError> {
        let (_, mut attendee) = AttendeeStore::get(caller())?;

        if !attendee.has_pending_invite(event_id) {
            return Err(
                ApiError::not_found().add_message("Attendee does not have a pending invite")
            );
        }

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;

        if let Some(invite) = attendee.get_invite(&event_id) {
            if accept {
                attendee.turn_invite_into_joined(event_id);
                attendees.create_member_from_invite(caller());
            } else {
                attendee.remove_invite(event_id);
                attendees.remove_invite(&caller());
            }

            AttendeeStore::update(caller(), attendee.clone())?;
            EventAttendeeStore::update(event_id, attendees.clone())?;

            NotificationCalls::notification_owner_join_request_event_accept_or_decline(
                caller(),
                invite,
                attendees.get_member_principals(),
                accept,
            )?;
        }
        Ok(attendee)
    }

    pub fn get_event_attendees(event_id: u64) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
        let (_, event_attendees) = EventAttendeeStore::get(event_id)?;
        let attendees = AttendeeStore::get_many(event_attendees.get_member_principals());

        let response = attendees
            .into_iter()
            .map(|(principal, attendee)| {
                JoinedAttendeeResponse::new(
                    event_id,
                    attendee.joined.get(&event_id).unwrap().group_id,
                    principal,
                )
            })
            .collect();
        Ok(response)
    }

    pub fn get_event_attendees_profiles_and_roles(
        event_id: u64,
    ) -> Result<Vec<(ProfileResponse, Vec<String>)>, ApiError> {
        let (_, event_attendees) = EventAttendeeStore::get(event_id)?;
        let (_, event) = EventStore::get(event_id)?;

        let mut result: Vec<(ProfileResponse, Vec<String>)> = vec![];

        for principal in event_attendees.get_member_principals() {
            if let Ok((_, profile)) = ProfileStore::get(principal) {
                if let Ok((_, member)) = MemberStore::get(principal) {
                    let roles = member.get_roles(event.group_id);
                    result.push((ProfileResponse::new(principal, profile), roles));
                }
            }
        }

        Ok(result)
    }

    pub fn get_event_invites(
        event_id: u64,
        group_id: u64,
    ) -> Result<Vec<InviteAttendeeResponse>, ApiError> {
        let (_, event_attendees) = EventAttendeeStore::get(event_id)?;

        let invites = AttendeeStore::get_many(event_attendees.get_invite_principals())
            .iter()
            .map(|(principal, attendee)| {
                let invite_type = attendee.invites.get(&event_id).unwrap().invite_type.clone();
                InviteAttendeeResponse::new(event_id, group_id, *principal, invite_type)
            })
            .collect();

        Ok(invites)
    }

    pub fn get_event_invites_with_profiles(
        event_id: u64,
    ) -> Result<Vec<(ProfileResponse, InviteAttendeeResponse)>, ApiError> {
        let (_, event_attendees) = EventAttendeeStore::get(event_id)?;
        let (_, event) = EventStore::get(event_id)?;

        let mut result: Vec<(ProfileResponse, InviteAttendeeResponse)> = vec![];

        for (principal, profile) in ProfileStore::get_many(event_attendees.get_invite_principals())
        {
            if let Ok((_, attendee)) = AttendeeStore::get(principal) {
                let invite = attendee.get_invite(&event_id);
                if let Some(invite) = invite {
                    result.push((
                        ProfileResponse::new(principal, profile),
                        InviteAttendeeResponse::new(
                            event_id,
                            event.group_id,
                            principal,
                            invite.invite_type,
                        ),
                    ));
                }
            }
        }

        Ok(result)
    }

    pub fn get_self_attendee() -> Result<Attendee, ApiError> {
        let (_, attendee) = AttendeeStore::get(caller())?;
        Ok(attendee)
    }

    pub fn get_self_events() -> Vec<EventResponse> {
        match AttendeeStore::get(caller()) {
            Ok((_, attendee)) => {
                let events = Self::get_events_by_id(
                    attendee.get_multiple_joined().iter().map(|g| g.0).collect(),
                );
                events
            }
            Err(_) => vec![],
        }
    }

    pub fn get_events_by_id(event_ids: Vec<u64>) -> Vec<EventResponse> {
        EventStore::get_many(event_ids)
            .into_iter()
            .map(|data| {
                EventResponse::new(
                    data.0,
                    data.1.clone(),
                    Self::get_boosted_event(data.0),
                    Self::get_event_caller_data(data.0, data.1.group_id),
                    Self::get_attendees_count(data.0),
                )
            })
            .collect()
    }

    pub fn get_attending_from_principal(
        principal: Principal,
    ) -> Result<Vec<JoinedAttendeeResponse>, ApiError> {
        let (_, attendee) = AttendeeStore::get(principal)?;
        let response = attendee
            .joined
            .into_iter()
            .map(|(event_id, join)| JoinedAttendeeResponse::new(event_id, join.group_id, principal))
            .collect();
        Ok(response)
    }

    pub fn leave_event(event_id: u64) -> Result<(), ApiError> {
        let (_, mut attendee) = AttendeeStore::get(caller())?;
        if !attendee.is_event_joined(&event_id) {
            return Err(ApiError::not_found());
        }

        let (_, event) = EventStore::get(event_id)?;
        if event.owner == caller() {
            return Err(ApiError::bad_request().add_message("Owner cannot leave event"));
        }

        attendee.remove_joined(event_id);
        AttendeeStore::update(caller(), attendee)?;

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_member(&caller());
        EventAttendeeStore::update(event_id, attendees)?;

        Ok(())
    }

    pub fn remove_event_invite(event_id: u64) -> Result<(), ApiError> {
        let (_, mut attendee) = AttendeeStore::get(caller())?;
        if !attendee.has_pending_invite(event_id) || !attendee.has_pending_join_request(event_id) {
            return Err(ApiError::not_found());
        }

        attendee.remove_invite(event_id);
        AttendeeStore::update(caller(), attendee)?;

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_invite(&caller());
        EventAttendeeStore::update(event_id, attendees)?;

        Ok(())
    }

    pub fn remove_attendee_from_event(
        attendee_principal: Principal,
        event_id: u64,
    ) -> Result<(), ApiError> {
        let (_, mut attendee) = AttendeeStore::get(attendee_principal)?;
        if !attendee.is_event_joined(&event_id) {
            return Err(ApiError::not_found());
        }

        let (_, event) = EventStore::get(event_id)?;

        attendee.remove_joined(event_id);
        AttendeeStore::update(attendee_principal, attendee)?;

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_member(&attendee_principal);
        EventAttendeeStore::update(event_id, attendees.clone())?;

        NotificationCalls::notification_remove_event_attendee(
            JoinedAttendeeResponse::new(event_id, event.group_id, attendee_principal),
            attendees.get_member_principals(),
        );

        Ok(())
    }

    pub fn remove_attendee_invite_from_event(
        attendee_principal: Principal,
        event_id: u64,
    ) -> Result<(), ApiError> {
        let (_, mut attendee) = AttendeeStore::get(attendee_principal)?;
        if !attendee.is_event_invited(&event_id) {
            return Err(ApiError::bad_request().add_message("Attendee is not invited to the group"));
        }

        if let Some(invite) = attendee.get_invite(&event_id) {
            if let Some(notification_id) = invite.notification_id {
                NotificationCalls::notification_remove_event_invite(
                    notification_id,
                    InviteAttendeeResponse::new(
                        event_id,
                        invite.group_id,
                        attendee_principal,
                        invite.invite_type,
                    ),
                );
            }
        }

        attendee.remove_invite(event_id);
        AttendeeStore::update(attendee_principal, attendee)?;

        let (_, mut attendees) = EventAttendeeStore::get(event_id)?;
        attendees.remove_invite(&attendee_principal);
        EventAttendeeStore::update(event_id, attendees)?;

        Ok(())
    }

    fn get_boosted_event(id: u64) -> Option<Boost> {
        match BoostCalls::get_boost_by_subject(Subject::Event(id)) {
            Ok((_, boosted)) => Some(boosted),
            Err(_) => None,
        }
    }

    fn get_event_caller_data(event_id: u64, group_id: u64) -> Option<EventCallerData> {
        let is_starred = ProfileStore::get(caller())
            .is_ok_and(|(_, profile)| profile.is_starred(&Subject::Event(event_id)));

        let mut joined: Option<JoinedAttendeeResponse> = None;
        let mut invite: Option<InviteAttendeeResponse> = None;
        if let Ok((_, attendee)) = AttendeeStore::get(caller()) {
            if attendee.is_event_joined(&event_id) {
                joined = Some(JoinedAttendeeResponse::new(event_id, group_id, caller()));
            };

            if attendee.is_event_invited(&event_id) {
                invite = Some(InviteAttendeeResponse::new(
                    event_id,
                    group_id,
                    caller(),
                    // Can be safely unwrapped because we checked if the user is invited
                    attendee.get_invite(&event_id).unwrap().invite_type.clone(),
                ));
            }
        }

        Some(EventCallerData::new(joined, invite, is_starred))
    }

    pub fn get_attendees_count(event_id: u64) -> u64 {
        match EventAttendeeStore::get(event_id) {
            Ok((_, member)) => member.get_member_count(),
            Err(_) => 0,
        }
    }
}
