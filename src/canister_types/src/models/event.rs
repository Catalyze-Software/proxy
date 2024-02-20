use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    models::{
        asset::Asset, date_range::DateRange, location::Location, privacy::Privacy,
        sort_direction::SortDirection,
    },
};

use super::{
    api_error::ApiError,
    attendee::{InviteAttendeeResponse, JoinedAttendeeResponse},
    boosted::Boosted,
    identifier::Identifier,
};

impl_storable_for!(Event);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct Event {
    name: String,
    description: String,
    date: DateRange,
    privacy: Privacy,
    pub group_id: u64,
    created_by: Principal,
    pub owner: Principal,
    website: String,
    location: Location,
    image: Asset,
    banner_image: Asset,
    tags: Vec<u32>,
    is_canceled: (bool, String),
    is_deleted: bool,
    attendee_count: u64,
    metadata: Option<String>,
    updated_on: u64,
    created_on: u64,
}

impl Event {
    pub fn match_privacy(&self, privacy: Privacy) -> bool {
        self.privacy == privacy
    }
}

impl From<PostEvent> for Event {
    fn from(post_event: PostEvent) -> Self {
        Self {
            name: post_event.name,
            description: post_event.description,
            date: post_event.date,
            privacy: post_event.privacy,
            group_id: post_event.group_id,
            created_by: caller(),
            owner: caller(),
            website: post_event.website,
            location: post_event.location,
            image: post_event.image,
            banner_image: post_event.banner_image,
            tags: post_event.tags,
            is_canceled: (false, "".to_string()),
            is_deleted: false,
            attendee_count: Default::default(),
            metadata: post_event.metadata,
            updated_on: time(),
            created_on: time(),
        }
    }
}

impl Event {
    pub fn update(&mut self, update_event: UpdateEvent) -> Self {
        self.name = update_event.name;
        self.description = update_event.description;
        self.date = update_event.date;
        self.privacy = update_event.privacy;
        self.website = update_event.website;
        self.location = update_event.location;
        self.image = update_event.image;
        self.banner_image = update_event.banner_image;
        self.tags = update_event.tags;
        self.metadata = update_event.metadata;
        self.updated_on = time();
        self.clone()
    }

    pub fn set_owner(&mut self, owner: Principal) -> Self {
        self.owner = owner;
        self.updated_on = time();
        self.clone()
    }

    pub fn cancel(&mut self, reason: String) -> Self {
        self.is_canceled = (true, reason);
        self.updated_on = time();
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = time();
        self.clone()
    }

