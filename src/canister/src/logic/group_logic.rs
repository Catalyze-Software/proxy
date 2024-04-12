use super::{
    boost_logic::BoostCalls, notification_logic::NotificationCalls, profile_logic::ProfileCalls,
};
use crate::{
    helpers::{
        group_permission::has_permission,
        time_helper::hours_to_nanoseconds,
        token_balance::{
            dip20_balance_of, dip721_balance_of, ext_balance_of, icrc_balance_of,
            legacy_dip721_balance_of,
        },
        validator::Validator,
    },
    storage::{
        GroupEventsStore, GroupMemberStore, GroupStore, MemberStore, ProfileStore, StorageMethods,
    },
};
use candid::Principal;
use canister_types::{
    misc::role_misc::{default_roles, read_only_permissions},
    models::{
        api_error::ApiError,
        boosted::Boost,
        date_range::DateRange,
        event_collection::EventCollection,
        group::{
            Group, GroupCallerData, GroupFilter, GroupResponse, GroupSort, GroupsCount, PostGroup,
            UpdateGroup,
        },
        invite_type::InviteType,
        member::{InviteMemberResponse, JoinedMemberResponse, Member},
        member_collection::{self, MemberCollection},
        neuron::{DissolveState, ListNeurons, ListNeuronsResponse},
        paged_response::PagedResponse,
        permission::{Permission, PermissionActionType, PermissionType, PostPermission},
        privacy::{GatedType, NeuronGatedRules, Privacy, TokenGated},
        role::Role,
        subject::{Subject, SubjectType},
        validation::{ValidateField, ValidationType},
    },
};
use ic_cdk::{
    api::{call, time},
    caller,
};
use std::collections::HashMap;

pub struct GroupCalls;
pub struct GroupValidation;

impl GroupCalls {
    // TODO: add logic for nft and token gated groups
    pub async fn add_group(
        post_group: PostGroup,
        account_identifier: Option<String>,
    ) -> Result<GroupResponse, ApiError> {
        // Check if the group data is valid
        GroupValidation::validate_post_group(post_group.clone())?;

        // Check if the group name already exists
        if GroupStore::find(|_, group| group.name.to_lowercase() == post_group.name.to_lowercase())
            .is_some()
        {
            return Err(ApiError::bad_request().add_message("Group name already exists"));
        }

        // Check if the caller has permission to create the group
        GroupValidation::validate_group_privacy(&caller(), account_identifier, &post_group).await?;

        // Get the member and add the group to the member
        let (_, mut member) = MemberStore::get(caller())?;

        // Create and store the group
        let (new_group_id, new_group) = GroupStore::insert(Group::from(post_group))?;

        // generate and store an group identifier
        member.add_joined(new_group_id, vec!["owner".to_string()]);

        MemberStore::update(caller(), member)?;

        // Add member to the member collection
        let mut member_collection = MemberCollection::new();
        member_collection.add_member(caller());
        GroupMemberStore::insert_by_key(new_group_id.clone(), member_collection)?;

        // initialze the group event collection
        GroupEventsStore::insert_by_key(new_group_id.clone(), EventCollection::new())?;

        GroupResponse::from_result(
            Ok((new_group_id, new_group)),
            None,
            0,
            1, // the owner is a member
            Self::get_group_caller_data(new_group_id),
        )
    }

    pub fn get_group(id: u64) -> Result<GroupResponse, ApiError> {
        let (members_count, events_count) = Self::get_group_count_data(&id);
        GroupResponse::from_result(
            GroupStore::get(id),
            Self::get_boosted_group(id),
            events_count,
            members_count,
            Self::get_group_caller_data(id),
        )
    }

    pub fn get_groups(
        limit: usize,
        page: usize,
        filters: Vec<GroupFilter>,
        sort: GroupSort,
    ) -> Result<PagedResponse<GroupResponse>, ApiError> {
        // get all the groups and filter them based on the privacy
        // exclude all InviteOnly groups that the caller is not a member of
        let mut groups = GroupStore::filter(|group_id, group| {
            if group.privacy == Privacy::InviteOnly {
                if let Ok((_, caller_member)) = MemberStore::get(caller()) {
                    return caller_member.is_group_joined(group_id);
                }
                return false;
            } else {
                return true;
            }
        })
        .into_iter()
        .collect::<HashMap<u64, Group>>();

        for filter in filters {
            for (id, group) in &groups.clone() {
                if !filter.is_match(id, group) {
                    groups.remove(&id);
                }
            }
        }

        let sorted_groups = sort.sort(groups);
        let result: Vec<GroupResponse> = sorted_groups
            .into_iter()
            .map(|(group_id, group)| {
                let (members_count, events_count) = Self::get_group_count_data(&group_id);
                GroupResponse::new(
                    group_id,
                    group,
                    Self::get_boosted_group(group_id),
                    events_count,
                    members_count,
                    Self::get_group_caller_data(group_id),
                )
            })
            .collect();

        Ok(PagedResponse::new(page, limit, result))
    }

