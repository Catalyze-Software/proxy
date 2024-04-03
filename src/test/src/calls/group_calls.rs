#![allow(unused)]
use crate::{ENV, SENDER};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    filter_type::FilterType,
    group::{GroupFilter, GroupResponse, GroupSort, PostGroup, UpdateGroup},
    member::{InviteMemberResponse, JoinedMemberResponse, Member},
    paged_response::PagedResponse,
    permission::PostPermission,
    role::Role,
    sort_direction::SortDirection,
};
use pocket_ic::{query_candid_as, update_candid_as};

pub fn add_group(post_group: PostGroup, account_identifier: Option<String>) -> GroupResponse {
    let group_response: GroupResponse =
        update_candid_as::<(PostGroup, Option<String>), (Result<GroupResponse, ApiError>,)>(
            &ENV.pic,
            ENV.canister_id,
            SENDER.with(|s| s.borrow().unwrap()),
            "add_group",
            (post_group, account_identifier),
        )
        .expect("Failed to call add_group from pocketIC")
        .0
        .expect("Failed to add group");

    group_response
}

pub fn get_group(group_id: u64) -> Result<GroupResponse, ApiError> {
    query_candid_as::<(u64,), (Result<GroupResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_group",
        (group_id,),
    )
    .expect("Failed to call get_group from pocketIC")
    .0
}

pub fn get_groups() -> PagedResponse<GroupResponse> {
    let sort_direction: SortDirection = SortDirection::Asc;

    let paged_response: PagedResponse<GroupResponse> = query_candid_as::<
        (usize, usize, Vec<FilterType<GroupFilter>>, GroupSort),
        (Result<PagedResponse<GroupResponse>, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_groups",
        (10, 1, vec![], GroupSort::Name(sort_direction)),
    )
    .expect("Failed to call get_groups from pocketIC")
    .0
    .expect("Failed to get groups");

    paged_response
}

pub fn edit_group(group_id: u64, update_group: UpdateGroup) -> Result<GroupResponse, ApiError> {
    update_candid_as::<(u64, UpdateGroup), (Result<GroupResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "edit_group",
        (group_id, update_group),
    )
    .expect("Failed to call edit_group from pocketIC")
    .0
}

// deprecated
// pub fn get_group_owner_and_privacy(
//     _group_identifier: Principal,
// ) -> Result<(Principal, Privacy), ApiError>

pub fn get_groups_by_id(group_ids: Vec<u64>) -> Vec<GroupResponse> {
    query_candid_as::<(Vec<u64>,), (Vec<GroupResponse>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_groups_by_id",
        (group_ids,),
    )
    .expect("Failed to call get_groups_by_id from pocketIC")
    .0
}

pub fn delete_group(group_id: u64) -> Result<GroupResponse, ApiError> {
    update_candid_as::<(u64,), (Result<GroupResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "delete_group",
        (group_id,),
    )
    .expect("Failed to call delete_group from pocketIC")
    .0
}

pub fn add_wallet_to_group(
    group_id: u64,
    wallet_canister: Principal,
    description: String,
) -> Result<GroupResponse, ApiError> {
    update_candid_as::<(u64, Principal, String), (Result<GroupResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "add_wallet_to_group",
        (group_id, wallet_canister, description),
    )
    .expect("Failed to call add_wallet_to_group from pocketIC")
    .0
}

pub fn remove_wallet_from_group(
    group_id: u64,
    wallet_canister: Principal,
) -> Result<GroupResponse, ApiError> {
    update_candid_as::<(u64, Principal), (Result<GroupResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_wallet_from_group",
        (group_id, wallet_canister),
    )
    .expect("Failed to call remove_wallet_from_group from pocketIC")
    .0
}

pub fn add_role_to_group(
    group_id: u64,
    role_name: String,
    color: String,
    index: u64,
) -> Result<Role, ApiError> {
    update_candid_as::<(u64, String, String, u64), (Result<Role, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "add_role_to_group",
        (group_id, role_name, color, index),
    )
    .expect("Failed to call add_role_to_group from pocketIC")
    .0
}

pub fn remove_group_role(group_id: u64, role_name: String) -> Result<bool, ApiError> {
    update_candid_as::<(u64, String), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_group_role",
        (group_id, role_name),
    )
    .expect("Failed to call remove_group_role from pocketIC")
    .0
}

pub fn get_group_roles(group_id: u64) -> Result<Vec<Role>, ApiError> {
    update_candid_as::<(u64,), (Result<Vec<Role>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_group_roles",
        (group_id,),
    )
    .expect("Failed to call get_group_roles from pocketIC")
    .0
}

pub fn edit_role_permissions(
    group_id: u64,
    role_name: String,
    post_permissions: Vec<PostPermission>,
) -> Result<bool, ApiError> {
    update_candid_as::<(u64, String, Vec<PostPermission>), (Result<bool, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "edit_role_permissions",
        (group_id, role_name, post_permissions),
    )
    .expect("Failed to call edit_role_permissions from pocketIC")
    .0
}

pub fn join_group(
    group_id: u64,
    account_identifier: Option<String>,
) -> Result<JoinedMemberResponse, ApiError> {
    update_candid_as::<(u64, Option<String>), (Result<JoinedMemberResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "join_group",
        (group_id, account_identifier),
    )
    .expect("Failed to call join_group from pocketIC")
    .0
}

