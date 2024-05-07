use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use super::{attendee::Attendee, event::Event, group::Group, member::Member, profile::Profile};

#[derive(
    CandidType, Deserialize, Serialize, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Subject {
    #[default]
    None,
    Group(u64),
    Event(u64),
    Profile(Principal),
    Member(Principal),
    Attendee(Principal),
}

impl Subject {
    pub fn get_type(&self) -> SubjectType {
        match self {
            Subject::None => SubjectType::None,
            Subject::Group(_) => SubjectType::Group,
            Subject::Event(_) => SubjectType::Event,
            Subject::Profile(_) => SubjectType::Profile,
            Subject::Member(_) => SubjectType::Member,
            Subject::Attendee(_) => SubjectType::Attendee,
        }
    }

    pub fn get_id(&self) -> &u64 {
        match self {
            Subject::Group(id) => id,
            Subject::Event(id) => id,
            _ => &0,
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubjectType {
    None,
    Group,
    Event,
    Profile,
    Member,
    Attendee,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SubjectResponse {
    None,
    Group(Option<(u64, Group)>),
    Event(Option<(u64, Event)>),
    Profile(Option<(Principal, Profile)>),
    Member(Option<(Principal, Member)>),
    Attendee(Option<(Principal, Attendee)>),
}
