use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use catalyze_shared::date_range::DateRange;
use ic_stable_structures::{Storable, storable::Bound};
use serde::Serialize;

use canister_types::models::{
    asset::Asset, location::Location, privacy::Privacy, sort_direction::SortDirection,
};

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub date: DateRange,
    pub privacy: Privacy,
    pub group_identifier: Principal,
    pub created_by: Principal,
    pub owner: Principal,
    pub website: String,
    #[serde(default)]
    pub location: Location,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
    pub is_canceled: (bool, String),
    pub is_deleted: bool,
    pub attendee_count: HashMap<Principal, usize>,
    #[serde(default)]
    pub metadata: Option<String>,
    pub updated_on: u64,
    pub created_on: u64,
}

impl Storable for Event {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for Event {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            date: Default::default(),
            privacy: Default::default(),
            group_identifier: Principal::anonymous(),
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

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum EventSort {
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
    StartDate(SortDirection),
    EndDate(SortDirection),
    AttendeeCount(SortDirection),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum EventFilter {
    Name(String),
    StartDate(DateRange),
    EndDate(DateRange),
    Owner(Principal),
    Identifiers(Vec<Principal>),
    Tag(u32),
    IsCanceled(bool),
    UpdatedOn(DateRange),
    CreatedOn(DateRange),
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
    pub attendee_count: usize,
    pub is_canceled: (bool, String),
    pub is_deleted: bool,
    pub tags: Vec<u32>,
    pub metadata: Option<String>,
    pub updated_on: u64,
    pub created_on: u64,
    pub group_identifier: Principal,
}
