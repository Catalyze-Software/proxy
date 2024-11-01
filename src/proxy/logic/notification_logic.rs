use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    attendee::{AttendeeInvite, InviteAttendeeResponse, JoinedAttendeeResponse},
    friend_request::{FriendRequest, FriendRequestResponse},
    member::{InviteMemberResponse, JoinedMemberResponse, MemberInvite},
    notification::{
        EventNotificationType, GroupNotificationType, MultisigNotificationType, Notification,
        NotificationResponse, NotificationType, RelationNotificationType,
        TransactionNotificationType,
    },
    transaction_data::{TransactionCompleteData, TransactionData},
    user_notifications::{UserNotificationData, UserNotifications},
    websocket_message::WSMessage,
};
use ic_cdk::caller;

use crate::{
    storage::{
        NotificationStore, StorageInsertable, StorageInsertableByKey, StorageQueryable,
        StorageUpdateable, UserNotificationStore,
    },
    MULTISIG_INDEX,
};

use super::websocket_logic::Websocket;

pub struct NotificationCalls;

impl NotificationCalls {
    // Friend request notifications

    /// stores + sends notification
    pub fn notification_add_friend_request(
        friend_request: FriendRequestResponse,
    ) -> Result<u64, ApiError> {
        let (notification_id, notification) = Self::add_notification(
            vec![friend_request.to],
            NotificationType::Relation(RelationNotificationType::FriendRequest(
                friend_request.clone(),
            )),
            true,
        )?;

        Self::send_notification(Some(notification_id), notification, friend_request.to);
        Ok(notification_id)
    }

    /// stores + sends notification
    pub fn notification_accept_or_decline_friend_request(
        friend_request_data: (u64, FriendRequest),
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        // get the associated friend request
        let (_, friend_request) = friend_request_data;

        // check if the notification exists
        if let Some(notification_id) = friend_request.notification_id {
            let (_, mut notification) = NotificationStore::get(notification_id)?;

            // check if the notification is a friend request
            if let NotificationType::Relation(RelationNotificationType::FriendRequest(
                friend_request,
            )) = &notification.notification_type.clone()
            {
                // mark the notification as accepted, this also marks it as not actionable
                let notification_type = match is_accepted {
                    true => RelationNotificationType::FriendRequestAccept(friend_request.clone()),
                    false => RelationNotificationType::FriendRequestDecline(friend_request.clone()),
                };
                notification
                    .mark_as_accepted(is_accepted, NotificationType::Relation(notification_type));
                let _ = NotificationStore::update(notification_id, notification.clone());

                Self::send_notification(
                    Some(notification_id),
                    notification.clone(),
                    friend_request.requested_by,
                );
                Self::send_notification(None, notification, friend_request.to);

                Ok(())
            } else {
                Err(ApiError::bad_request().add_message("Notification is not a friend request"))
            }
        } else {
            Err(ApiError::not_found())
        }
    }

    // sends notification
    pub fn notification_remove_friend_request(receiver: Principal, friend_request_id: u64) {
        Self::send_notification(
            None,
            Notification::new(
                NotificationType::Relation(RelationNotificationType::FriendRequestRemove(
                    friend_request_id,
                )),
                false,
            ),
            receiver,
        );
    }

    // sends notification
    pub fn notification_remove_friend(receiver: Principal, friend_principal: Principal) {
        Self::send_notification(
            None,
            Notification::new(
                NotificationType::Relation(RelationNotificationType::FriendRemove(
                    friend_principal,
                )),
                false,
            ),
            receiver,
        );
    }

    // Group notifications

