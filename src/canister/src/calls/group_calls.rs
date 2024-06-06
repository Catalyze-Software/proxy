/// # Group methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Group) role based authentication

/// # Questions
/// * Should the `get_groups` / `get_groups_by_id` give back a more summier response per group compared to the `get_group` function?
/// And what about the public / private access of these calls?\
///
use candid::Principal;
use catalyze_shared::api_error::ApiError;
use ic_cdk::{query, update};

use canister_types::models::{
    group::{GroupFilter, GroupResponse, GroupsCount, GroupSort, PostGroup, UpdateGroup},
    member::{InviteMemberResponse, JoinedMemberResponse, Member},
    paged_response::PagedResponse,
    permission::{PermissionType, PostPermission},
    profile::ProfileResponse,
    relation_type::RelationType,
    role::Role,
};

use crate::{
    helpers::{
        group_permission::{can_delete, can_edit, can_read},
        guards::has_access,
    },
    logic::group_logic::GroupCalls,
};

/// Add a group to the canister  - [`[update]`](update)
/// # Arguments
/// * `post_group` - The group to add
/// * `account_identifier` - Optional account identifier needed in case when the group is Gated
/// # Returns
/// * `GroupResponse` - The added group
/// # Errors
/// * `ApiError` - If something went wrong while adding the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub async fn add_group(
    post_group: PostGroup,
    account_identifier: Option<String>,
) -> Result<GroupResponse, ApiError> {
    GroupCalls::add_group(post_group, account_identifier).await
}

/// Get a group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `GroupResponse` - The group
/// # Errors
/// * `ApiError` - If something went wrong while getting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query]
pub fn get_group(group_id: u64) -> Result<GroupResponse, ApiError> {
    GroupCalls::get_group(group_id)
}

/// Get a group by name - [`[query]`](query)
/// # Arguments
/// * `name` - The name of the group
/// # Returns
/// * `GroupResponse` - The group
/// # Errors
/// * `ApiError` - If something went wrong while getting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query]
pub fn get_group_by_name(name: String) -> Result<GroupResponse, ApiError> {
    GroupCalls::get_group_by_name(name)
}

/// Get groups - [`[query]`](query)
/// # Arguments
/// * `limit` - The maximum number of groups to return per page
/// * `page` - The page number
/// * `sort` - The sort direction
/// * `filters` - The filters to apply
/// # Returns
/// * `PagedResponse<GroupResponse>` - The groups
/// # Errors
/// * `ApiError` - If something went wrong while getting the groups
#[query]
pub fn get_groups(
    limit: usize,
    page: usize,
    filters: Vec<GroupFilter>,
    sort: GroupSort,
) -> Result<PagedResponse<GroupResponse>, ApiError> {
    GroupCalls::get_groups(limit, page, filters, sort)
}

/// Get group counts - [`[query]`](query)
/// # Returns
/// * `GroupsCount` - The groups count
#[query]
pub fn get_groups_count(query: Option<String>) -> GroupsCount {
    GroupCalls::get_groups_count(query)
}

/// Edit a group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `update_group` - The group to update
/// # Returns
/// * `GroupResponse` - The updated group
/// # Errors
/// * `ApiError` - If something went wrong while updating the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn edit_group(group_id: u64, update_group: UpdateGroup) -> Result<GroupResponse, ApiError> {
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::edit_group(group_id, update_group)
}

/// Get groups by their identifiers - [`[query]`](query)
/// # Arguments
/// * `group_ids` - The identifiers of the groups
/// # Returns
/// * `Vec<GroupResponse>` - The groups
/// # Errors
/// * `ApiError` - If something went wrong while getting the groups
#[query(guard = "has_access")]
pub fn get_groups_by_id(group_ids: Vec<u64>) -> Vec<GroupResponse> {
    GroupCalls::get_groups_by_id(group_ids)
}

/// Soft deletes a group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `GroupResponse` - The deleted group
/// # Errors
/// * `ApiError` - If something went wrong while deleting the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn delete_group(group_id: u64) -> Result<(bool, bool, bool), ApiError> {
    can_delete(group_id, PermissionType::Group(None))?;
    Ok(GroupCalls::delete_group(group_id))
}

