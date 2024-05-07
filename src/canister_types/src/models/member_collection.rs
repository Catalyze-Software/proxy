use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(MemberCollection);

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct MemberCollection {
    members: Vec<Principal>,
    invites: Vec<Principal>,
}

impl MemberCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_member_principals(&self) -> Vec<Principal> {
        self.members.clone()
    }

    pub fn get_invite_principals(&self) -> Vec<Principal> {
        self.invites.clone()
    }

    pub fn get_member_count(&self) -> u64 {
        self.members.len() as u64
    }

    pub fn get_invite_count(&self) -> u64 {
        self.invites.len() as u64
    }

    pub fn add_member(&mut self, principal: Principal) {
        if !self.members.contains(&principal) {
            self.members.push(principal);
        }
    }

    pub fn remove_member(&mut self, principal: &Principal) {
        self.members.retain(|p| p != principal);
    }

    pub fn add_invite(&mut self, principal: Principal) {
        if !self.invites.contains(&principal) {
            self.invites.push(principal);
        }
    }

    pub fn is_member(&self, principal: &Principal) -> bool {
        self.members.contains(principal)
    }

    pub fn is_invited(&self, principal: &Principal) -> bool {
        self.invites.contains(principal)
    }

    pub fn remove_invite(&mut self, principal: &Principal) {
        self.invites.retain(|p| p != principal);
    }

    pub fn create_member_from_invite(&mut self, principal: Principal) {
        self.remove_invite(&principal);
        self.add_member(principal);
    }
}
