use candid::{decode_one, encode_one, CandidType, Principal};
use serde::{Deserialize, Serialize};

use super::{api_error::ApiError, notification::NotificationResponse};

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub enum WSMessage {
    UnreadCount(u64),
    SendNotification((Principal, NotificationResponse)),
    Notification(NotificationResponse),

    Error(ApiError),
}

impl WSMessage {
    pub fn serialize(&self) -> Vec<u8> {
        match encode_one(self) {
            Ok(value) => value,
            Err(_) => {
                // WebsocketMessage::Error(ApiError::deserialize().add_info("Serialization error"))
                vec![]
            }
        }
    }

    pub fn deserialize(data: &[u8]) -> Self {
        match decode_one(data) {
            Ok(value) => value,
            Err(_) => WSMessage::Error(ApiError::deserialize().add_info("Deserialization error")),
        }
    }
}
