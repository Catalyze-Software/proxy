use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(EventCollection);

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct EventCollection {
    pub events: Vec<u64>,
}

impl EventCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_event_ids(&self) -> Vec<u64> {
        self.events.clone()
    }

    pub fn get_events_count(&self) -> u64 {
        self.events.len() as u64
    }

    pub fn add_event(&mut self, id: u64) {
        self.events.push(id);
    }

    pub fn remove_event(&mut self, id: &u64) {
        self.events.retain(|p| p != id);
    }

    pub fn has_event(&self, id: &u64) -> bool {
        self.events.contains(id)
    }
}
