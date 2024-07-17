use crate::{
    helpers::guards::{has_access, is_not_anonymous},
    logic::{friend_request_logic::FriendRequestCalls, profile_logic::ProfileCalls},
};
/// # Profile methods
/// # TODO:
/// * Check if the guard are correctly placed
/// * (Application) role based authentication

/// # Questions
/// Check the public / private access of these calls? (anon / registered / application role)
///
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    friend_request::FriendRequestResponse,
    profile::{PostProfile, ProfileResponse, UpdateProfile},
    relation_type::RelationType,
    subject::{Subject, SubjectResponse, SubjectType},
    wallet::PostWallet,
};
use ic_cdk::{query, update};

/// Adds a profile to the canister - [`[update]`](update)
/// # Arguments
/// * `post_profile` - The profile to add
/// # Returns
/// * `ProfileResponse` - The profile that was added
/// # Errors
/// * `ApiError` - If something went wrong while adding the profile
/// # Note
/// This function is guarded by the [`is_not_anonymous`](is_not_anonymous) function.
#[update(guard = "is_not_anonymous")]
pub fn add_profile(post_profile: PostProfile) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::add_profile(post_profile)
}

/// Gets a profile by the given user principal - [`[query]`](query)
/// # Arguments
/// * `principal` - The user principal to get the profile by
/// # Returns
/// * `ProfileResponse` - The profile that was found
/// # Errors
/// * `ApiError` - If something went wrong while getting the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_profile(principal: Principal) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::get_profile(principal)
}

/// Gets profiles by the given user principals - [`[query]`](query)
/// # Arguments
/// * `principals` - The user principals to get the profiles by
/// # Returns
/// * `Vec<ProfileResponse>` - The profiles that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_profiles(principals: Vec<Principal>) -> Vec<ProfileResponse> {
    ProfileCalls::get_profiles(principals)
}

/// Gets profiles that match the given query - [`[query]`](query)
/// # Arguments
/// * `query` - The query to search for
/// # Returns
/// * `Vec<ProfileResponse>` - The profiles that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn query_profiles(query: String) -> Vec<ProfileResponse> {
    ProfileCalls::query_profiles(query)
}

/// Edit the caller his a profile - [`[update]`](update)
/// # Arguments
/// * `update_profile` - The profile to update
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while updating the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn edit_profile(update_profile: UpdateProfile) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::update_profile(update_profile)
}

/// Adds a wallet to the caller his profile - [`[update]`](update)
/// # Change
/// * was `add_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `wallet` - The wallet to add
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while adding the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_wallet_to_profile(wallet: PostWallet) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::add_wallet_to_profile(wallet)
}

/// Sets a wallet as the primary wallet of the caller his profile - [`[update]`](update)
/// # Arguments
/// * `wallet_principal` - The wallet principal to set as the primary wallet
/// # Returns
/// * `ProfileResponse` - Profile response with the updated wallet
/// # Errors
/// * `ApiError` - If something went wrong while setting the wallet as the primary wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn set_wallet_as_primary(wallet_principal: Principal) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::set_wallet_as_primary(wallet_principal)
}

/// Removes a wallet from the caller his profile - [`[update]`](update)
/// # Change
/// * was `remove_wallet` but due to conflict with other methods it was renamed
/// # Arguments
/// * `wallet_principal` - The wallet to remove
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while removing the wallet
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_wallet_from_profile(
    wallet_principal: Principal,
) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::remove_wallet_from_profile(wallet_principal)
}

/// Adds a starred subject to the caller his profile - [`[update]`](update)
/// # Arguments
/// * `subject` - The subject to add to starred
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while adding the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_starred(subject: Subject) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::add_starred(subject)
}

/// Removes a starred subject from the caller his profile - [`[update]`](update)
/// # Arguments
/// * `subject` - The subject to remove from starred
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while removing the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_starred(subject: Subject) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::remove_starred(subject)
}