/// Add a wallet reference to the group - [`[update]`](update)
/// # Change
/// * was `add_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `wallet_canister` - The wallet canister principal
/// * `description` - The description of the wallet
/// # Returns
/// * `GroupResponse` - The updated group
/// # Errors
/// * `ApiError` - If something went wrong while adding the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_wallet_to_group(
    group_id: u64,
    wallet_canister: Principal,
    description: String,
) -> Result<GroupResponse, ApiError> {
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::add_wallet_to_group(group_id, wallet_canister, description)
}

/// Remove a wallet reference from the group - [`[update]`](update)
/// # Change
/// * was `remove_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `wallet_canister` - The wallet canister principal
/// # Returns
/// * `GroupResponse` - The updated group
/// # Errors
/// * `ApiError` - If something went wrong while removing the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_wallet_from_group(
    group_id: u64,
    wallet_canister: Principal,
) -> Result<GroupResponse, ApiError> {
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::remove_wallet_from_group(group_id, wallet_canister)
}

/// Add a role to the group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `role_name` - The name of the role
/// * `color` - The color of the role
/// * `index` - The index of the role
/// # Returns
/// * `Role` - The added role
/// # Errors
/// * `ApiError` - If something went wrong while adding the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// Was `add_role`
#[update(guard = "has_access")]
pub fn add_role_to_group(
    group_id: u64,
    role_name: String,
    color: String,
    index: u64,
) -> Result<Role, ApiError> {
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::add_role_to_group(group_id, role_name, color, index)
}

/// Remove a role from the group - [`[update]`](update)
/// # Change
/// * was `remove_role` but interferes with the `remove_role` function in the member methods
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `role_name` - The name of the role
/// # Returns
/// * `bool` - Whether the role was removed
/// # Errors
/// * `ApiError` - If something went wrong while removing the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_group_role(group_id: u64, role_name: String) -> Result<bool, ApiError> {
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::remove_group_role(group_id, role_name)
}

/// Get the roles of the group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<Role>` - The roles of the group
/// # Note
/// Default unmutable roles are always returned on top of the custom group specific roles.
/// /// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn get_group_roles(group_id: u64) -> Result<Vec<Role>, ApiError> {
    can_read(group_id, PermissionType::Group(None))?;
    Ok(GroupCalls::get_group_roles(group_id))
}

/// Edit role permissions for a group role - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `role_name` - The name of the role
/// * `post_permissions` - The permissions to update
/// # Returns
/// * `bool` - Whether the permissions were updated
/// # Errors
/// * `ApiError` - If something went wrong while updating the permissions
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn edit_role_permissions(
    group_id: u64,
    role_name: String,
    post_permissions: Vec<PostPermission>,
) -> Result<bool, ApiError> {
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::edit_role_permissions(group_id, role_name, post_permissions)
}

/// Join a group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to join
/// * `account_identifier` - Optional account identifier of the user in case the group is Gated
/// # Returns
/// * `JoinedMemberResponse` - The joined member details
/// # Errors
/// * `ApiError` - If something went wrong while joining the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub async fn join_group(
    group_id: u64,
    account_identifier: Option<String>,
) -> Result<JoinedMemberResponse, ApiError> {
    GroupCalls::join_group(group_id, account_identifier).await
}

/// Invite a user to a group - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to invite the user to
/// * `member_principal` - The principal of the user to invite
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while inviting the user
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn invite_to_group(group_id: u64, member_principal: Principal) -> Result<Member, ApiError> {
    can_edit(group_id, PermissionType::Invite(None))?;
    GroupCalls::invite_to_group(member_principal, group_id)
}

/// Accept an invite to a group as a admin - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to accept the invite for
/// * `member_principal` - The principal of the user to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while accepting the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn accept_user_request_group_invite(
    group_id: u64,
    member_principal: Principal,
) -> Result<Member, ApiError> {
    can_edit(group_id, PermissionType::Invite(None))?;
    GroupCalls::accept_or_decline_user_request_group_invite(member_principal, group_id, true)
}

/// Decline an invite to a group as a admin - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to accept the invite for
/// * `member_principal` - The principal of the user to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while declining the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn decline_user_request_group_invite(
    group_id: u64,
    member_principal: Principal,
) -> Result<Member, ApiError> {
    can_edit(group_id, PermissionType::Invite(None))?;
    GroupCalls::accept_or_decline_user_request_group_invite(member_principal, group_id, false)
}

