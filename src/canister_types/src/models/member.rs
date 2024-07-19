use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use serde::Serialize;

use crate::{
    impl_storable_for,
    misc::role_misc::{ADMIN_ROLE, MEMBER_ROLE, MODERATOR_ROLE, OWNER_ROLE},
};

use super::invite_type::InviteType;

pub type GroupIdentifier = Principal;

impl_storable_for!(Member);

#[derive(Clone, Default, Debug, CandidType, Deserialize, Serialize)]
pub struct Member {
    pub joined: HashMap<u64, Join>,
    pub invites: HashMap<u64, MemberInvite>,
}

impl Member {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_joined(&mut self, group_id: u64, roles: Vec<String>) {
        self.joined.insert(
            group_id,
            Join {
                roles,
                updated_at: time(),
                created_at: time(),
            },
        );
    }

    pub fn get_joined(&self, group_id: &u64) -> Option<Join> {
        self.joined.get(group_id).cloned()
    }

    pub fn get_owned(&self) -> Vec<u64> {
        self.joined
            .iter()
            .filter(|(_, v)| v.roles.contains(&OWNER_ROLE.to_string()))
            .map(|(k, _)| *k)
            .collect()
    }

    pub fn get_multiple_joined(&self) -> Vec<(u64, Join)> {
        self.joined.iter().map(|(k, v)| (*k, v.clone())).collect()
    }

    pub fn remove_joined(&mut self, group_id: u64) {
        self.joined.remove(&group_id);
    }

    pub fn add_group_role(&mut self, group_id: &u64, role: &str) {
        if let Some(group) = self.joined.get_mut(group_id) {
            group.roles.push(role.to_string());
        }
    }

    pub fn replace_roles(&mut self, group_id: &u64, roles: Vec<String>) {
        if let Some(group) = self.joined.get_mut(group_id) {
            group.roles = roles;
        }
    }

    pub fn remove_group_role(&mut self, group_id: &u64, role: &String) {
        if let Some(group) = self.joined.get_mut(group_id) {
            group.roles.retain(|r| r != role);
        }
    }

    pub fn add_invite(
        &mut self,
        group_id: u64,
        invite_type: InviteType,
        notification_id: Option<u64>,
    ) {
        self.invites.insert(
            group_id,
            MemberInvite {
                invite_type,
                notification_id,
                updated_at: time(),
                created_at: time(),
            },
        );
    }

    pub fn get_invite(&self, group_id: &u64) -> Option<MemberInvite> {
        self.invites.get(group_id).cloned()
    }

    pub fn turn_invite_into_joined(&mut self, group_id: u64) {
        self.invites.remove(&group_id);
        self.add_joined(group_id, vec![MEMBER_ROLE.to_string()]);
    }

    pub fn remove_invite(&mut self, group_id: u64) {
        self.invites.remove(&group_id);
    }

    pub fn has_group_role(&self, &group_id: &u64, role: &String) -> bool {
        if let Some(group) = self.joined.get(&group_id) {
            return group.roles.contains(role);
        }
        false
    }

    pub fn is_group_owner(&self, group_id: &u64) -> bool {
        self.has_group_role(group_id, &OWNER_ROLE.to_string())
    }

    pub fn is_group_admin(&self, group_id: &u64) -> bool {
        self.has_group_role(group_id, &ADMIN_ROLE.to_string())
    }

    pub fn is_group_moderator(&self, group_id: &u64) -> bool {
        self.has_group_role(group_id, &MODERATOR_ROLE.to_string())
    }

    pub fn is_group_member(&self, group_id: &u64) -> bool {
        self.has_group_role(group_id, &MEMBER_ROLE.to_string())
    }

    pub fn is_group_invited(&self, group_id: &u64) -> bool {
        self.invites.contains_key(group_id)
    }

    pub fn is_group_joined(&self, group_id: &u64) -> bool {
        self.joined.contains_key(group_id)
    }

    /// Check if the member has a pending join request for the group
    /// # Note
    /// Member requested to join the group

    pub fn has_pending_join_request(&self, group_id: u64) -> bool {
        if let Some(invite) = self.invites.get(&group_id) {
            return invite.invite_type == InviteType::UserRequest;
        }
        false
    }

    /// Check if the member has a pending owner request for the group
    /// # Note
    /// Member is invited by group members
    pub fn has_pending_group_invite(&self, group_id: u64) -> bool {
        if let Some(invite) = self.invites.get(&group_id) {
            return invite.invite_type == InviteType::OwnerRequest;
        }
        false
    }

    pub fn get_roles(&self, group_id: u64) -> Vec<String> {
        if let Some(group) = self.joined.get(&group_id) {
            return group.roles.clone();
        }
        vec![]
    }
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Join {
    pub roles: Vec<String>,
    pub updated_at: u64,
    pub created_at: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct MemberInvite {
    pub notification_id: Option<u64>,
    pub invite_type: InviteType,
    pub updated_at: u64,
    pub created_at: u64,
}

impl MemberInvite {
    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct JoinedMemberResponse {
    pub group_id: u64,
    pub principal: Principal,
    pub roles: Vec<String>,
}

impl JoinedMemberResponse {
    pub fn new(principal: Principal, member: Member, group_id: u64) -> Self {
        Self {
            group_id,
            principal,
            roles: member.get_roles(group_id),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct InviteMemberResponse {
    pub group_id: u64,
    pub principal: Principal,
    pub invite: Option<MemberInvite>,
}

impl InviteMemberResponse {
    pub fn new(principal: Principal, member: Member, group_id: u64) -> Self {
        Self {
            group_id,
            principal,
            invite: member.get_invite(&group_id),
        }
    }
}
