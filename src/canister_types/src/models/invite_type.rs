use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum InviteType {
    OwnerRequest,
    #[default]
    UserRequest,
}