/// Gets the starred subjects from the caller his profile - [`[query]`](query)
/// # Arguments
/// * `subject_type` - The starred subjects type to fetch
/// # Returns
/// * `Vec<Principal>` - The group identifiers that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_starred_by_subject_type(subject_type: SubjectType) -> Vec<u64> {
    ProfileCalls::get_starred_by_subject(subject_type)
}

/// Adds a pinned subject to the caller his profile - [`[update]`](update)
/// # Arguments
/// * `subject` - The subject to add to pinned
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while adding the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_pinned(subject: Subject) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::add_pinned(subject)
}

/// Removes a pinned subject from the caller his profile - [`[update]`](update)
/// # Arguments
/// * `subject` - The subject to remove from pinned
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while removing the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_pinned(subject: Subject) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::remove_pinned(subject)
}

/// Gets the pinned subjects from the caller his profile - [`[query]`](query)
/// /// # Arguments
/// * `subject_type` - The pinned subjects type to fetch
/// # Returns
/// * `Vec<Principal>` - The group identifiers that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_pinned_by_subject_type(subject_type: SubjectType) -> Vec<SubjectResponse> {
    ProfileCalls::get_pinned_by_subject(subject_type)
}

/// Create a friend request on behalf of the caller - [`[update]`](update)
/// # Arguments
/// * `to` - The principal to send the friend request to
/// * `message` - The message to send with the friend request
/// # Returns
/// * `FriendRequestResponse` - The friend request that was created
/// # Errors
/// * `ApiError` - If something went wrong while creating the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_friend_request(
    to: Principal,
    message: String,
) -> Result<FriendRequestResponse, ApiError> {
    FriendRequestCalls::add_friend_request(to, message)
}

/// Accept a friend request that is addressed to the caller - [`[update]`](update)
/// # Arguments
/// * `friend_request_id` - The friend request identifier to accept
/// # Returns
/// * `bool` - If the friend request was accepted
/// # Errors
/// * `ApiError` - If something went wrong while accepting the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn accept_friend_request(friend_request_id: u64) -> Result<bool, ApiError> {
    FriendRequestCalls::accept_friend_request(friend_request_id)
}

/// Remove a friend request created by the caller - [`[update]`](update)
/// # Arguments
/// * `friend_request_id` - The friend request identifier to remove
/// # Returns
/// * `bool` - If the friend request was removed
/// # Errors
/// * `ApiError` - If something went wrong while removing the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_friend_request(friend_request_id: u64) -> Result<bool, ApiError> {
    FriendRequestCalls::remove_friend_request(friend_request_id)
}

/// Gets the friend requests that are addressed to the caller - [`[query]`](query)
/// # Returns
/// * `Vec<FriendRequestResponse>` - The friend requests that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_incoming_friend_requests() -> Vec<FriendRequestResponse> {
    FriendRequestCalls::get_incoming_friend_requests()
}

/// Gets the friend requests that are addressed to the caller with the corresponding profile - [`[query]`](query)
/// # Returns
/// * `Vec<(FriendRequestResponse, ProfileResponse)>` - The friend requests that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_incoming_friend_requests_with_profile() -> Vec<(FriendRequestResponse, ProfileResponse)>
{
    FriendRequestCalls::get_incoming_friend_requests_with_profile()
}

/// Gets the friend requests that are send by the caller - [`[query]`](query)
/// # Returns
/// * `Vec<FriendRequestResponse>` - The friend requests that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_outgoing_friend_requests() -> Vec<FriendRequestResponse> {
    FriendRequestCalls::get_outgoing_friend_requests()
}

/// Gets the friend requests that are send to the caller with the corresponding profile - [`[query]`](query)
/// # Returns
/// * `Vec<(FriendRequestResponse, ProfileResponse)>` - The friend requests that were found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_outgoing_friend_requests_with_profile() -> Vec<(FriendRequestResponse, ProfileResponse)>
{
    FriendRequestCalls::get_outgoing_friend_requests_with_profile()
}