    pub fn get_groups_count(query: Option<String>) -> GroupsCount {
        let groups = GroupStore::filter(|_, group| match &query {
            Some(query) => group.name.to_lowercase().contains(&query.to_lowercase()),
            None => true,
        });

        let (joined, invited) = match MemberStore::get(caller()) {
            Ok((_, member)) => (member.joined.len() as u64, member.invites.len() as u64),
            Err(_) => (0, 0),
        };

        let new = groups
            .iter()
            .filter(|(_, group)| {
                DateRange::new(time() - hours_to_nanoseconds(24), time())
                    .is_within(group.created_on)
            })
            .count() as u64;

        let starred = ProfileCalls::get_starred_by_subject(SubjectType::Group).len() as u64;

        let result = GroupsCount {
            total: groups.len() as u64,
            joined,
            invited,
            starred,
            new,
        };

        return result;
    }

    pub fn edit_group(id: u64, update_group: UpdateGroup) -> Result<GroupResponse, ApiError> {
        let (id, mut group) = GroupStore::get(id)?;
        group.update(update_group);
        let (members_count, events_count) = Self::get_group_count_data(&id);

        GroupResponse::from_result(
            GroupStore::update(id, group),
            Self::get_boosted_group(id),
            events_count,
            members_count,
            Self::get_group_caller_data(id),
        )
    }

    pub fn get_group_owner_and_privacy(id: u64) -> Result<(Principal, Privacy), ApiError> {
        let (_, group) = GroupStore::get(id)?;
        Ok((group.owner, group.privacy))
    }

    pub fn get_groups_by_id(group_ids: Vec<u64>) -> Vec<GroupResponse> {
        GroupStore::get_many(group_ids)
            .into_iter()
            .map(|(group_id, group)| {
                let (members_count, events_count) = Self::get_group_count_data(&group_id);
                GroupResponse::new(
                    group_id,
                    group,
                    Self::get_boosted_group(group_id),
                    events_count,
                    members_count,
                    Self::get_group_caller_data(group_id),
                )
            })
            .collect()
    }

    // TODO: check if we need to hard delete it after a period of time
    // TODO: check if we need to remove the group from the members
    pub fn delete_group(group_id: u64) -> (bool, bool, bool) {
        (
            GroupStore::remove(group_id),
            GroupMemberStore::remove(group_id),
            GroupEventsStore::remove(group_id),
        )
    }

    pub fn add_wallet_to_group(
        group_id: u64,
        wallet_canister: Principal,
        description: String,
    ) -> Result<GroupResponse, ApiError> {
        let (id, mut group) = GroupStore::get(group_id)?;
        group.wallets.insert(wallet_canister, description);

        let (members_count, events_count) = Self::get_group_count_data(&id);

        GroupResponse::from_result(
            GroupStore::update(id, group),
            Self::get_boosted_group(id),
            events_count,
            members_count,
            Self::get_group_caller_data(id),
        )
    }

    pub fn remove_wallet_from_group(
        group_id: u64,
        wallet_canister: Principal,
    ) -> Result<GroupResponse, ApiError> {
        let (id, mut group) = GroupStore::get(group_id)?;
        group.wallets.remove(&wallet_canister);

        let (members_count, events_count) = Self::get_group_count_data(&id);

        GroupResponse::from_result(
            GroupStore::update(id, group),
            Self::get_boosted_group(id),
            events_count,
            members_count,
            Self::get_group_caller_data(id),
        )
    }

    // Was add_role
    pub fn add_role_to_group(
        group_id: u64,
        role_name: String,
        color: String,
        index: u64,
    ) -> Result<Role, ApiError> {
        let (id, mut group) = GroupStore::get(group_id)?;
        let role = Role::new(
            role_name.into(),
            false,
            read_only_permissions(),
            color,
            Some(index),
        );
        group.roles.push(role.clone());
        GroupStore::update(id, group)?;
        Ok(role)
    }