/// Accept an invite from a group as a user - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while accepting the invite
#[update(guard = "has_access")]
pub fn accept_owner_request_group_invite(group_id: u64) -> Result<Member, ApiError> {
    GroupCalls::accept_or_decline_owner_request_group_invite(group_id, true)
}

/// Decline an invite from a group as a user - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to accept the invite for
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while declining the invite
#[update(guard = "has_access")]
pub fn decline_owner_request_group_invite(group_id: u64) -> Result<Member, ApiError> {
    GroupCalls::accept_or_decline_owner_request_group_invite(group_id, false)
}

/// Assign a role to a specific group member - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `role` - The role to assign
/// * `member_identifier` - The identifier of the group member
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while assigning the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn assign_role(
    group_id: u64,
    role: String,
    member_principal: Principal,
) -> Result<Member, ApiError> {
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::add_group_role_to_member(role, member_principal, group_id)
}

/// Remove a role from a specific group member - [`[update]`](update)
/// # Change
/// * was `remove_role` but interferes with the `remove_role` function in the group methods
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `role` - The role to remove
/// * `member_identifier` - The identifier of the group member
/// # Returns
/// * `Member` - The updated member entry
/// # Errors
/// * `ApiError` - If something went wrong while removing the role
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn remove_member_role(
    group_id: u64,
    role: String,
    member_principal: Principal,
) -> Result<Member, ApiError> {
    can_edit(group_id, PermissionType::Group(None))?;
    GroupCalls::remove_group_role_from_member(role, member_principal, group_id)
}

/// Get the member entry of a specific group member - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `member_principal` - The principal of the group member
/// # Returns
/// * `JoinedMemberResponse` - The member entry
/// # Errors
/// * `ApiError` - If something went wrong while getting the member entry
#[query]
pub fn get_group_member(
    group_id: u64,
    member_principal: Principal,
) -> Result<JoinedMemberResponse, ApiError> {
    // can_read(group_id, PermissionType::Group(None))?;
    GroupCalls::get_group_member(member_principal, group_id)
}

/// Get the member entry and profile of a specific group member - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `member_principal` - The principal of the group member
/// # Returns
/// * `(JoinedMemberResponse, ProfileResponse)` - The member entry and profile
/// # Errors
/// * `ApiError` - If something went wrong while getting the member entry
#[query]
pub fn get_group_member_with_profile(
    group_id: u64,
    member_principal: Principal,
) -> Result<(JoinedMemberResponse, ProfileResponse), ApiError> {
    GroupCalls::get_group_member_with_profile(member_principal, group_id)
}

/// Get the groups for specific members - [`[query]`](query)
/// # Arguments
/// * `member_principals` - The principals of the members
/// # Returns
/// * `Vec<JoinedMemberResponse>` - The groups per member
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_groups_for_members(member_principals: Vec<Principal>) -> Vec<JoinedMemberResponse> {
    GroupCalls::get_groups_for_members(member_principals)
}

/// Get the group members for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<JoinedMemberResponse>` - The group members
/// # Errors
/// * `ApiError` - If something went wrong while getting the group members
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_group_members(group_id: u64) -> Result<Vec<JoinedMemberResponse>, ApiError> {
    // can_read(group_id, PermissionType::Group(None))?;
    GroupCalls::get_group_members(group_id)
}

/// Get the group members for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<(JoinedMemberResponse, ProfileResponse)` - The group members with profiles
/// # Errors
/// * `ApiError` - If something went wrong while getting the group members
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_group_members_with_profiles(
    group_id: u64,
) -> Result<Vec<(JoinedMemberResponse, ProfileResponse)>, ApiError> {
    GroupCalls::get_group_members_with_profiles(group_id)
}

/// Get the caller member entry - [`[query]`](query)
/// # Change
/// * was `get_self_member` but due to conflict with other methods it was renamed
/// # Returns
/// * `Member` - The member entry
/// # Errors
/// * `ApiError` - If something went wrong while getting the member entry
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_self_member() -> Result<Member, ApiError> {
    GroupCalls::get_self_member()
}
/// Get the caller joined groups - [`[query]`](query)
/// # Returns
/// * `Vec<GroupResponse>` - All groups the user is part of
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_self_groups() -> Vec<GroupResponse> {
    GroupCalls::get_self_groups()
}

