use std::fmt;

use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    #[default]
    Desc,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SortDirection::*;
        match self {
            Asc => write!(f, "Asc"),
            Desc => write!(f, "Desc"),
        }
    }
}
