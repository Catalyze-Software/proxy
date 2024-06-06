use std::fmt::{Display, Formatter};

use candid::{CandidType, Deserialize};
use catalyze_shared::api_error::ApiError;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, CandidType, Deserialize, Serialize)]
pub enum TopicKind {
    Tag,
    Interest,
    Skill,
}

impl Display for TopicKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TopicKind::Tag => write!(f, "Tag"),
            TopicKind::Interest => write!(f, "Interest"),
            TopicKind::Skill => write!(f, "Skill"),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Topic {
    pub id: u64,
    pub kind: TopicKind,
    pub value: String,
}

impl From<((u64, String), TopicKind)> for Topic {
    fn from(((id, value), kind): ((u64, String), TopicKind)) -> Self {
        Self { id, kind, value }
    }
}

impl From<Topic> for Result<Topic, ApiError> {
    fn from(val: Topic) -> Self {
        Ok(val)
    }
}
