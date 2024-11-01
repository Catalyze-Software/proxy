use candid::{decode_one, encode_one, CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct NotificationData {
    pub canister_id: Principal,
    pub group_identifier: Principal,
    pub created_by: Principal,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Storable for NotificationData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct AppMessageSend {
    pub text: String,
    pub timestamp: u64,
    pub receiver: Principal,
}

impl AppMessageSend {
    #[allow(unused)]
    pub fn to_receive(&self) -> AppMessageReceive {
        AppMessageReceive {
            text: self.text.clone(),
            timestamp: self.timestamp,
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum MessageType {
    Receive(AppMessageReceive),
    Send(AppMessageSend),
}

impl MessageType {
    #[allow(unused)]
    pub fn serialize(&self) -> Vec<u8> {
        encode_one(self).unwrap()
    }

    #[allow(unused)]
    pub fn deserialize(data: &[u8]) -> Self {
        decode_one(data).unwrap()
    }
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct AppMessageReceive {
    pub text: String,
    pub timestamp: u64,
}
