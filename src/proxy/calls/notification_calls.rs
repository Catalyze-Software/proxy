use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    notification::{MultisigNotificationType, NotificationResponse},
    transaction_data::{TransactionCompleteData, TransactionData},
    user_notifications::UserNotificationData,
};
use ic_cdk::{caller, query, update};

use crate::logic::notification_logic::NotificationCalls;

#[query]
fn get_notifications() -> Vec<NotificationResponse> {
    NotificationCalls::get_user_notifications(caller())
}

#[query]
fn get_unread_notifications() -> Vec<NotificationResponse> {
    NotificationCalls::get_user_unread_notifications(caller())
}

#[update]
fn mark_notifications_as_read(
    ids: Vec<u64>,
    is_read: bool,
) -> Result<Vec<(u64, UserNotificationData)>, ApiError> {
    NotificationCalls::mark_notifications_as_read(caller(), ids, is_read)
}

#[update]
fn remove_notifications(ids: Vec<u64>) -> Vec<(u64, UserNotificationData)> {
    NotificationCalls::remove_user_notifications(caller(), ids)
}

#[update]
fn remove_all_notifications() -> Vec<(u64, UserNotificationData)> {
    NotificationCalls::remove_all_user_notifications(caller())
}

#[update]
fn add_transaction_notification(transaction: TransactionData) -> bool {
    if caller().to_string() != "4bli7-7iaaa-aaaap-ahd4a-cai" {
        return false;
    }

    NotificationCalls::notification_add_transaction(transaction)
}

#[update]
fn add_transactions_complete_notification(data: TransactionCompleteData) -> bool {
    if caller().to_string() != "4bli7-7iaaa-aaaap-ahd4a-cai" {
        return false;
    }

    NotificationCalls::notification_add_complete_transaction(data)
}

#[update]
pub fn multisig_whitelist_notice_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig(
        receivers,
        MultisigNotificationType::WhitelistNotice((multisig_wallet_canister, group_id)),
    )
}

#[update]
pub fn multisig_proposal_accept_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    proposal_id: u64,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig(
        receivers,
        MultisigNotificationType::ProposalAccept((multisig_wallet_canister, proposal_id, group_id)),
    )
}

#[update]
pub fn multisig_proposal_decline_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    proposal_id: u64,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig(
        receivers,
        MultisigNotificationType::ProposalDecline((
            multisig_wallet_canister,
            proposal_id,
            group_id,
        )),
    )
}
#[update]
pub fn multisig_proposal_status_update_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    proposal_id: u64,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig_silent(
        receivers,
        MultisigNotificationType::ProposalStatusUpdate((
            multisig_wallet_canister,
            proposal_id,
            group_id,
        )),
    )
}
#[update]
pub fn multisig_new_proposal_notification(
    receivers: Vec<Principal>,
    multisig_wallet_canister: Principal,
    proposal_id: u64,
    group_id: u64,
) -> bool {
    NotificationCalls::notification_add_multisig(
        receivers,
        MultisigNotificationType::NewProposal((multisig_wallet_canister, proposal_id, group_id)),
    )
}
