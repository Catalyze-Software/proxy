use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    member::{JoinedMemberResponse, Member},
};

use crate::storage::{MemberStore, StorageInsertableByKey, StorageQueryable};

pub struct MemberCalls;
pub struct MemberMapper;

impl MemberCalls {
    pub fn create_empty_member(principal: Principal) -> Result<(Principal, Member), ApiError> {
        if MemberStore::get(principal).is_ok() {
            return Err(ApiError::duplicate()
                .add_method_name("create_empty_member")
                .add_message("Member already exists"));
        }

        let new_member = Member::new();
        MemberStore::insert_by_key(principal, new_member)
    }
}

impl MemberMapper {
    pub fn to_joined_member_response(
        principal: Principal,
        member: Member,
        group_id: u64,
    ) -> Result<JoinedMemberResponse, ApiError> {
        if !member.is_group_joined(&group_id) {
            return Err(ApiError::not_found()
                .add_method_name("to_joined_member_response")
                .add_message("Member not joined the group"));
        };
        Ok(JoinedMemberResponse::new(principal, member, group_id))
    }
}