    pub fn remove_group_role(group_id: u64, role_name: String) -> Result<bool, ApiError> {
        let (group_id, mut group) = GroupStore::get(group_id)?;

        // get the index of the role
        let index = group.roles.iter().position(|r| r.name == role_name);
        // remove the actual role from the group based on the index
        if let Some(index) = index {
            group.roles.remove(index);
            GroupStore::update(group_id, group)?;

            // get all members from the group with the role
            let group_member_principals = GroupMemberStore::get(group_id)
                .map(|(_, member_collection)| member_collection.get_member_principals())
                .unwrap_or(vec![]);

            MemberStore::get_many(group_member_principals)
                .into_iter()
                .for_each(|(principal, mut member)| {
                    if member.get_roles(group_id).is_empty() {
                        member.add_group_role(&group_id, &"member".to_string());
                    }
                    member.remove_group_role(&group_id, &role_name);
                    MemberStore::update(principal, member).unwrap();
                });

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_group_roles(group_id: u64) -> Vec<Role> {
        let (_, group) = GroupStore::get(group_id).unwrap();
        group.roles
    }

    pub fn edit_role_permissions(
        group_id: u64,
        role_name: String,
        post_permissions: Vec<PostPermission>,
    ) -> Result<bool, ApiError> {
        let (id, mut group) = GroupStore::get(group_id)?;

        // get the index of the role
        let index = group.roles.iter().position(|r| r.name == role_name);
        // remove the actual role from the group based on the index
        if let Some(index) = index {
            let role = group.roles.get_mut(index).unwrap();
            role.permissions = post_permissions
                .into_iter()
                .map(|permission| Permission::from(permission))
                .collect();

            GroupStore::update(id, group)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn join_group(
        group_id: u64,
        account_identifier: Option<String>,
    ) -> Result<JoinedMemberResponse, ApiError> {
        let member =
            GroupValidation::validate_member_join(caller(), group_id, &account_identifier).await?;

        MemberStore::update(caller(), member.clone())?;

        let (id, mut member_collection) = GroupMemberStore::get(group_id)?;
        member_collection.add_member(caller());
        GroupMemberStore::update(id, member_collection)?;

        Ok(JoinedMemberResponse::new(caller(), member, group_id))
    }

    // Invite a member to the group
    pub fn invite_to_group(
        invitee_principal: Principal,
        group_id: u64,
    ) -> Result<Member, ApiError> {
        let (_, mut invitee_member) = MemberStore::get(invitee_principal)?;

        // Check if the member is already in the group
        if invitee_member.is_group_joined(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is already in the group"));
        }

        // Check if the member is already invited to the group
        if invitee_member.is_group_invited(&group_id) {
            return Err(
                ApiError::bad_request().add_message("Member is already invited to the group")
            );
        }

        let (_, group) = GroupStore::get(group_id)?;
        // Check if the group is invite only
        if group.privacy == Privacy::InviteOnly {
            return Err(ApiError::bad_request().add_message("Group is invite only"));
        }

        // Add the group to the member
        let notification_id =
            NotificationCalls::notification_owner_join_request_group(invitee_principal, group_id)?;

        invitee_member.add_invite(group_id, InviteType::OwnerRequest, Some(notification_id));
        MemberStore::update(invitee_principal, invitee_member.clone())?;

        let (id, mut member_collection) = GroupMemberStore::get(group_id)?;
        member_collection.add_invite(invitee_principal);
        GroupMemberStore::update(id, member_collection)?;

        Ok(invitee_member)
    }

    pub fn accept_or_decline_user_request_group_invite(
        principal: Principal,
        group_id: u64,
        accept: bool,
    ) -> Result<Member, ApiError> {
        let (_, mut member) = MemberStore::get(principal)?;
        let invite = member.get_invite(&group_id);

        if !member.has_pending_join_request(group_id) {
            return Err(
                ApiError::bad_request().add_message("Member does not have a pending join request")
            );
        }

        if let Some(invite) = invite {
            let higher_role_members: Vec<Principal> = GroupCalls::get_group_members_by_permission(
                group_id,
                PermissionType::Invite(None),
                PermissionActionType::Write,
            )
            .unwrap_or(vec![])
            .iter()
            .map(|m| m.principal)
            .collect();

            NotificationCalls::notification_user_join_request_group_accept_or_decline(
                higher_role_members,
                invite,
                accept,
            )?;

            if accept {
                member.turn_invite_into_joined(group_id);
                GroupMemberStore::get(group_id).map(|(_, mut member_collection)| {
                    member_collection.create_member_from_invite(principal);
                    GroupMemberStore::update(group_id, member_collection).unwrap();
                })?;
            } else {
                member.remove_invite(group_id);
                GroupMemberStore::get(group_id).map(|(_, mut member_collection)| {
                    member_collection.remove_invite(&principal);
                    GroupMemberStore::update(group_id, member_collection).unwrap();
                })?;
            }

            // Add the group to the member and set the role
            MemberStore::update(principal, member.clone())?;
        }

        Ok(member)
    }

    // user accepts invite to the group
    pub fn accept_or_decline_owner_request_group_invite(
        group_id: u64,
        accept: bool,
    ) -> Result<Member, ApiError> {
        let (_, mut member) = MemberStore::get(caller())?;

        // Check if the member has a pending join request for the group
        if !member.has_pending_group_invite(group_id) {
            return Err(
                ApiError::bad_request().add_message("Member does not have a pending invite")
            );
        }
        if let Some(invite) = member.get_invite(&group_id) {
            // Add the group to the member and set the role
            if accept {
                member.turn_invite_into_joined(group_id);
                GroupMemberStore::get(group_id).map(|(_, mut member_collection)| {
                    member_collection.create_member_from_invite(caller());
                    GroupMemberStore::update(group_id, member_collection).unwrap();
                })?;
            } else {
                member.remove_invite(group_id);
                GroupMemberStore::get(group_id).map(|(_, mut member_collection)| {
                    member_collection.remove_invite(&caller());
                    GroupMemberStore::update(group_id, member_collection).unwrap();
                })?;
            }

            NotificationCalls::notification_owner_join_request_group_accept_or_decline(
                caller(),
                invite,
                accept,
            )?;

            MemberStore::update(caller(), member.clone())?;
        }

        Ok(member)
    }

    // was assign_role
    pub fn add_group_role_to_member(
        role: String,
        member_principal: Principal,
        group_id: u64,
    ) -> Result<Member, ApiError> {
        let (_, group) = GroupStore::get(group_id)?;

        let mut roles = default_roles();
        roles.append(&mut group.roles.clone());
        // Check if the role exists
        if !roles.iter().any(|r| r.name == role) {
            return Err(ApiError::bad_request().add_message("Role does not exist"));
        }

        let (_, mut member) = MemberStore::get(member_principal)?;
        // Add the role to the member
        member.replace_roles(&group_id, vec![role]);

        MemberStore::update(member_principal, member.clone())?;

        Ok(member)
    }

    // was remove_member_role
    pub fn remove_group_role_from_member(
        role: String,
        member_principal: Principal,
        group_id: u64,
    ) -> Result<Member, ApiError> {
        let (_, group) = GroupStore::get(group_id)?;

        let mut roles = default_roles();
        roles.append(&mut group.roles.clone());

        // Check if the role exists
        if !roles.iter().any(|r| r.name == role) {
            return Err(ApiError::bad_request().add_message("Role does not exist"));
        }

        let (_, mut member) = MemberStore::get(member_principal)?;
        // Remove the role from the member
        member.remove_group_role(&group_id, &role);

        MemberStore::update(member_principal, member.clone())?;

        Ok(member)
    }

    pub fn get_group_member(
        principal: Principal,
        group_id: u64,
    ) -> Result<JoinedMemberResponse, ApiError> {
        let (_, member) = MemberStore::get(principal)?;

        // Check if the member is in the group
        if !member.is_group_joined(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is not in the group"));
        }

        Ok(JoinedMemberResponse::new(principal, member, group_id))
    }

    pub fn get_groups_for_members(principals: Vec<Principal>) -> Vec<JoinedMemberResponse> {
        let members = MemberStore::get_many(principals);

        let mut result: Vec<JoinedMemberResponse> = vec![];

        for (principal, member) in members {
            for (group_id, _) in member.get_multiple_joined() {
                result.push(JoinedMemberResponse::new(
                    principal,
                    member.clone(),
                    group_id,
                ));
            }
        }

        result
    }

    pub fn get_group_members(group_id: u64) -> Result<Vec<JoinedMemberResponse>, ApiError> {
        let (_, member_collection) = GroupMemberStore::get(group_id)?;
        let members = MemberStore::get_many(member_collection.get_member_principals());

        let mut result: Vec<JoinedMemberResponse> = vec![];

        for (principal, member) in members {
            result.push(JoinedMemberResponse::new(principal, member, group_id));
        }

        Ok(result)
    }

    pub fn get_group_members_by_permission(
        group_id: u64,
        permission_type: PermissionType,
        permission_action_type: PermissionActionType,
    ) -> Result<Vec<JoinedMemberResponse>, ApiError> {
        Ok(Self::get_group_members(group_id)?
            .into_iter()
            .filter(|m| {
                has_permission(
                    m.principal,
                    group_id,
                    &permission_type,
                    &permission_action_type,
                )
                .is_ok()
            })
            .collect())
    }

    pub fn get_self_member() -> Result<Member, ApiError> {
        let (_, member) = MemberStore::get(caller())?;
        Ok(member)
    }

    pub fn get_self_groups() -> Vec<GroupResponse> {
        match MemberStore::get(caller()) {
            Ok((_, member)) => {
                let groups = Self::get_groups_by_id(
                    member.get_multiple_joined().iter().map(|g| g.0).collect(),
                );
                groups
            }
            Err(_) => vec![],
        }
    }

    pub fn get_member_roles(principal: Principal, group_id: u64) -> Result<Vec<String>, ApiError> {
        let (_, member) = MemberStore::get(principal)?;
        Ok(member.get_roles(group_id))
    }

    pub fn leave_group(group_id: u64) -> Result<(), ApiError> {
        let (_, mut member) = MemberStore::get(caller())?;

        // Check if the member is in the group
        if !member.is_group_joined(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is not in the group"));
        }

        let (_, group) = GroupStore::get(group_id)?;

        if group.owner == caller() {
            return Err(ApiError::bad_request().add_message("Owner cannot leave the group"));
        }

        // Remove the group from the member
        member.remove_joined(group_id);

        MemberStore::update(caller(), member)?;

        let (id, mut member_collection) = GroupMemberStore::get(group_id)?;
        member_collection.remove_member(&caller());
        GroupMemberStore::update(id, member_collection)?;

        Ok(())
    }

    pub fn remove_invite(group_id: u64) -> Result<(), ApiError> {
        let (_, mut member) = MemberStore::get(caller())?;

        // Check if the member is in the group
        if !member.is_group_invited(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is not invited to the group"));
        }

        // Remove the group from the member
        member.remove_invite(group_id);

        MemberStore::update(caller(), member)?;

        let (id, mut member_collection) = GroupMemberStore::get(group_id)?;
        member_collection.remove_invite(&caller());
        GroupMemberStore::update(id, member_collection)?;

        Ok(())
    }

    pub fn remove_member_from_group(principal: Principal, group_id: u64) -> Result<(), ApiError> {
        let (_, mut member) = MemberStore::get(principal)?;

        // Check if the member is in the group
        if !member.is_group_joined(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is not in the group"));
        }

        // Remove the group from the member
        member.remove_joined(group_id);

        MemberStore::update(principal, member)?;

        let (id, mut member_collection) = GroupMemberStore::get(group_id)?;
        member_collection.remove_member(&principal);
        GroupMemberStore::update(id, member_collection)?;

        Ok(())
    }

    pub fn remove_member_invite_from_group(
        principal: Principal,
        group_id: u64,
    ) -> Result<(), ApiError> {
        let (_, mut member) = MemberStore::get(principal)?;

        // Check if the member is in the group
        if !member.is_group_invited(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is not invited to the group"));
        }

        // Remove the group from the member
        member.remove_invite(group_id);

        MemberStore::update(principal, member)?;

        let (id, mut member_collection) = GroupMemberStore::get(group_id)?;
        member_collection.remove_invite(&principal);
        GroupMemberStore::update(id, member_collection)?;

        Ok(())
    }

    pub fn get_group_invites(group_id: u64) -> Result<Vec<InviteMemberResponse>, ApiError> {
        let (_, member_collection) = GroupMemberStore::get(group_id)?;
        let members = MemberStore::get_many(member_collection.get_invite_principals());

        let mut result: Vec<InviteMemberResponse> = vec![];

        for (principal, member) in members {
            result.push(InviteMemberResponse::new(principal, member, group_id));
        }

        Ok(result)
    }

    fn get_boosted_group(id: u64) -> Option<Boost> {
        match BoostCalls::get_boost_by_subject(Subject::Group(id)) {
            Ok((_, boosted)) => Some(boosted),
            Err(_) => None,
        }
    }

    fn get_group_caller_data(group_id: u64) -> Option<GroupCallerData> {
        let is_starred = ProfileStore::get(caller())
            .is_ok_and(|(_, profile)| profile.is_starred(&Subject::Group(group_id)));

        let is_pinned = ProfileStore::get(caller())
            .is_ok_and(|(_, profile)| profile.is_pinned(&Subject::Group(group_id)));

        let mut joined: Option<JoinedMemberResponse> = None;
        let mut invite: Option<InviteMemberResponse> = None;
        if let Ok((_, member)) = MemberStore::get(caller()) {
            if member.is_group_joined(&group_id) {
                joined = Some(JoinedMemberResponse::new(
                    caller(),
                    member.clone(),
                    group_id,
                ));
            };

            if member.is_group_invited(&group_id) {
                invite = Some(InviteMemberResponse::new(caller(), member, group_id));
            }
        }

        Some(GroupCallerData::new(joined, invite, is_starred, is_pinned))
    }

    pub fn get_group_count_data(group_id: &u64) -> (u64, u64) {
        let member_count = match GroupMemberStore::get(group_id.clone()) {
            Ok((_, member_collection)) => member_collection.get_member_count(),
            Err(_) => 0,
        };

        let event_count = match GroupEventsStore::get(group_id.clone()) {
            Ok((_, event_collection)) => event_collection.get_events_count(),
            Err(_) => 0,
        };

        (member_count, event_count)
    }
}

impl GroupValidation {
    pub fn validate_post_group(post_group: PostGroup) -> Result<(), ApiError> {
        let validator_fields = vec![
            ValidateField(
                ValidationType::StringLength(post_group.name, 3, 64),
                "name".to_string(),
            ),
            ValidateField(
                ValidationType::StringLength(post_group.description, 0, 2500),
                "description".to_string(),
            ),
            ValidateField(
                ValidationType::StringLength(post_group.website, 0, 200),
                "website".to_string(),
            ),
            ValidateField(
                ValidationType::Count(post_group.tags.len(), 0, 25),
                "tags".to_string(),
            ),
        ];

        Validator::new(validator_fields).validate()
    }

    pub fn validate_update_group(update_group: UpdateGroup) -> Result<(), ApiError> {
        let validator_fields = vec![
            ValidateField(
                ValidationType::StringLength(update_group.name, 3, 64),
                "name".to_string(),
            ),
            ValidateField(
                ValidationType::StringLength(update_group.description, 0, 2500),
                "description".to_string(),
            ),
            ValidateField(
                ValidationType::StringLength(update_group.website, 0, 200),
                "website".to_string(),
            ),
            ValidateField(
                ValidationType::Count(update_group.tags.len(), 0, 25),
                "tags".to_string(),
            ),
        ];

        Validator::new(validator_fields).validate()
    }

    async fn validate_group_privacy(
        caller: &Principal,
        account_identifier: Option<String>,
        post_group: &PostGroup,
    ) -> Result<(), ApiError> {
        use Privacy::*;
        match post_group.privacy.clone() {
            Public => Ok(()),
            Private => Ok(()),
            InviteOnly => Ok(()),
            Gated(gated_type) => {
                let mut is_valid: u64 = 0;
                use GatedType::*;
                match gated_type {
                    Neuron(neuron_canisters) => {
                        for neuron_canister in neuron_canisters {
                            if Self::validate_neuron_gated(
                                caller.clone(),
                                neuron_canister.governance_canister,
                                neuron_canister.rules,
                            )
                            .await
                            {
                                is_valid += 1;
                            }
                            if is_valid >= post_group.privacy_gated_type_amount.unwrap_or_default()
                            {
                                break;
                            }
                        }
                        if is_valid >= post_group.privacy_gated_type_amount.unwrap_or_default() {
                            Ok(())
                            // If the caller does not own the neuron, throw an error
                        } else {
                            return Err(ApiError::unauthorized().add_message(
                                "You are not owning the required neuron to join this group",
                            ));
                        }
                    }
                    Token(nft_canisters) => {
                        // Loop over the canisters and check if the caller owns a specific NFT (inter-canister call)
                        for nft_canister in nft_canisters {
                            if Self::validate_nft_gated(caller, &account_identifier, &nft_canister)
                                .await
                            {
                                is_valid += 1;
                            }
                            if is_valid >= post_group.privacy_gated_type_amount.unwrap_or_default()
                            {
                                break;
                            }
                        }
                        if is_valid >= post_group.privacy_gated_type_amount.unwrap_or_default() {
                            Ok(())
                            // If the caller does not own the neuron, throw an error
                        } else {
                            return Err(ApiError::unauthorized().add_message(
                                "You are not owning the required NFT to join this group",
                            ));
                        }
                    }
                }
            }
        }
    }

    // Method to check if the caller owns a specific NFT
    pub async fn validate_nft_gated(
        principal: &Principal,
        account_identifier: &Option<String>,
        nft_canister: &TokenGated,
    ) -> bool {
        // Check if the canister is a EXT, DIP20 or DIP721 canister
        match nft_canister.standard.as_str() {
            // If the canister is a EXT canister, check if the caller owns the NFT
            // This call uses the account_identifier
            "EXT" => match account_identifier {
                Some(_account_identifier) => {
                    let response =
                        ext_balance_of(nft_canister.principal, _account_identifier.clone()).await;
                    response as u64 >= nft_canister.amount
                }
                None => false,
            },
            // If the canister is a DIP20 canister, check if the caller owns the NFT
            "DIP20" => {
                let response = dip20_balance_of(nft_canister.principal, principal).await;
                response as u64 >= nft_canister.amount
            }
            // If the canister is a DIP721 canister, check if the caller owns the NFT
            "DIP721" => {
                let response = dip721_balance_of(nft_canister.principal, principal).await;
                response as u64 >= nft_canister.amount
            }
            // If the canister is a LEGACY DIP721 canister, check if the caller owns the NFT
            "DIP721_LEGACY" => {
                let response = legacy_dip721_balance_of(nft_canister.principal, principal).await;
                response as u64 >= nft_canister.amount
            }
            // If the canister is a ICRC canister, check if the caller owns the amount of tokens
            "ICRC" => {
                let response = icrc_balance_of(nft_canister.principal, principal).await;
                response >= nft_canister.amount as u128
            }
            _ => false,
        }
    }

    // Method to check if the caller owns a specific neuron and it applies to the set rules
    pub async fn validate_neuron_gated(
        principal: Principal,
        governance_canister: Principal,
        rules: Vec<NeuronGatedRules>,
    ) -> bool {
        let list_neuron_arg = ListNeurons {
            of_principal: Some(principal),
            limit: 100,
            start_page_at: None,
        };

        let call: Result<(ListNeuronsResponse,), _> =
            call::call(governance_canister, "list_neurons", (list_neuron_arg,)).await;

        match call {
            Ok((neurons,)) => {
                let mut is_valid: HashMap<Vec<u8>, bool> = HashMap::new();
                // iterate over the neurons and check if the neuron applies to all the set rules
                for neuron in neurons.neurons {
                    let neuron_id = neuron.id.unwrap().id;
                    is_valid.insert(neuron_id.clone(), true);
                    for rule in rules.clone() {
                        use NeuronGatedRules::*;
                        match rule {
                            IsDisolving(_) => {
                                match &neuron.dissolve_state {
                                    Some(_state) => {
                                        use DissolveState::*;
                                        match _state {
                                            // neuron is not in a dissolving state
                                            DissolveDelaySeconds(_time) => {
                                                is_valid.insert(neuron_id, false);
                                                break;
                                            }
                                            // means that the neuron is in a dissolving state
                                            WhenDissolvedTimestampSeconds(_time) => {}
                                        }
                                    }
                                    None => {
                                        is_valid.insert(neuron_id, false);
                                        break;
                                    }
                                }
                            }
                            MinAge(_min_age_in_seconds) => {
                                if neuron.created_timestamp_seconds < _min_age_in_seconds {
                                    is_valid.insert(neuron_id, false);
                                    break;
                                }
                            }
                            MinStake(_min_stake) => {
                                let neuron_stake =
                                    neuron.cached_neuron_stake_e8s as f64 / 100_000_000.0;
                                let min_stake = _min_stake as f64 / 100_000_000.0;

                                if neuron_stake.ceil() < min_stake.ceil() {
                                    is_valid.insert(neuron_id, false);
                                    break;
                                }
                            }
                            MinDissolveDelay(_min_dissolve_delay_in_seconds) => {
                                match &neuron.dissolve_state {
                                    Some(_state) => {
                                        use DissolveState::*;
                                        match _state {
                                            // neuron is not in a dissolving state, time is locking period in seconds
                                            DissolveDelaySeconds(_dissolve_delay_in_seconds) => {
                                                if &_min_dissolve_delay_in_seconds
                                                    > _dissolve_delay_in_seconds
                                                {
                                                    is_valid.insert(neuron_id, false);
                                                    break;
                                                }
                                            }
                                            // if the neuron is dissolving, make invalid
                                            // means that the neuron is in a dissolving state, timestamp when neuron is done dissolving in seconds
                                            WhenDissolvedTimestampSeconds(_) => {
                                                is_valid.insert(neuron_id, false);
                                                break;
                                            }
                                        }
                                    }
                                    None => {
                                        is_valid.insert(neuron_id, false);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                return is_valid.iter().any(|v| v.1 == &true);
            }
            Err(_) => false,
        }
    }

    async fn validate_member_join(
        caller: Principal,
        group_id: u64,
        account_identifier: &Option<String>,
    ) -> Result<Member, ApiError> {
        let (group_id, group) = GroupStore::get(group_id)?;
        let (_, mut member) = MemberStore::get(caller)?;

        // Check if the member is already in the group
        if member.is_group_joined(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is already in the group"));
        }

        use Privacy::*;
        match group.privacy {
            // If the group is public, add the member to the group
            Public => {
                member.add_joined(group_id, vec!["member".to_string()]);
                let group_member_principals = GroupCalls::get_group_members(group_id)?
                    .iter()
                    .map(|member| member.principal)
                    .collect();

                NotificationCalls::notification_join_public_group(
                    group_member_principals,
                    group_id,
                );
                Ok(member)
            }
            // If the group is private, add the invite to the member
            Private => {
                let higher_role_members: Vec<Principal> =
                    GroupCalls::get_group_members_by_permission(
                        group_id,
                        PermissionType::Invite(None),
                        PermissionActionType::Write,
                    )
                    .unwrap_or(vec![])
                    .iter()
                    .map(|m| m.principal)
                    .collect();

                let notification_id = NotificationCalls::notification_user_join_request_group(
                    higher_role_members,
                    group_id,
                )?;

                member.add_invite(group_id, InviteType::UserRequest, Some(notification_id));
                Ok(member)
            }
            // If the group is invite only, throw an error
            InviteOnly => Err(ApiError::bad_request().add_message("Group is invite only")),

            // Self::validate_neuron(caller, neuron_canister.governance_canister, neuron_canister.rules).await
            // If the group is gated, check if the caller owns a specific NFT
            Gated(gated_type) => {
                let mut is_valid = false;
                use GatedType::*;
                match gated_type {
                    Neuron(neuron_canisters) => {
                        for neuron_canister in neuron_canisters {
                            is_valid = Self::validate_neuron_gated(
                                caller,
                                neuron_canister.governance_canister,
                                neuron_canister.rules,
                            )
                            .await;
                            if is_valid {
                                break;
                            }
                        }
                        if is_valid {
                            member.add_joined(group_id, vec!["member".to_string()]);
                            Ok(member)
                            // If the caller does not own the neuron, throw an error
                        } else {
                            return Err(ApiError::unauthorized().add_message(
                                "You are not owning the required neuron to join this group",
                            ));
                        }
                    }
                    Token(nft_canisters) => {
                        // Loop over the canisters and check if the caller owns a specific NFT (inter-canister call)
                        for nft_canister in nft_canisters {
                            is_valid = Self::validate_nft_gated(
                                &caller,
                                account_identifier,
                                &nft_canister,
                            )
                            .await;
                            if is_valid {
                                break;
                            }
                        }
                        if is_valid {
                            member.add_joined(group_id, vec!["member".to_string()]);
                            Ok(member)
                            // If the caller does not own the NFT, throw an error
                        } else {
                            return Err(ApiError::unauthorized().add_message(
                                "You are not owning the required NFT to join this group",
                            ));
                        }
                    }
                }
            }
        }
    }
}
