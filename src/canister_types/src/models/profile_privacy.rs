use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProfilePrivacy {
    Public,
    #[default]
    Private,
}
