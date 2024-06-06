use catalyze_shared::api_error::ApiError;
use ic_cdk::{caller, query, update};

use canister_types::models::{
    notification::NotificationResponse,
    transaction_data::{TransactionCompleteData, TransactionData},
    user_notifications::UserNotificationData,
};

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