/// Decline a friend request that is addressed to the caller - [`[update]`](update)
/// # Arguments
/// * `friend_request_id` - The friend request identifier to decline
/// # Returns
/// * `bool` - If the friend request was declined
/// # Errors
/// * `ApiError` - If something went wrong while declining the friend request
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn decline_friend_request(friend_request_id: u64) -> Result<bool, ApiError> {
    FriendRequestCalls::decline_friend_request(friend_request_id)
}

/// Remove friend from caller profile and remove caller from friend profile - [`[update]`](update)
/// # Arguments
/// * `principal` - The friend principal to remove from the caller his profile
/// # Returns
/// * `bool` - If the friend was removed from the caller his profile
/// # Errors
/// * `ApiError` - If something went wrong while removing the friend
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn remove_friend(principal: Principal) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::remove_friend(principal)
}

/// Block a user on the application level - [`[update]`](update)
/// # Arguments
/// * `principal` - The principal to block
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while blocking the user
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: Check full implementation for this
#[update(guard = "has_access")]
pub fn block_user(principal: Principal) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::block_user(principal)
}

/// Unblock a user on the application level - [`[update]`](update)
/// # Arguments
/// * `principal` - The principal to unblock
/// # Returns
/// * `ProfileResponse` - The profile that was updated
/// # Errors
/// * `ApiError` - If something went wrong while unblocking the user
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
/// TODO: Check full implementation for this
#[update(guard = "has_access")]
pub fn unblock_user(principal: Principal) -> Result<ProfileResponse, ApiError> {
    ProfileCalls::unblock_user(principal)
}

/// Get the current relation for the caller based on the relation type - [`[query]`](query)
/// # Arguments
/// * `relation_type` - The relation type to get the relation for `friend` or `blocked`
/// # Returns
/// * `Vec<Principal>` - The principals of the relations
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_relations(relation_type: RelationType) -> Vec<Principal> {
    ProfileCalls::get_relations(relation_type)
}

/// Get the current relation profiles for the caller based on the relation type - [`[query]`](query)
/// # Arguments
/// * `relation_type` - The relation type to get the relation for `friend` or `blocked`
/// # Returns
/// * `Vec<ProfileResponse>` - The principals of the relations
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_relations_with_profiles(relation_type: RelationType) -> Vec<ProfileResponse> {
    ProfileCalls::get_relations_with_profiles(relation_type)
}

/// Get the current relation count for the caller based on the relation type - [`[query]`](query)
/// # Arguments
/// * `principal` - The principal to get the relation count for
/// * `relation_type` - The relation type to get the relation count for `friend` or `blocked`
/// # Returns
/// * `u64` - The relation count that was found
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_relations_count(relation_type: RelationType) -> u64 {
    ProfileCalls::get_relations(relation_type).len() as u64
}

/// Approve a code of conduct version - [`[update]`](update)
/// # Arguments
/// * `version` - The code of conduct version to approve
/// # Returns
/// * `bool` - If the code of conduct version was approved
/// # Errors
/// * `ApiError` - If something went wrong while approving the code of conduct version
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn approve_code_of_conduct(version: u64) -> Result<bool, ApiError> {
    ProfileCalls::approve_code_of_conduct(version)
}

/// Approve a privacy policy version - [`[update]`](update)
/// # Arguments
/// * `version` - The privacy policy version to approve
/// # Returns
/// * `bool` - If the privacy policy version was approved
/// # Errors
/// * `ApiError` - If something went wrong while approving the privacy policy version
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn approve_privacy_policy(version: u64) -> Result<bool, ApiError> {
    ProfileCalls::approve_privacy_policy(version)
}

/// Approve a terms of service version - [`[update]`](update)
/// # Arguments
/// * `version` - The terms of service version to approve
/// # Returns
/// * `bool` - If the terms of service version was approved
/// # Errors
/// * `ApiError` - If something went wrong while approving the terms of service version
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn approve_terms_of_service(version: u64) -> Result<bool, ApiError> {
    ProfileCalls::approve_terms_of_service(version)
}
