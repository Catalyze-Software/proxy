use super::{
    boost_logic::BoostCalls, event_logic::EventCalls, history_event_logic::HistoryEventLogic,
    notification_logic::NotificationCalls, profile_logic::ProfileCalls,
};
use crate::{
    helpers::{
        group_permission::has_permission,
        token_balance::{
            dip20_balance_of, dip721_balance_of, ext_balance_of, icrc_balance_of,
            legacy_dip721_balance_of,
        },
    },
    storage::{boosteds, events, groups, profiles, RewardBufferStore},
    USER_GROUP_CREATION_LIMIT,
};
use candid::Principal;
use catalyze_shared::{
    boosted::{BoostedEntry, BoostedFilter},
    general_structs::privacy::Privacy,
    group,
    misc::role_misc::{default_roles, read_only_permissions},
    models::{
        api_error::ApiError,
        boosted::Boost,
        date_range::DateRange,
        event_collection::EventCollection,
        group_with_members::{
            GroupFilter, GroupResponse, GroupSort, GroupWithMembers, GroupsCount, PostGroup,
            UpdateGroup,
        },
        history_event::GroupRoleChangeKind,
        invite_type::InviteType,
        member_collection::MemberCollection,
        neuron::{DissolveState, ListNeurons, ListNeuronsResponse},
        paged_response::PagedResponse,
        permission::{Permission, PermissionActionType, PermissionType, PostPermission},
        privacy::{GatedType, NeuronGatedRules, TokenGated},
        profile::ProfileResponse,
        relation_type::RelationType,
        role::Role,
        subject::{Subject, SubjectType},
        validation::{ValidateField, ValidationType},
    },
    old_member::{InviteMemberResponse, JoinedMemberResponse, Member, MemberInvite},
    privacy::PrivacyType,
    profile::ProfileEntry,
    time_helper::hours_to_nanoseconds,
    validator::Validator,
    CanisterResult, Filter, Sorter, StorageClient, StorageClientInsertable,
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
    ) -> CanisterResult<GroupResponse> {
        // Check if the group data is valid
        GroupValidation::validate_post_group(post_group.clone())?;

        // Check if the group name already exists
        if groups()
            .find(GroupFilter::Name(post_group.name.clone()).into())
            .await?
            .is_some()
        {
            return Err(ApiError::duplicate().add_message("Group name already exists"));
        }

        // Check if the caller has permission to create the group
        GroupValidation::validate_group_privacy(&caller(), account_identifier, &post_group).await?;

        // Get the member and add the group to the member
        let (_, member) = groups().get_member(caller()).await?;

        if member.get_owned().len() >= USER_GROUP_CREATION_LIMIT {
            return Err(ApiError::bad_request().add_message(
                format!("You can only own {} groups", USER_GROUP_CREATION_LIMIT).as_str(),
            ));
        }

        // Create and store the group
        let (new_group_id, new_group) = groups().insert(post_group.into()).await?;

        // notify the reward buffer store that the group member count has changed
        RewardBufferStore::notify_group_member_count_changed(new_group_id);

        GroupResponse::from_result(Ok((new_group_id, new_group)), None)
    }

    pub async fn get_group(id: u64) -> CanisterResult<GroupResponse> {
        GroupResponse::from_result(groups().get(id).await, Self::get_boosted_group(id).await?)
    }

    pub async fn get_group_by_name(name: String) -> CanisterResult<GroupResponse> {
        if let Some((id, _)) = groups().find(GroupFilter::Name(name).into()).await? {
            return Self::get_group(id).await;
        };

        Err(ApiError::not_found().add_message("Group not found"))
    }

    pub async fn get_groups(
        limit: usize,
        page: usize,
        filters: Vec<GroupFilter>,
        sort: GroupSort,
    ) -> CanisterResult<PagedResponse<GroupResponse>> {
        // get all the groups and filter them based on the privacy
        // exclude all InviteOnly groups that the caller is not a member of
        let filters = vec![GroupFilter::OptionallyInvited(caller())]
            .iter()
            .chain(filters.iter())
            .cloned()
            .collect::<Vec<_>>();

        let resp = groups()
            .filter_paginated(limit, page, sort, filters)
            .await?;

        let mut boosted_groups = HashMap::new();

        for (id, _) in resp.data.clone() {
            boosted_groups.insert(id, Self::get_boosted_group(id).await?);
        }

        resp.map(|(id, group)| {
            GroupResponse::new(*id, group.clone(), boosted_groups.get(id).unwrap().clone())
        })
        .into_result()
    }

    pub async fn get_boosted_groups() -> CanisterResult<Vec<GroupResponse>> {
        let mut result = vec![];

        for (_, boosted) in BoostCalls::get_boosts_by_subject(SubjectType::Group).await? {
            let id = *boosted.subject.get_id();

            let resp = GroupResponse::from_result(
                groups().get(id).await,
                Self::get_boosted_group(id).await?,
            )?;

            result.push(resp);
        }

        Ok(result)
    }

    pub async fn get_groups_count(query: Option<String>) -> CanisterResult<GroupsCount> {
        let groups = match query {
            Some(query) => groups().filter(GroupFilter::Name(query).into()).await,
            None => groups().get_all().await,
        }?;

        let user_id = caller();

        let (joined, invited) =
            groups
                .iter()
                .fold((0, 0), |(mut members, mut invited), (_, group)| {
                    if group.is_member(user_id) {
                        members += 1;
                    }

                    if group.is_invited(user_id) {
                        invited += 1;
                    }

                    (members, invited)
                });

        let new = groups
            .iter()
            .filter(|(_, group)| {
                DateRange::new(time() - hours_to_nanoseconds(24), time())
                    .is_within(group.created_on)
            })
            .count() as u64;

        let starred = ProfileCalls::get_starred_by_subject(SubjectType::Group)
            .await
            .len() as u64;

        Ok(GroupsCount {
            total: groups.len() as u64,
            joined,
            invited,
            starred,
            new,
        })
    }

    pub async fn edit_group(id: u64, update_group: UpdateGroup) -> CanisterResult<GroupResponse> {
        let (id, mut group) = groups().get(id).await?;
        group.update(update_group);

        GroupResponse::from_result(
            groups().update(id, group).await,
            Self::get_boosted_group(id).await?,
        )
    }

    pub async fn get_group_owner_and_privacy(id: u64) -> CanisterResult<(Principal, Privacy)> {
        let (_, group) = groups().get(id).await?;
        Ok((group.owner, group.privacy))
    }

    pub async fn get_groups_by_id(group_ids: Vec<u64>) -> CanisterResult<Vec<GroupResponse>> {
        let mut result = vec![];

        for (group_id, group) in groups().get_many(group_ids).await? {
            let resp =
                GroupResponse::new(group_id, group, Self::get_boosted_group(group_id).await?);

            result.push(resp);
        }

        Ok(result)
    }

    pub async fn delete_group(group_id: u64) -> CanisterResult<bool> {
        let (_, group) = groups().get(group_id).await?;

        if let Some((boost_id, _)) = boosteds()
            .find(BoostedFilter::Subject(Subject::Group(group_id)).into())
            .await?
        {
            boosteds().remove(boost_id).await?;
        }

        // remove all pinned and starred from the profiles
        let profile_list = profiles()
            .get_many(group.get_members())
            .await?
            .clone()
            .iter_mut()
            .map(|(id, profile)| {
                let subject = Subject::Group(group_id);

                if profile.is_starred(&subject) || profile.is_pinned(&subject) {
                    profile.remove_starred(&subject);
                    profile.remove_pinned(&subject);
                }

                (*id, profile.clone())
            })
            .collect::<Vec<_>>();

        profiles().update_many(profile_list).await?;
        events().remove_many(group.events).await?;
        groups().remove(group_id).await
    }

    pub async fn add_wallet_to_group(
        group_id: u64,
        wallet_canister: Principal,
        description: String,
    ) -> CanisterResult<GroupResponse> {
        let (id, mut group) = groups().get(group_id).await?;
        group.wallets.insert(wallet_canister, description);

        GroupResponse::from_result(
            groups().update(id, group).await,
            Self::get_boosted_group(id).await?,
        )
    }

    pub async fn remove_wallet_from_group(
        group_id: u64,
        wallet_canister: Principal,
    ) -> CanisterResult<GroupResponse> {
        let (id, mut group) = groups().get(group_id).await?;
        group.wallets.remove(&wallet_canister);

        GroupResponse::from_result(
            groups().update(id, group).await,
            Self::get_boosted_group(id).await?,
        )
    }

    // Was add_role
    pub async fn add_role_to_group(
        group_id: u64,
        role_name: String,
        color: String,
        index: u64,
    ) -> CanisterResult<Role> {
        let (id, mut group) = groups().get(group_id).await?;
        let role = Role::new(
            role_name,
            false,
            read_only_permissions(),
            color,
            Some(index),
        );
        group.members.roles.push(role.clone());
        groups().update(id, group).await?;
        Ok(role)
    }

    pub async fn remove_group_role(group_id: u64, role_name: String) -> CanisterResult<bool> {
        let (group_id, mut group) = groups().get(group_id).await?;

        // get the index of the role
        let index = group.members.roles.iter().position(|r| r.name == role_name);

        if index.is_none() {
            return Ok(false);
        }

        let index = index.unwrap();
        // remove the actual role from the group based on the index
        group.members.roles.remove(index);

        for (principal, join) in group.members.members.iter_mut() {
            let index = join.roles.iter().position(|r| *r == role_name);

            if let Some(index) = index {
                join.roles.remove(index);

                HistoryEventLogic::send(
                    group_id,
                    *principal,
                    join.roles.clone(),
                    GroupRoleChangeKind::Remove,
                )
                .await?;
            }
        }

        groups().update(group_id, group).await?;

        Ok(true)
    }

    pub async fn get_group_roles(group_id: u64) -> CanisterResult<Vec<Role>> {
        let (_, group) = groups().get(group_id).await?;
        Ok(group.members.roles)
    }

    pub async fn edit_role_permissions(
        group_id: u64,
        role_name: String,
        post_permissions: Vec<PostPermission>,
    ) -> CanisterResult<bool> {
        let (id, mut group) = groups().get(group_id).await?;

        // get the index of the role
        let index = group.members.roles.iter().position(|r| r.name == role_name);
        // remove the actual role from the group based on the index
        if let Some(index) = index {
            let role = group.members.roles.get_mut(index).unwrap();
            role.permissions = post_permissions.into_iter().map(Permission::from).collect();

            groups().update(id, group).await?;
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn join_group(
        group_id: u64,
        account_identifier: Option<String>,
    ) -> CanisterResult<JoinedMemberResponse> {
        let member =
            GroupValidation::validate_member_join(caller(), group_id, &account_identifier).await?;

        let (_, mut group) = groups().get(group_id).await?;

        group.add_member(caller());
        groups().update(group_id, group).await?;

        Ok(JoinedMemberResponse::new(caller(), member, group_id))
    }

    // Invite a member to the group
    pub async fn invite_to_group(
        invitee_principal: Principal,
        group_id: u64,
    ) -> CanisterResult<Member> {
        let (_, mut group) = groups().get(group_id).await?;

        // Check if the member is already in the group
        if group.is_member(invitee_principal) {
            return Err(ApiError::bad_request().add_message("Member is already in the group"));
        }

        // Check if the member is already invited to the group
        if group.is_invited(invitee_principal) {
            return Err(
                ApiError::bad_request().add_message("Member is already invited to the group")
            );
        }

        // Check if the group is invite only
        if group.privacy.privacy_type == PrivacyType::InviteOnly {
            return Err(ApiError::bad_request().add_message("Group is invite only"));
        }

        let (_, mut invitee_member) = groups().get_member(invitee_principal).await?;

        // we don't have the `invitee_member.notification_id` at this point, not sure if needed
        let invite_member_response =
            InviteMemberResponse::new(invitee_principal, invitee_member.clone(), group_id);

        let notification_id = NotificationCalls::notification_owner_join_request_group(
            invitee_principal,
            invite_member_response,
            Self::get_higher_role_members(group_id),
        )?;

        group.add_invite(
            invitee_principal,
            InviteType::OwnerRequest,
            Some(notification_id),
        );

        groups().update(group_id, group).await?;

        invitee_member.add_invite(group_id, InviteType::OwnerRequest, Some(notification_id));

        Ok(invitee_member)
    }

    pub async fn accept_or_decline_user_request_group_invite(
        principal: Principal,
        group_id: u64,
        accept: bool,
    ) -> CanisterResult<Member> {
        let (_, mut group) = groups().get(group_id).await?;

        if !Self::has_pending_join_request(group.clone(), principal) {
            return Err(
                ApiError::bad_request().add_message("Member does not have a pending join request")
            );
        }

        let invite = group.members.invites.get(&principal).unwrap();
        let invite = MemberInvite {
            invite_type: invite.invite_type.clone(),
            notification_id: invite.notification_id,
            created_at: invite.created_at,
            updated_at: invite.updated_at,
        };

        NotificationCalls::notification_user_join_request_group_accept_or_decline(
            invite,
            accept,
            group.get_members(),
            Self::get_higher_role_members(group_id),
        )?;

        if accept {
            group.convert_invite_to_member(principal);
        } else {
            group.remove_invite(principal);
        }

        groups().update(group_id, group).await?;

        // notify the reward buffer store that the group member count has changed
        RewardBufferStore::notify_group_member_count_changed(group_id);

        // Add the group to the member and set the role
        groups()
            .get_member(principal)
            .await
            .map(|(_, member)| member)
    }

    // user accepts invite to the group
    pub async fn accept_or_decline_owner_request_group_invite(
        group_id: u64,
        accept: bool,
    ) -> CanisterResult<Member> {
        let (_, mut group) = groups().get(group_id).await?;

        let principal = caller();
        // Check if the member has a pending join request for the group
        if !Self::has_pending_invite(group.clone(), principal) {
            return Err(ApiError::not_found().add_message("Member does not have a pending invite"));
        }

        let invite = group.members.invites.get(&principal).unwrap().clone();

        // Add the group to the member and set the role
        if accept {
            group.convert_invite_to_member(principal);
        } else {
            group.remove_invite(principal);
        }

        // TODO: not sure that we still need to use this type
        let invite = MemberInvite {
            invite_type: invite.invite_type,
            notification_id: invite.notification_id,
            created_at: invite.created_at,
            updated_at: invite.updated_at,
        };

        groups().update(group_id, group.clone()).await?;

        NotificationCalls::notification_owner_join_request_group_accept_or_decline(
            principal,
            invite,
            accept,
            group.get_members(),
            Self::get_higher_role_members(group_id),
        )?;

        // notify the reward buffer store that the group member count has changed
        RewardBufferStore::notify_group_member_count_changed(group_id);

        groups()
            .get_member(principal)
            .await
            .map(|(_, member)| member)
    }

    // was assign_role
    pub async fn add_group_role_to_member(
        role: String,
        member_principal: Principal,
        group_id: u64,
    ) -> CanisterResult<Member> {
        let (_, mut group) = groups().get(group_id).await?;

        let mut roles = default_roles();
        roles.append(&mut group.members.roles.clone());
        // Check if the role exists
        if !roles.iter().any(|r| r.name == role) {
            return Err(ApiError::bad_request().add_message("Role does not exist"));
        }

        let member = group.members.members.get_mut(&member_principal);
        if member.is_none() {
            return Err(ApiError::bad_request().add_message("Member is not in the group"));
        }

        let member = member.unwrap();

        member.set_role(role.clone());
        groups().update(group_id, group).await?;

        HistoryEventLogic::send(
            group_id,
            member_principal,
            vec![role],
            GroupRoleChangeKind::Replace,
        )
        .await?;

        let member = groups().get_member(member_principal).await?.1;

        NotificationCalls::notification_change_group_member_role(
            JoinedMemberResponse::new(member_principal, member.clone(), group_id),
            Self::get_higher_role_members(group_id),
        );

        Ok(member)
    }

    // was remove_member_role
    pub async fn remove_group_role_from_member(
        role: String,
        member_principal: Principal,
        group_id: u64,
    ) -> CanisterResult<Member> {
        let (_, mut group) = groups().get(group_id).await?;

        let mut roles = default_roles();
        roles.append(&mut group.members.roles.clone());

        // Check if the role exists
        if !roles.iter().any(|r| r.name == role) {
            return Err(ApiError::bad_request().add_message("Role does not exist"));
        }

        let member = group.members.members.get_mut(&member_principal);
        if member.is_none() {
            return Err(ApiError::bad_request().add_message("Member is not in the group"));
        }
        let member = member.unwrap();
        member.remove_role(role.clone());
        let roles = member.roles.clone();
        groups().update(group_id, group).await?;

        HistoryEventLogic::send(
            group_id,
            member_principal,
            roles,
            GroupRoleChangeKind::Remove,
        )
        .await?;

        groups()
            .get_member(member_principal)
            .await
            .map(|(_, member)| member)
    }

    pub async fn get_group_member(
        principal: Principal,
        group_id: u64,
    ) -> CanisterResult<JoinedMemberResponse> {
        let (_, member) = groups().get_member(principal).await?;

        // Check if the member is in the group
        if !member.is_group_joined(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is not in the group"));
        }

        Ok(JoinedMemberResponse::new(principal, member, group_id))
    }

    pub async fn get_groups_for_members(
        principals: Vec<Principal>,
    ) -> CanisterResult<Vec<JoinedMemberResponse>> {
        let members = groups().get_many_members(principals).await?;

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

        Ok(result)
    }

    pub async fn get_group_members(group_id: u64) -> CanisterResult<Vec<JoinedMemberResponse>> {
        let (_, group) = groups().get(group_id).await?;

        let result = group
            .members
            .members
            .into_iter()
            .map(|(principal, m)| JoinedMemberResponse {
                group_id,
                principal,
                roles: m.roles,
            })
            .collect();

        Ok(result)
    }

    pub async fn get_group_member_with_profile(
        principal: Principal,
        group_id: u64,
    ) -> CanisterResult<(JoinedMemberResponse, ProfileResponse)> {
        let (_, group) = groups().get(group_id).await?;

        // Check if the member is in the group
        if !group.is_member(principal) {
            return Err(ApiError::bad_request().add_message("Member is not in the group"));
        }

        let (_, profile) = profiles().get(principal).await?;
        let member = JoinedMemberResponse {
            group_id,
            principal,
            roles: group.members.members.get(&principal).unwrap().roles.clone(),
        };

        Ok((member, ProfileResponse::new(principal, profile)))
    }

    pub async fn get_group_members_with_profiles(
        group_id: u64,
    ) -> CanisterResult<Vec<(JoinedMemberResponse, ProfileResponse)>> {
        let (_, group) = groups().get(group_id).await?;
        let result = profiles()
            .get_many(group.get_members())
            .await?
            .into_iter()
            .map(|(principal, profile)| {
                let member = JoinedMemberResponse {
                    group_id,
                    principal,
                    roles: group.members.members.get(&principal).unwrap().roles.clone(),
                };

                (member, ProfileResponse::new(principal, profile))
            })
            .collect();

        Ok(result)
    }

    pub fn get_group_members_by_permission(
        group_id: u64,
        permission_type: PermissionType,
        permission_action_type: PermissionActionType,
    ) -> CanisterResult<Vec<JoinedMemberResponse>> {
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

    pub async fn get_self_member() -> CanisterResult<Member> {
        groups()
            .get_member(caller())
            .await
            .map(|(_, member)| member)
    }

    pub async fn get_self_groups() -> CanisterResult<Vec<GroupResponse>> {
        let (_, member) = groups().get_member(caller()).await?;
        let group_ids = member.get_multiple_joined().iter().map(|g| g.0).collect();
        let groups = groups().get_many(group_ids).await?;

        Ok(groups
            .into_iter()
            .map(|(id, group)| GroupResponse::new(id, group, None))
            .collect())
    }

    pub async fn get_member_roles(
        principal: Principal,
        group_id: u64,
    ) -> CanisterResult<Vec<String>> {
        let (_, member) = groups().get_member(principal).await?;
        Ok(member.get_roles(group_id))
    }

    pub fn leave_group(group_id: u64) -> CanisterResult<()> {
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

        NotificationCalls::notification_leave_group(
            member_collection.get_member_principals(),
            group_id,
        );

        member_collection.remove_member(&caller());
        GroupMemberStore::update(id, member_collection)?;

        Ok(())
    }

    pub fn remove_invite(group_id: u64) -> CanisterResult<()> {
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

    pub fn get_banned_group_members(group_id: u64) -> Vec<Principal> {
        if let Ok((_, group)) = GroupStore::get(group_id) {
            return group
                .special_members
                .into_iter()
                .filter(|m| m.1 == RelationType::Blocked.to_string())
                .map(|m| m.0)
                .collect();
        }

        Default::default()
    }

    pub fn remove_member_from_group(principal: Principal, group_id: u64) -> CanisterResult<()> {
        let (_, mut member) = MemberStore::get(principal)?;

        // Check if the member is in the group
        if !member.is_group_joined(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is not in the group"));
        }

        // Remove the group from the member
        member.remove_joined(group_id);

        MemberStore::update(principal, member.clone())?;

        let (id, mut member_collection) = GroupMemberStore::get(group_id)?;
        member_collection.remove_member(&principal);
        GroupMemberStore::update(id, member_collection)?;

        NotificationCalls::notification_remove_group_member(
            JoinedMemberResponse::new(principal, member, group_id),
            Self::get_higher_role_members(group_id),
        );

        Ok(())
    }

    pub fn remove_member_invite_from_group(
        principal: Principal,
        group_id: u64,
    ) -> CanisterResult<()> {
        let (_, mut member) = MemberStore::get(principal)?;

        // Check if the member is in the group
        if !member.is_group_invited(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is not invited to the group"));
        }

        NotificationCalls::notification_remove_group_invite(
            InviteMemberResponse::new(principal, member.clone(), group_id),
            Self::get_higher_role_members(group_id),
        );

        // Remove the group from the member
        member.remove_invite(group_id);
        let _ = MemberStore::update(principal, member.clone())?;

        let (_, mut member_collection) = GroupMemberStore::get(group_id)?;
        member_collection.remove_invite(&principal);
        GroupMemberStore::update(group_id, member_collection)?;

        Ok(())
    }

    pub fn get_group_invites(group_id: u64) -> CanisterResult<Vec<InviteMemberResponse>> {
        let (_, member_collection) = GroupMemberStore::get(group_id)?;
        let members = MemberStore::get_many(member_collection.get_invite_principals());

        let mut result: Vec<InviteMemberResponse> = vec![];

        for (principal, member) in members {
            result.push(InviteMemberResponse::new(principal, member, group_id));
        }

        Ok(result)
    }

    pub async fn get_group_invites_with_profiles(
        group_id: u64,
    ) -> CanisterResult<Vec<(InviteMemberResponse, ProfileResponse)>> {
        let (_, member_collection) = GroupMemberStore::get(group_id)?;
        let members = MemberStore::get_many(member_collection.get_invite_principals());

        let mut result: Vec<(InviteMemberResponse, ProfileResponse)> = vec![];

        let profiles = profiles()
            .get_many(member_collection.get_invite_principals())
            .await?;

        for (principal, member) in members {
            let (_, profile) = profiles
                .iter()
                .find(|(id, _)| id == &principal)
                .expect("Profile not found");

            result.push((
                InviteMemberResponse::new(principal, member, group_id),
                ProfileResponse::new(principal, profile.clone()),
            ));
        }

        Ok(result)
    }

    async fn get_boosted_group(id: u64) -> CanisterResult<Option<Boost>> {
        let boost = BoostCalls::get_boost_by_subject(Subject::Group(id))
            .await?
            .map(|x| x.1);

        Ok(boost)
    }

    async fn get_group_caller_data(group_id: u64) -> Option<GroupCallerData> {
        let profile = profiles().get(caller()).await;
        let member_resp = MemberStore::get(caller());
        Self::get_group_caller_data_sync(group_id, profile, member_resp)
    }

    fn get_group_caller_data_sync(
        group_id: u64,
        profile_resp: CanisterResult<ProfileEntry>,
        member_resp: CanisterResult<(Principal, Member)>,
    ) -> Option<GroupCallerData> {
        let is_starred = profile_resp
            .clone()
            .is_ok_and(|(_, profile)| profile.is_starred(&Subject::Group(group_id)));

        let is_pinned =
            profile_resp.is_ok_and(|(_, profile)| profile.is_pinned(&Subject::Group(group_id)));

        let mut joined: Option<JoinedMemberResponse> = None;
        let mut invite: Option<InviteMemberResponse> = None;

        if let Ok((_, member)) = member_resp {
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
        let member_count = match GroupMemberStore::get(*group_id) {
            Ok((_, member_collection)) => member_collection.get_member_count(),
            Err(_) => 0,
        };

        let event_count = match GroupEventsStore::get(*group_id) {
            Ok((_, event_collection)) => event_collection.get_events_count(),
            Err(_) => 0,
        };

        (member_count, event_count)
    }

    pub fn add_special_member_to_group(
        group_id: u64,
        principal: Principal,
        relation: RelationType,
    ) -> CanisterResult<()> {
        let (_, mut group) = GroupStore::get(group_id)?;

        group.add_special_member(principal, relation);
        GroupStore::update(group_id, group)?;
        Ok(())
    }

    pub fn remove_special_member_from_group(
        group_id: u64,
        principal: Principal,
    ) -> CanisterResult<()> {
        let (_, mut group) = GroupStore::get(group_id)?;

        group.remove_special_member_from_group(principal);
        GroupStore::update(group_id, group)?;
        Ok(())
    }

    pub fn get_higher_role_members(group_id: u64) -> Vec<Principal> {
        GroupCalls::get_group_members_by_permission(
            group_id,
            PermissionType::Invite(None),
            PermissionActionType::Write,
        )
        .unwrap_or_default()
        .iter()
        .map(|m| m.principal)
        .collect()
    }

    async fn validate_member_join(
        caller: Principal,
        group_id: u64,
        account_identifier: &Option<String>,
    ) -> CanisterResult<Member> {
        let (group_id, group) = GroupStore::get(group_id)?;
        let (_, mut member) = MemberStore::get(caller)?;

        if group.is_banned_member(caller) {
            return Err(ApiError::unauthorized().add_message("You are allowed to join this group"));
        }

        // Check if the member is already in the group
        if member.is_group_joined(&group_id) {
            return Err(ApiError::bad_request().add_message("Member is already in the group"));
        }

        let (_, mut member_collection) = GroupMemberStore::get(group_id)?;

        use Privacy::*;
        let validated_member = match group.privacy {
            // If the group is public, add the member to the group
            Public => {
                member.add_joined(group_id, vec!["member".to_string()]);
                let group_member_principals = GroupCalls::get_group_members(group_id)?
                    .iter()
                    .map(|member| member.principal)
                    .collect();

                member_collection.add_member(caller);

                NotificationCalls::notification_join_public_group(
                    group_member_principals,
                    group_id,
                );

                // notify the reward buffer store that the group member count has changed
                RewardBufferStore::notify_group_member_count_changed(group_id);

                Ok(member)
            }
            // If the group is private, add the invite to the member
            Private => {
                let notification_id = NotificationCalls::notification_user_join_request_group(
                    GroupCalls::get_higher_role_members(group_id),
                    InviteMemberResponse::new(caller, member.clone(), group_id),
                )?;

                member_collection.add_invite(caller);

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
                            member_collection.add_member(caller);

                            // notify the reward buffer store that the group member count has changed
                            RewardBufferStore::notify_group_member_count_changed(group_id);

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
                            member_collection.add_member(caller);

                            // notify the reward buffer store that the group member count has changed
                            RewardBufferStore::notify_group_member_count_changed(group_id);

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
        };

        GroupMemberStore::update(group_id, member_collection)?;

        validated_member
    }

    fn has_pending_join_request(group: GroupWithMembers, principal: Principal) -> bool {
        if let Some(invite) = group.members.invites.get(&principal) {
            return invite.invite_type == InviteType::UserRequest;
        }
        false
    }

    pub fn has_pending_invite(group: GroupWithMembers, principal: Principal) -> bool {
        if let Some(invite) = group.members.invites.get(&principal) {
            return invite.invite_type == InviteType::OwnerRequest;
        }
        false
    }
}

impl GroupValidation {
    pub fn validate_post_group(post_group: PostGroup) -> CanisterResult<()> {
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

    pub fn validate_update_group(update_group: UpdateGroup) -> CanisterResult<()> {
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
    ) -> CanisterResult<()> {
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
                                *caller,
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
                            return Ok(());
                            // If the caller does not own the neuron, throw an error
                        }

                        Err(ApiError::unauthorized().add_message(
                            "You are not owning the required neuron to join this group",
                        ))
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
                            return Ok(());
                            // If the caller does not own the neuron, throw an error
                        }

                        Err(ApiError::unauthorized()
                            .add_message("You are not owning the required NFT to join this group"))
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
}
