use std::fmt;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::date_range::DateRange;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct Length {
    pub min: usize,
    pub max: usize,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub field: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct ValidateField(pub ValidationType, pub String);

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub enum ValidationType {
    #[default]
    None,
    StringLength(String, usize, usize),
    DateRange(DateRange),
    Email(String),
    Count(usize, usize, usize),
}

impl fmt::Display for ValidationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ValidationType::*;
        match self {
            None => write!(f, "None"),
            StringLength(value, min, max) => {
                write!(
                    f,
                    "StringLength - value: {}, min: {}, max: {}",
                    value, min, max
                )
            }
            DateRange(value) => write!(f, "DateRange - {:?}", value),
            Email(value) => write!(f, "Email - {:?}", value),
            Count(value, min, max) => {
                write!(f, "Count - value: {}, min: {}, max: {}", value, min, max)
            }
        }
    }
}