pub fn invite_to_group(group_id: u64, member_principal: Principal) -> Result<Member, ApiError> {
    update_candid_as::<(u64, Principal), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "invite_to_group",
        (group_id, member_principal),
    )
    .expect("Failed to call invite_to_group from pocketIC")
    .0
}

pub fn accept_user_request_group_invite(
    group_id: u64,
    member_principal: Principal,
) -> Result<Member, ApiError> {
    update_candid_as::<(u64, Principal), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "accept_user_request_group_invite",
        (group_id, member_principal),
    )
    .expect("Failed to call accept_user_request_group_invite from pocketIC")
    .0
}

pub fn decline_user_request_group_invite(
    group_id: u64,
    member_principal: Principal,
) -> Result<Member, ApiError> {
    update_candid_as::<(u64, Principal), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "decline_user_request_group_invite",
        (group_id, member_principal),
    )
    .expect("Failed to call decline_user_request_group_invite from pocketIC")
    .0
}

pub fn accept_owner_request_group_invite(group_id: u64) -> Result<Member, ApiError> {
    update_candid_as::<(u64,), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "accept_owner_request_group_invite",
        (group_id,),
    )
    .expect("Failed to call accept_owner_request_group_invite from pocketIC")
    .0
}

pub fn decline_owner_request_group_invite(group_id: u64) -> Result<Member, ApiError> {
    update_candid_as::<(u64,), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "decline_owner_request_group_invite",
        (group_id,),
    )
    .expect("Failed to call decline_owner_request_group_invite from pocketIC")
    .0
}

pub fn assign_role(
    group_id: u64,
    role: String,
    member_identifier: Principal,
) -> Result<Member, ApiError> {
    update_candid_as::<(u64, String, Principal), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "assign_role",
        (group_id, role, member_identifier),
    )
    .expect("Failed to call assign_role from pocketIC")
    .0
}

pub fn remove_member_role(
    group_id: u64,
    role: String,
    member_identifier: Principal,
) -> Result<Member, ApiError> {
    update_candid_as::<(u64, String, Principal), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_member_role",
        (group_id, role, member_identifier),
    )
    .expect("Failed to call remove_member_role from pocketIC")
    .0
}

pub fn get_group_member(
    group_id: u64,
    member_principal: Principal,
) -> Result<JoinedMemberResponse, ApiError> {
    query_candid_as::<(u64, Principal), (Result<JoinedMemberResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_group_member",
        (group_id, member_principal),
    )
    .expect("Failed to call get_group_member from pocketIC")
    .0
}

pub fn get_groups_for_members(member_principals: Vec<Principal>) -> Vec<JoinedMemberResponse> {
    query_candid_as::<(Vec<Principal>,), (Vec<JoinedMemberResponse>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_groups_for_members",
        (member_principals,),
    )
    .expect("Failed to call get_groups_for_members from pocketIC")
    .0
}

pub fn get_group_members(group_id: u64) -> Result<Vec<JoinedMemberResponse>, ApiError> {
    query_candid_as::<(u64,), (Result<Vec<JoinedMemberResponse>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_group_members",
        (group_id,),
    )
    .expect("Failed to call get_group_members from pocketIC")
    .0
}

pub fn get_self_group() -> Member {
    let member: Member = query_candid_as::<(), (Result<Member, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_self_group",
        (),
    )
    .expect("Failed to call get_self_group from pocketIC")
    .0
    .expect("Failed to get self group");

    member
}

pub fn get_member_roles(
    group_id: u64,
    member_principal: Principal,
) -> Result<Vec<String>, ApiError> {
    query_candid_as::<(u64, Principal), (Result<Vec<String>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_member_roles",
        (group_id, member_principal),
    )
    .expect("Failed to call get_member_roles from pocketIC")
    .0
}

pub fn leave_group(group_id: u64) -> Result<(), ApiError> {
    update_candid_as::<(u64,), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "leave_group",
        (group_id,),
    )
    .expect("Failed to call leave_group from pocketIC")
    .0
}

pub fn remove_invite(group_id: u64) -> Result<(), ApiError> {
    update_candid_as::<(u64,), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_invite",
        (group_id,),
    )
    .expect("Failed to call remove_invite from pocketIC")
    .0
}

pub fn remove_member_from_group(group_id: u64, principal: Principal) -> Result<(), ApiError> {
    update_candid_as::<(u64, Principal), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_member_from_group",
        (group_id, principal),
    )
    .expect("Failed to call remove_member_from_group from pocketIC")
    .0
}

pub fn remove_member_invite_from_group(
    group_id: u64,
    principal: Principal,
) -> Result<(), ApiError> {
    update_candid_as::<(u64, Principal), (Result<(), ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "remove_member_invite_from_group",
        (group_id, principal),
    )
    .expect("Failed to call remove_member_invite_from_group from pocketIC")
    .0
}

pub fn get_group_invites(group_id: u64) -> Result<Vec<InviteMemberResponse>, ApiError> {
    update_candid_as::<(u64,), (Result<Vec<InviteMemberResponse>, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_group_invites",
        (group_id,),
    )
    .expect("Failed to call get_group_invites from pocketIC")
    .0
}
