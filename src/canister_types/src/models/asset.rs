use std::fmt;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use super::storage::CanisterStorage;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub enum Asset {
    #[default]
    None,
    CanisterStorage(CanisterStorage),
    Url(String),
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Asset::*;
        match self {
            None => write!(f, "None"),
            CanisterStorage(value) => {
                write!(f, "CanisterStorage - {:?}", serde_json::to_string(value))
            }
            Url(value) => write!(f, "NotFound - {:?}", value),
        }
    }
}