/// Get the roles of a specific group member - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// * `member_principal` - The principal of the group member
/// # Returns
/// * `Vec<String>` - The roles of the group member
/// # Errors
/// * `ApiError` - If something went wrong while getting the roles
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query]
pub fn get_member_roles(
    group_id: u64,
    member_principal: Principal,
) -> Result<Vec<String>, ApiError> {
    // can_read(group_id, PermissionType::Group(None))?;
    GroupCalls::get_member_roles(member_principal, group_id)
}

/// Leave a group as a caller - [`[update]`](update)
/// # Arguments
/// * `group_id` - The identifier of the group to leave
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while leaving the group
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn leave_group(group_id: u64) -> Result<(), ApiError> {
    GroupCalls::leave_group(group_id)
}

/// Remove an invite for a group as a user
/// # Arguments
/// * `group_id` - The identifier of the group to remove the invite for
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_invite(group_id: u64) -> Result<(), ApiError> {
    GroupCalls::remove_invite(group_id)
}

/// Remove a member from a group
/// # Arguments
/// * `group_id` - The identifier of the group to remove the member from
/// * `principal` - The principal of the member to remove
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the member
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[update(guard = "has_access")]
pub fn remove_member_from_group(group_id: u64, principal: Principal) -> Result<(), ApiError> {
    can_delete(group_id, PermissionType::Member(None))?;
    GroupCalls::remove_member_from_group(principal, group_id)
}

/// Remove a member invite from a group as an admin
/// # Arguments
/// * `group_id` - The identifier of the group to remove the invite from
/// * `principal` - The principal of the member to remove the invite for
/// # Returns
/// * `()` - Empty tuple
/// # Errors
/// * `ApiError` - If something went wrong while removing the invite
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_member_invite_from_group(
    group_id: u64,
    principal: Principal,
) -> Result<(), ApiError> {
    can_delete(group_id, PermissionType::Invite(None))?;
    GroupCalls::remove_member_invite_from_group(principal, group_id)
}

/// Get the group invites for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<InviteMemberResponse>` - The group invites
/// # Errors
/// * `ApiError` - If something went wrong while getting the group invites
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[query(guard = "has_access")]
pub fn get_group_invites(group_id: u64) -> Result<Vec<InviteMemberResponse>, ApiError> {
    can_read(group_id, PermissionType::Invite(None))?;
    GroupCalls::get_group_invites(group_id)
}

/// Get the group invites with profiles for a specific group - [`[query]`](query)
/// # Arguments
/// * `group_id` - The identifier of the group
/// # Returns
/// * `Vec<(InviteMemberResponse, ProfileResponse)>` - The group invites
/// # Errors
/// * `ApiError` - If something went wrong while getting the group invites
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: This action is guarded by group role based authorization
#[query(guard = "has_access")]
pub fn get_group_invites_with_profiles(
    group_id: u64,
) -> Result<Vec<(InviteMemberResponse, ProfileResponse)>, ApiError> {
    can_read(group_id, PermissionType::Invite(None))?;
    GroupCalls::get_group_invites_with_profiles(group_id)
}

#[query(guard = "has_access")]
pub fn get_banned_group_members(group_id: u64) -> Result<Vec<Principal>, ApiError> {
    can_edit(group_id, PermissionType::Member(None))?;
    Ok(GroupCalls::get_banned_group_members(group_id))
}

#[update(guard = "has_access")]
pub fn ban_group_member(group_id: u64, member_principal: Principal) -> Result<(), ApiError> {
    can_edit(group_id, PermissionType::Member(None))?;
    GroupCalls::remove_member_from_group(member_principal, group_id)?;
    GroupCalls::add_special_member_to_group(group_id, member_principal, RelationType::Blocked)
}

#[update(guard = "has_access")]
pub fn remove_ban_from_group_member(
    group_id: u64,
    member_principal: Principal,
) -> Result<(), ApiError> {
    can_edit(group_id, PermissionType::Member(None))?;
    GroupCalls::remove_special_member_from_group(group_id, member_principal)
}