    pub fn is_from_group(&self, group_id: u64) -> bool {
        self.group_id == group_id
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            date: Default::default(),
            privacy: Default::default(),
            group_id: Default::default(),
            created_by: Principal::anonymous(),
            owner: Principal::anonymous(),
            website: Default::default(),
            location: Default::default(),
            image: Default::default(),
            banner_image: Default::default(),
            tags: Default::default(),
            is_canceled: Default::default(),
            is_deleted: Default::default(),
            attendee_count: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            metadata: Default::default(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PostEvent {
    name: String,
    description: String,
    date: DateRange,
    privacy: Privacy,
    website: String,
    location: Location,
    image: Asset,
    pub group_id: u64,
    banner_image: Asset,
    metadata: Option<String>,
    tags: Vec<u32>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UpdateEvent {
    pub name: String,
    pub description: String,
    pub date: DateRange,
    pub privacy: Privacy,
    pub website: String,
    pub location: Location,
    pub image: Asset,
    pub owner: Principal,
    pub banner_image: Asset,
    pub metadata: Option<String>,
    pub tags: Vec<u32>,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct EventCallerData {
    pub joined: Option<JoinedAttendeeResponse>,
    pub invite: Option<InviteAttendeeResponse>,
    pub is_starred: bool,
}

impl EventCallerData {
    pub fn new(
        joined: Option<JoinedAttendeeResponse>,
        invite: Option<InviteAttendeeResponse>,
        is_starred: bool,
    ) -> Self {
        Self {
            joined,
            invite,
            is_starred,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum EventSort {
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
    StartDate(SortDirection),
    EndDate(SortDirection),
    AttendeeCount(SortDirection),
}

impl EventSort {
    pub fn sort(&self, events: HashMap<u64, Event>) -> Vec<(u64, Event)> {
        let mut events: Vec<(u64, Event)> = events.into_iter().collect();
        match self {
            EventSort::CreatedOn(SortDirection::Asc) => {
                events.sort_by(|a, b| a.1.created_on.cmp(&b.1.created_on))
            }
            EventSort::CreatedOn(SortDirection::Desc) => {
                events.sort_by(|a, b| b.1.created_on.cmp(&a.1.created_on))
            }
            EventSort::UpdatedOn(SortDirection::Asc) => {
                events.sort_by(|a, b| a.1.updated_on.cmp(&b.1.updated_on))
            }
            EventSort::UpdatedOn(SortDirection::Desc) => {
                events.sort_by(|a, b| b.1.updated_on.cmp(&a.1.updated_on))
            }
            EventSort::StartDate(SortDirection::Asc) => {
                events.sort_by(|a, b| a.1.date.start_date().cmp(&b.1.date.start_date()))
            }
            EventSort::StartDate(SortDirection::Desc) => {
                events.sort_by(|a, b| b.1.date.start_date().cmp(&a.1.date.start_date()))
            }
            EventSort::EndDate(SortDirection::Asc) => {
                events.sort_by(|a, b| a.1.date.end_date().cmp(&b.1.date.end_date()))
            }
            EventSort::EndDate(SortDirection::Desc) => {
                events.sort_by(|a, b| b.1.date.end_date().cmp(&a.1.date.end_date()))
            }
            EventSort::AttendeeCount(SortDirection::Asc) => {
                events.sort_by(|a, b| a.1.attendee_count.cmp(&b.1.attendee_count))
            }
            EventSort::AttendeeCount(SortDirection::Desc) => {
                events.sort_by(|a, b| b.1.attendee_count.cmp(&a.1.attendee_count))
            }
        }
        events
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum EventFilter {
    None,
    Name(String),
    StartDate(DateRange),
    EndDate(DateRange),
    Owner(Principal),
    Groups(Vec<u64>),
    Ids(Vec<u64>),
    Tag(u32),
    IsCanceled(bool),
    UpdatedOn(DateRange),
    CreatedOn(DateRange),
}

impl EventFilter {
    pub fn is_match(&self, id: &u64, event: &Event) -> bool {
        match self {
            EventFilter::None => true,
            EventFilter::Name(name) => event.name.to_lowercase().contains(&name.to_lowercase()),
            EventFilter::StartDate(date) => date.is_within(event.date.start_date()),
            EventFilter::EndDate(date) => date.is_within(event.date.end_date()),
            EventFilter::Owner(owner) => *owner == event.owner,
            EventFilter::Groups(groups) => groups.contains(&event.group_id),
            EventFilter::Ids(ids) => ids.contains(&id),
            EventFilter::Tag(tag) => event.tags.contains(tag),
            EventFilter::IsCanceled(is_canceled) => event.is_canceled.0 == *is_canceled,
            EventFilter::UpdatedOn(date) => date.is_within(event.updated_on),
            EventFilter::CreatedOn(date) => date.is_within(event.created_on),
        }
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        EventFilter::None
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EventResponse {
    pub identifier: Principal,
    pub name: String,
    pub description: String,
    pub date: DateRange,
    pub privacy: Privacy,
    pub created_by: Principal,
    pub owner: Principal,
    pub website: String,
    pub location: Location,
    pub image: Asset,
    pub banner_image: Asset,
    pub attendee_count: u64,
    pub is_canceled: (bool, String),
    pub is_deleted: bool,
    pub tags: Vec<u32>,
    pub metadata: Option<String>,
    pub updated_on: u64,
    pub created_on: u64,
    pub group_id: u64,
    pub boosted: Option<Boosted>,
    pub caller_data: Option<EventCallerData>,
}

impl EventResponse {
    pub fn new(
        id: u64,
        event: Event,
        boosted: Option<Boosted>,
        caller_data: Option<EventCallerData>,
    ) -> Self {
        let identifier = Identifier::generate(super::identifier::IdentifierKind::Event(id));

        Self {
            identifier: identifier.to_principal().unwrap(),
            name: event.name,
            description: event.description,
            date: event.date,
            privacy: event.privacy,
            created_by: event.created_by,
            owner: event.owner,
            website: event.website,
            location: event.location,
            image: event.image,
            banner_image: event.banner_image,
            attendee_count: event.attendee_count,
            is_canceled: event.is_canceled,
            is_deleted: event.is_deleted,
            tags: event.tags,
            metadata: event.metadata,
            updated_on: event.updated_on,
            created_on: event.created_on,
            group_id: event.group_id,
            boosted,
            caller_data,
        }
    }

    pub fn from_result(
        id: u64,
        event: Result<Event, ApiError>,
        boosted: Option<Boosted>,
        caller_data: Option<EventCallerData>,
    ) -> Result<Self, ApiError> {
        match event {
            Err(e) => Err(e),
            Ok(event) => Ok(Self::new(id, event, boosted, caller_data)),
        }
    }
}
