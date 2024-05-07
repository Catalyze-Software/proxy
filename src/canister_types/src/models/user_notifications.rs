use std::collections::HashMap;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(UserNotifications);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UserNotificationData {
    is_read: bool,
    is_sender: bool,
}

impl UserNotificationData {
    pub fn new(is_read: bool, is_sender: bool) -> Self {
        Self { is_read, is_sender }
    }

    pub fn mark_as_read(&mut self, is_read: bool) {
        self.is_read = is_read;
    }

    pub fn mark_as_sender(&mut self, is_sender: bool) {
        self.is_sender = is_sender;
    }

    pub fn is_read(&self) -> bool {
        self.is_read
    }

    pub fn is_sender(&self) -> bool {
        self.is_sender
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct UserNotifications(HashMap<u64, UserNotificationData>);

impl UserNotifications {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, id: u64, is_read: bool, is_sender: bool) {
        self.0
            .entry(id)
            .or_insert_with(|| UserNotificationData::new(is_read, is_sender));
    }

    pub fn remove(&mut self, id: &u64) {
        self.0.remove(id);
    }

    pub fn mark_as_read(&mut self, id: &u64, is_read: bool) {
        if let Some(data) = self.clone().0.get_mut(id) {
            data.mark_as_read(is_read);
            self.0.insert(*id, data.clone());
        }
    }

    pub fn mark_as_read_many(&mut self, ids: Vec<u64>, is_read: bool) {
        for id in ids {
            if let Some(data) = self.clone().0.get_mut(&id) {
                data.mark_as_read(is_read);
                self.0.insert(id, data.clone());
            }
        }
    }

    pub fn get_unread_ids(&self) -> Vec<u64> {
        self.0
            .clone()
            .into_iter()
            .filter(|(_, data)| !data.is_read())
            .map(|(id, _)| id)
            .collect()
    }

    pub fn remove_many(&mut self, ids: Vec<u64>) {
        for id in ids {
            self.0.remove(&id);
        }
    }

    pub fn get(&self, id: &u64) -> Option<UserNotificationData> {
        self.0.get(id).cloned()
    }

    pub fn contains(&self, id: &u64) -> bool {
        self.0.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn to_vec(&self) -> Vec<(u64, UserNotificationData)> {
        self.0.clone().into_iter().collect()
    }

    pub fn ids(&self) -> Vec<u64> {
        self.0.clone().into_keys().collect()
    }
}
