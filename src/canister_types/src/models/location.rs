use std::fmt;

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, CandidType, Deserialize)]
pub enum Location {
    #[default]
    None,
    Physical(PhysicalLocation),
    Digital(String),
    MultiLocation(MultiLocation),
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct Address {
    pub label: String,
    pub street: String,
    pub house_number: String,
    pub house_number_addition: String,
    pub postal_code: String,
    pub state_or_province: String,
    pub city: String,
    pub country: String,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct MultiLocation {
    pub physical: PhysicalLocation,
    pub digital: String,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Location::*;
        match self {
            None => write!(f, "None"),
            Physical(value) => {
                write!(f, "CanisterStorage - {:?}", value)
            }
            Digital(value) => write!(f, "Digital - {:?}", value),
            MultiLocation(value) => write!(f, "MultiLocation - {:?}", value),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, CandidType, Deserialize)]
pub struct PhysicalLocation {
    pub longtitude: f32,
    pub lattitude: f32,
    pub address: Address,
}