    // sends notification
    pub fn notification_join_public_group(receivers: Vec<Principal>, group_id: u64) {
        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::UserJoinGroup(group_id)),
                    false,
                ),
                receiver,
            );
        }
    }

    pub fn notification_leave_group(receivers: Vec<Principal>, group_id: u64) {
        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::UserLeaveGroup(group_id)),
                    false,
                ),
                receiver,
            );
        }
    }

    // stores + sends notification
    pub fn notification_user_join_request_group(
        receivers: Vec<Principal>,
        invite_member_response: InviteMemberResponse,
    ) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_and_send_notification(
            receivers,
            NotificationType::Group(GroupNotificationType::JoinGroupUserRequest(
                invite_member_response,
            )),
            true,
        )?;

        Ok(notification_id)
    }

    // sends notifications
    pub fn notification_user_join_request_group_accept_or_decline(
        invite: MemberInvite,
        is_accepted: bool,
        group_members: Vec<Principal>,
        higher_role_members: Vec<Principal>,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = NotificationStore::get(notification_id)?;

            if let NotificationType::Group(GroupNotificationType::JoinGroupUserRequest(
                invite_member_response,
            )) = notification.notification_type.clone()
            {
                let notification_type = match is_accepted {
                    true => {
                        GroupNotificationType::JoinGroupUserRequestAccept(invite_member_response)
                    }
                    false => {
                        GroupNotificationType::JoinGroupUserRequestDecline(invite_member_response)
                    }
                };

                notification
                    .mark_as_accepted(is_accepted, NotificationType::Group(notification_type));
                let _ = NotificationStore::update(notification_id, notification.clone());

                Self::send_notification(
                    Some(notification_id),
                    notification.clone(),
                    notification.sender, // the person who request to join
                );

                match is_accepted {
                    true => {
                        for r in higher_role_members {
                            Self::send_notification(None, notification.clone(), r);
                        }
                    }
                    false => {
                        for r in group_members {
                            Self::send_notification(None, notification.clone(), r);
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    // stores + sends notification
    pub fn notification_owner_join_request_group(
        invitee_principal: Principal,
        invite_member_response: InviteMemberResponse,
        receivers: Vec<Principal>,
    ) -> Result<u64, ApiError> {
        let (notification_id, notification) = Self::add_and_send_notification(
            vec![invitee_principal],
            NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(
                invite_member_response,
            )),
            true,
        )?;

        for r in receivers {
            Self::send_notification(None, notification.clone(), r);
        }

        Ok(notification_id)
    }

    // sends notification
    pub fn notification_owner_join_request_group_accept_or_decline(
        invitee_principal: Principal,
        invite: MemberInvite,
        is_accepted: bool,
        group_members: Vec<Principal>,
        higher_role_members: Vec<Principal>,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = NotificationStore::get(notification_id)?;

            if let NotificationType::Group(GroupNotificationType::JoinGroupOwnerRequest(
                invite_member_response,
            )) = notification.notification_type.clone()
            {
                let notification_type = match is_accepted {
                    true => {
                        GroupNotificationType::JoinGroupOwnerRequestAccept(invite_member_response)
                    }
                    false => {
                        GroupNotificationType::JoinGroupOwnerRequestDecline(invite_member_response)
                    }
                };

                notification
                    .mark_as_accepted(is_accepted, NotificationType::Group(notification_type));
                let _ = NotificationStore::update(notification_id, notification.clone());

                match is_accepted {
                    true => {
                        Self::send_notification(None, notification.clone(), invitee_principal);
                        for r in group_members {
                            if notification.sender == r {
                                Self::send_notification(
                                    Some(notification_id),
                                    notification.clone(),
                                    r,
                                );
                            } else {
                                Self::send_notification(None, notification.clone(), r);
                            }
                        }
                    }
                    false => {
                        for r in higher_role_members {
                            if notification.sender == r {
                                Self::send_notification(
                                    Some(notification_id),
                                    notification.clone(),
                                    r,
                                );
                            } else {
                                Self::send_notification(None, notification.clone(), r);
                            }
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    pub fn notification_change_group_member_role(
        member: JoinedMemberResponse,
        receivers: Vec<Principal>,
    ) {
        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::RoleAssignByOwner(
                        member.clone(),
                    )),
                    false,
                ),
                receiver,
            );
        }
    }

    pub fn notification_remove_group_member(
        member: JoinedMemberResponse,
        receivers: Vec<Principal>,
    ) {
        Self::send_notification(
            None,
            Notification::new(
                NotificationType::Group(GroupNotificationType::RemoveMemberByOwner(member.clone())),
                false,
            ),
            member.principal,
        );

        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Group(GroupNotificationType::RemoveMemberByOwner(
                        member.clone(),
                    )),
                    false,
                ),
                receiver,
            );
        }
    }

    pub fn notification_remove_group_invite(
        invite: InviteMemberResponse,
        receivers: Vec<Principal>,
    ) {
        if let Some(_invite) = invite.invite.clone() {
            if let Some(notification_id) = _invite.notification_id {
                if let Ok((_, mut notification)) = NotificationStore::get(notification_id) {
                    notification.mark_as_accepted(
                        false,
                        NotificationType::Group(GroupNotificationType::RemoveInviteByOwner(
                            invite.clone(),
                        )),
                    );
                    let _ = NotificationStore::update(notification_id, notification.clone());

                    Self::send_notification(None, notification.clone(), invite.principal);

                    for receiver in receivers {
                        Self::send_notification(None, notification.clone(), receiver);
                    }
                }
            }
        }
    }

    // Event notifications

    // sends notification
    pub fn notification_join_public_event(receivers: Vec<Principal>, group_id: u64, event_id: u64) {
        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Event(EventNotificationType::UserJoinEvent((
                        group_id, event_id,
                    ))),
                    false,
                ),
                receiver,
            );
        }
    }

    // store + sends notification
    pub fn notification_user_join_request_event(
        receivers: Vec<Principal>,
        invite_attendee_response: InviteAttendeeResponse,
    ) -> Result<u64, ApiError> {
        let (notification_id, _) = Self::add_and_send_notification(
            receivers,
            NotificationType::Event(EventNotificationType::JoinEventUserRequest(
                invite_attendee_response,
            )),
            true,
        )?;

        Ok(notification_id)
    }

    // sends notifications
    pub fn notification_user_join_request_event_accept_or_decline(
        receiver: Principal,
        invite: AttendeeInvite,
        event_attendees: Vec<Principal>,
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = NotificationStore::get(notification_id)?;

            if let NotificationType::Event(EventNotificationType::JoinEventUserRequest(
                invite_attendee_response,
            )) = notification.notification_type.clone()
            {
                let notification_type = match is_accepted {
                    true => {
                        EventNotificationType::JoinEventUserRequestAccept(invite_attendee_response)
                    }
                    false => {
                        EventNotificationType::JoinEventUserRequestDecline(invite_attendee_response)
                    }
                };

                notification
                    .mark_as_accepted(is_accepted, NotificationType::Event(notification_type));
                let _ = NotificationStore::update(notification_id, notification.clone());

                // send notification to the users who could have accepted the request
                Self::send_notification(None, notification.clone(), receiver);

                if is_accepted {
                    for r in event_attendees {
                        if notification.sender == r {
                            Self::send_notification(Some(notification_id), notification.clone(), r);
                        } else {
                            Self::send_notification(None, notification.clone(), r);
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    // sends notification
    pub fn notification_owner_join_request_event_accept_or_decline(
        invitee_principal: Principal,
        invite: AttendeeInvite,
        event_attendees: Vec<Principal>,
        is_accepted: bool,
    ) -> Result<(), ApiError> {
        if let Some(notification_id) = invite.notification_id {
            let (_, mut notification) = NotificationStore::get(notification_id)?;

            if let NotificationType::Event(EventNotificationType::JoinEventOwnerRequest(
                event_attendee_response,
            )) = notification.notification_type.clone()
            {
                let notification_type = match is_accepted {
                    true => {
                        EventNotificationType::JoinEventOwnerRequestAccept(event_attendee_response)
                    }
                    false => {
                        EventNotificationType::JoinEventOwnerRequestDecline(event_attendee_response)
                    }
                };

                notification
                    .mark_as_accepted(is_accepted, NotificationType::Event(notification_type));
                let _ = NotificationStore::update(notification_id, notification.clone());

                // send notification to the users who could have accepted the request
                Self::send_notification(None, notification.clone(), invitee_principal);

                if is_accepted {
                    for r in event_attendees {
                        if notification.sender == r {
                            Self::send_notification(Some(notification_id), notification.clone(), r);
                        } else {
                            Self::send_notification(None, notification.clone(), r);
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(ApiError::bad_request()
                .add_message("Notification is not a user join group request"))
        }
    }

    // stores + sends notification
    pub fn notification_owner_join_request_event(
        invitee_principal: Principal,
        invite_attendee_response: InviteAttendeeResponse,
        receivers: Vec<Principal>,
    ) -> Result<u64, ApiError> {
        let (notification_id, notification) = Self::add_and_send_notification(
            vec![invitee_principal],
            NotificationType::Event(EventNotificationType::JoinEventOwnerRequest(
                invite_attendee_response,
            )),
            true,
        )?;

        for r in receivers {
            Self::send_notification(None, notification.clone(), r);
        }

        Ok(notification_id)
    }

    pub fn notification_remove_event_invite(notification_id: u64, invite: InviteAttendeeResponse) {
        if let Ok((_, mut notification)) = NotificationStore::get(notification_id) {
            notification.mark_as_accepted(
                false,
                NotificationType::Event(EventNotificationType::RemoveInviteByOwner(invite.clone())),
            );
            let _ = NotificationStore::update(notification_id, notification.clone());

            Self::send_notification(None, notification.clone(), invite.principal);
        }
    }

    pub fn notification_remove_event_attendee(
        attendee: JoinedAttendeeResponse,
        receivers: Vec<Principal>,
    ) {
        Self::send_notification(
            None,
            Notification::new(
                NotificationType::Event(EventNotificationType::RemoveAttendeeByOwner(
                    attendee.clone(),
                )),
                false,
            ),
            attendee.principal,
        );

        for receiver in receivers {
            Self::send_notification(
                None,
                Notification::new(
                    NotificationType::Event(EventNotificationType::RemoveAttendeeByOwner(
                        attendee.clone(),
                    )),
                    false,
                ),
                receiver,
            );
        }
    }

    // Transaction notifications
    pub fn notification_add_transaction(transaction: TransactionData) -> bool {
        let _ = Self::add_and_send_notification_without_caller(
            vec![transaction.receiver],
            NotificationType::Transaction(TransactionNotificationType::SingleTransaction(
                transaction,
            )),
            false,
        );
        true
    }

    pub fn notification_add_complete_transaction(data: TransactionCompleteData) -> bool {
        let _ = Self::add_and_send_notification_without_caller(
            vec![data.sender],
            NotificationType::Transaction(TransactionNotificationType::TransactionsComplete(data)),
            false,
        );
        true
    }

    pub fn notification_add_multisig(
        receivers: Vec<Principal>,
        notification: MultisigNotificationType,
    ) -> bool {
        // Only the multisig can call this function
        if caller().to_string() != MULTISIG_INDEX {
            return false;
        }
        let _ = Self::add_and_send_notification_without_caller(
            receivers,
            NotificationType::Multisig(notification),
            false,
        );
        true
    }

    pub fn notification_add_multisig_silent(
        receivers: Vec<Principal>,
        notification: MultisigNotificationType,
    ) -> bool {
        // Only the multisig can call this function
        if caller().to_string() != MULTISIG_INDEX {
            return false;
        }

        for r in receivers {
            Self::send_notification(
                None,
                Notification::new(NotificationType::Multisig(notification.clone()), false),
                r,
            );
        }
        true
    }

    // sends notification
    pub fn get_user_unread_notifications(principal: Principal) -> Vec<NotificationResponse> {
        let user_notifications = Self::get_user_notification_ids(principal);
        NotificationStore::get_many(user_notifications.get_unread_ids())
            .iter()
            .map(|(id, data)| {
                NotificationResponse::new(Some(*id), data.clone(), user_notifications.get(id))
            })
            .collect()
    }

    pub fn get_user_notification_ids(principal: Principal) -> UserNotifications {
        let (_, notifications) =
            UserNotificationStore::get(principal).unwrap_or((principal, UserNotifications::new()));
        notifications
    }

    pub fn get_user_notifications(principal: Principal) -> Vec<NotificationResponse> {
        let user_notifications = Self::get_user_notification_ids(principal);
        let notifications = NotificationStore::get_many(user_notifications.ids());

        let mut notification_responses: Vec<NotificationResponse> = vec![];

        for (notification_id, notification_data) in notifications {
            notification_responses.push(NotificationResponse::new(
                Some(notification_id),
                notification_data,
                user_notifications.get(&notification_id),
            ));
        }

        notification_responses
    }

    pub fn mark_notifications_as_read(
        principal: Principal,
        ids: Vec<u64>,
        is_read: bool,
    ) -> Result<Vec<(u64, UserNotificationData)>, ApiError> {
        let (_, mut user_notifications) = UserNotificationStore::get(principal)?;
        user_notifications.mark_as_read_many(ids, is_read);
        let _ = UserNotificationStore::update(principal, user_notifications.clone());
        Ok(user_notifications.to_vec())
    }

    pub fn remove_user_notifications(
        principal: Principal,
        ids: Vec<u64>,
    ) -> Vec<(u64, UserNotificationData)> {
        let (_, mut user_notifications) =
            UserNotificationStore::get(principal).unwrap_or((principal, UserNotifications::new()));
        user_notifications.remove_many(ids);
        let _ = UserNotificationStore::update(principal, user_notifications.clone());
        user_notifications.to_vec()
    }

    pub fn remove_all_user_notifications(principal: Principal) -> Vec<(u64, UserNotificationData)> {
        let (_, mut user_notifications) =
            UserNotificationStore::get(principal).unwrap_or((principal, UserNotifications::new()));
        user_notifications.clear();
        let _ = UserNotificationStore::update(principal, user_notifications.clone());
        user_notifications.to_vec()
    }

    pub fn add_notification(
        receivers: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> Result<(u64, Notification), ApiError> {
        // Create the new notification
        let notification = Notification::new(notification_type, is_actionable);

        // store the new notification in the notification store
        let (new_notification_id, new_notification) = NotificationStore::insert(notification)?;

        // TODO: disabled for now, because of Selami
        // add the notification reference to the user's notifications
        if let Ok((_, mut caller_notifications)) = UserNotificationStore::get(caller()) {
            caller_notifications.add(new_notification_id, false, true);
            let _ = UserNotificationStore::update(caller(), caller_notifications);
        } else {
            let mut caller_notifications = UserNotifications::new();
            caller_notifications.add(new_notification_id, false, true);
            let _ = UserNotificationStore::insert_by_key(caller(), caller_notifications);
        }

        // send the notification to the receivers
        for receiver in receivers {
            if let Ok((_, mut notifications)) = UserNotificationStore::get(receiver) {
                notifications.add(new_notification_id, false, true);
                let _ = UserNotificationStore::update(receiver, notifications);
            } else {
                let mut notifications = UserNotifications::new();
                notifications.add(new_notification_id, false, true);
                let _ = UserNotificationStore::insert_by_key(receiver, notifications);
            }
        }

        Ok((new_notification_id, new_notification))
    }

    pub fn send_notification(
        // If the notification is silent, the notification id is not required as its not stored in the user's notifications
        notification_id: Option<u64>,
        notification: Notification,
        receiver: Principal,
    ) {
        let (_, user_notifications) = UserNotificationStore::get(receiver)
            .unwrap_or((Principal::anonymous(), UserNotifications::new()));

        match notification_id {
            Some(notification_id) => {
                let user_notification_data = user_notifications.get(&notification_id);
                let notification_response = NotificationResponse::new(
                    Some(notification_id),
                    notification,
                    user_notification_data,
                );

                Websocket::send_message(
                    receiver,
                    WSMessage::Notification(notification_response.clone()),
                );
            }
            None => {
                Websocket::send_message(
                    receiver,
                    WSMessage::Notification(NotificationResponse::new(None, notification, None)),
                );
            }
        }
    }

    pub fn add_and_send_notification(
        receivers: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> Result<(u64, Notification), ApiError> {
        // Create the new notification
        let notification = Notification::new(notification_type, is_actionable);

        // store the new notification in the notification store
        let (new_notification_id, new_notification) = NotificationStore::insert(notification)?;

        // add the notification reference to the user's notifications
        if let Ok((_, mut caller_notifications)) = UserNotificationStore::get(caller()) {
            caller_notifications.add(new_notification_id, false, true);
            let _ = UserNotificationStore::update(caller(), caller_notifications);
        } else {
            let mut caller_notifications = UserNotifications::new();
            caller_notifications.add(new_notification_id, false, true);
            let _ = UserNotificationStore::insert_by_key(caller(), caller_notifications);
        }

        // send the notification to the receivers
        for receiver in receivers {
            if let Ok((_, mut notifications)) = UserNotificationStore::get(receiver) {
                notifications.add(new_notification_id, false, true);
                let _ = UserNotificationStore::update(receiver, notifications);
            } else {
                let mut notifications = UserNotifications::new();
                notifications.add(new_notification_id, false, true);
                let _ = UserNotificationStore::insert_by_key(receiver, notifications);
            }

            // send the notification to the receiver
            Self::send_notification(
                Some(new_notification_id),
                new_notification.clone(),
                receiver,
            );
        }

        Ok((new_notification_id, new_notification))
    }

    pub fn add_and_send_notification_without_caller(
        receivers: Vec<Principal>,
        notification_type: NotificationType,
        is_actionable: bool,
    ) -> Result<(u64, Notification), ApiError> {
        // Create the new notification
        let notification = Notification::new(notification_type, is_actionable);

        // store the new notification in the notification store
        let (new_notification_id, new_notification) = NotificationStore::insert(notification)?;

        // send the notification to the receivers
        for receiver in receivers {
            if let Ok((_, mut notifications)) = UserNotificationStore::get(receiver) {
                notifications.add(new_notification_id, false, true);
                let _ = UserNotificationStore::update(receiver, notifications);
            } else {
                let mut notifications = UserNotifications::new();
                notifications.add(new_notification_id, false, true);
                let _ = UserNotificationStore::insert_by_key(receiver, notifications);
            }

            // send the notification to the receiver
            Self::send_notification(
                Some(new_notification_id),
                new_notification.clone(),
                receiver,
            );
        }

        Ok((new_notification_id, new_notification))
    }
}
