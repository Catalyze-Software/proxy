use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    friend_request::{FriendRequest, FriendRequestResponse},
    profile::ProfileResponse,
    relation_type::RelationType,
};
use ic_cdk::caller;

use crate::storage::{
    FriendRequestStore, ProfileStore, StorageInsertable, StorageQueryable, StorageUpdateable,
};

use super::notification_logic::NotificationCalls;

pub struct FriendRequestCalls;
pub struct FriendRequestMapper;
pub struct FriendRequestValidation;

impl FriendRequestCalls {
    pub fn add_friend_request(
        to: Principal,
        message: String,
    ) -> Result<FriendRequestResponse, ApiError> {
        let friend_request = FriendRequest::new(caller(), to, message);
        let (_, caller_profile) = ProfileStore::get(caller())?;

        if caller_profile.relations.contains_key(&to) {
            return Err(ApiError::duplicate()
                .add_method_name("add_friend_request")
                .add_message("Friend request already exists"));
        }

        // if somebody tries to make the same friend request
        if FriendRequestStore::find(|_, request| {
            request.to == to && request.requested_by == caller()
        })
        .is_some()
        {
            return Err(ApiError::duplicate()
                .add_method_name("add_friend_request")
                .add_message("Friend request already exists"));
        }

        // if there is a friend request from the caller to the to
        if FriendRequestStore::find(|_, request| {
            request.to == caller() && request.requested_by == to
        })
        .is_some()
        {
            return Err(ApiError::duplicate()
                .add_method_name("add_friend_request")
                .add_message("Friend request already exists"));
        }

        let (friend_request_id, mut inserted_friend_request) =
            FriendRequestStore::insert(friend_request)?;

        let friend_request_response =
            FriendRequestResponse::new(friend_request_id, inserted_friend_request.clone());

        let notification_id =
            NotificationCalls::notification_add_friend_request(friend_request_response)?;

        inserted_friend_request.set_notification_id(notification_id);

        FriendRequestMapper::to_result_response(FriendRequestStore::update(
            friend_request_id,
            inserted_friend_request,
        ))
    }

    pub fn accept_friend_request(friend_request_id: u64) -> Result<bool, ApiError> {
        let (_, friend_request) = FriendRequestStore::get(friend_request_id)?;

        if friend_request.to != caller() {
            return Err(ApiError::unauthorized()
                .add_method_name("accept_friend_request")
                .add_message("You are not authorized to accept this friend request"));
        }

        let (_, mut caller_profile) = ProfileStore::get(caller())?;

        caller_profile.relations.insert(
            friend_request.requested_by,
            RelationType::Friend.to_string(),
        );

        let (requested_by_principal, mut to_profile) =
            ProfileStore::get(friend_request.requested_by)?;
        to_profile
            .relations
            .insert(friend_request.to, RelationType::Friend.to_string());

        ProfileStore::update(caller(), caller_profile)?;
        ProfileStore::update(requested_by_principal, to_profile)?;

        NotificationCalls::notification_accept_or_decline_friend_request(
            (friend_request_id, friend_request),
            true,
        )?;

        Ok(FriendRequestStore::remove(friend_request_id))
    }

    pub fn decline_friend_request(friend_request_id: u64) -> Result<bool, ApiError> {
        let (_, friend_request) = FriendRequestStore::get(friend_request_id)?;

        if friend_request.to != caller() {
            return Err(ApiError::unauthorized()
                .add_method_name("decline_friend_request")
                .add_message("You are not authorized to decline this friend request"));
        }

        NotificationCalls::notification_accept_or_decline_friend_request(
            (friend_request_id, friend_request),
            false,
        )?;
        Ok(FriendRequestStore::remove(friend_request_id))
    }

    pub fn remove_friend_request(friend_request_id: u64) -> Result<bool, ApiError> {
        let (_, friend_request) = FriendRequestStore::get(friend_request_id)?;

        if friend_request.requested_by != caller() {
            return Err(ApiError::unauthorized()
                .add_method_name("remove_friend_request")
                .add_message("You are not authorized to remove this friend request"));
        }

        NotificationCalls::notification_remove_friend_request(friend_request.to, friend_request_id);
        Ok(FriendRequestStore::remove(friend_request_id))
    }

    pub fn get_incoming_friend_requests() -> Vec<FriendRequestResponse> {
        FriendRequestStore::filter(|_, request| request.to == caller())
            .into_iter()
            .map(FriendRequestMapper::to_response)
            .collect()
    }

    pub fn get_incoming_friend_requests_with_profile(
    ) -> Vec<(FriendRequestResponse, ProfileResponse)> {
        FriendRequestStore::filter(|_, request| request.to == caller())
            .into_iter()
            .map(|data| {
                // TODO: Unwrap is possibly safe, but needs to be handled better
                let (principal, profile) = ProfileStore::get(data.1.requested_by).unwrap();
                (
                    FriendRequestMapper::to_response(data),
                    ProfileResponse::new(principal, profile),
                )
            })
            .collect()
    }

    pub fn get_outgoing_friend_requests() -> Vec<FriendRequestResponse> {
        FriendRequestStore::filter(|_, request| request.requested_by == caller())
            .into_iter()
            .map(FriendRequestMapper::to_response)
            .collect()
    }

    pub fn get_outgoing_friend_requests_with_profile(
    ) -> Vec<(FriendRequestResponse, ProfileResponse)> {
        FriendRequestStore::filter(|_, request| request.requested_by == caller())
            .into_iter()
            .map(|data| {
                // TODO: Unwrap is possibly safe, but needs to be handled better
                let (principal, profile) = ProfileStore::get(data.1.to).unwrap();
                (
                    FriendRequestMapper::to_response(data),
                    ProfileResponse::new(principal, profile),
                )
            })
            .collect()
    }
}

impl FriendRequestMapper {
    pub fn to_response(friend_request: (u64, FriendRequest)) -> FriendRequestResponse {
        FriendRequestResponse {
            id: friend_request.0,
            requested_by: friend_request.1.requested_by,
            message: friend_request.1.message,
            to: friend_request.1.to,
            created_at: friend_request.1.created_at,
        }
    }
    pub fn to_result_response(
        friend_request_result: Result<(u64, FriendRequest), ApiError>,
    ) -> Result<FriendRequestResponse, ApiError> {
        match friend_request_result {
            Ok((id, friend_request)) => Ok(FriendRequestResponse {
                id,
                requested_by: friend_request.requested_by,
                message: friend_request.message,
                to: friend_request.to,
                created_at: friend_request.created_at,
            }),
            Err(e) => Err(e),
        }
    }
}
