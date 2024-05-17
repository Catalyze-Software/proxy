# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.9]

### Added

- methods to the `DateRange` implementation
- `Event` date check to see if an event is ongoing, if so the `StartDate` filter gets ignored

### Changed

-

### Removed

- testing folder
- `get_event_count` guard

### Fixed

- `get_boosted_events` was fetching data by the wrong `id`
- `get_boosted_groups` was fetching data by the wrong `id`

## [0.1.8]

### Added

- added `is_prod_developer` guard
- added `_dev_create_canister` call to spin up canisters on the same subnet
- CI/CD pipelines for all environments

### Changed

- added guards to the `_dev` prefixed calls
- removed `_dev_clear` calls

## [0.1.7]

### Added

- `add_topic` update call, to add such topic as tags, interests and skills.
- `get_topic` query call, to get the topic by their id and kind.
- `get_topics` query call, to get the topics by their ids and kind.
- `get_all_topics` query call, to get all the topics by their kind.

### Changed

- Refactored `StorageMethods` trait into the `Storage`, `StorageQueryable`, `StorageUpdateable`,
  `StorageInsertable` and `StorageInsertableByKey` traits.

## [0.1.6]

### Added

- Logging interface
- Specific login log api
- Added `decline_user_request_event_invite` update call
- Added `decline_owner_request_event_invite` update call
- Added `TransactionData` struct for handling transaction
- Added `TransactionDataComplete` struct for handling transaction
- Added `add_transaction_notification` update call
- Added `add_transactions_complete_notification` update call

### Changed

- changed `UserJoinEvent(event_id)` to `UserJoinEvent((group_id, event_id))`
- on group invite accept send silent notification to all group members
- on group invite decline send silent notification to all higher role members
- rename `accept_user_request_event_invite` to `accept_or_decline_user_request_event_invite`
- rename `accept_owner_request_event_invite` to `accept_or_decline_owner_request_event_invite`
- Changed error text for `create_empty_attendee` and `to_joined_attendee_response`
- Store counters start at `1` instead of `0`!!

### Fixed

- Fixed bug on `create_empty_attendee` where it was checking the `MemberStore` instead of the `AttendeeStore`
- Fixed notification bug for events and groups

### Removed

- Removed stores for storing the `Identifier - id` reference (thus removing backward compatibility)

## [0.1.5]

### Added

- Added `special_members` to group to block or privilage members within a group
- Added `ban_group_member` call
- Added `remove_ban_from_group_member` call
- Added `get_banned_group_members` call
- Added `_dev_check_member_sync` call to check if the stores are in sync
- Added `_dev_check_attendees_sync` call to check if the stores are in sync
- Added `_dev_check_events_sync` call to check if the stores are in sync
- Added `processed_by` to `Notification` struct
- Added `attendee_count` to `EventResponse`
- Added notification for group role change
- Added notification for group invite remove
- Added `_dev_clear_friend_request` call

### Changed

- Send notification to higher role members when owner sends `group_join_request` to user
- Send notification to higher role members when owner sends `event_join_request` to user (not used)
- Send notification to higher role members when user accepts group invite

### Fixed

- changed `get_group_invites` to `query`
- changed `get_group_invites_with_profiles` to `query`
- change `get_reports` permission to `can_read`
- Fix store clearing
- Fix `CallerData` to resturn the correct data for events
- Fixed removing attendee from event

### Removed

## [0.1.4]

### Added

- Added `get_group_by_name` query call
- Added `get_event_attendees_profiles_and_roles` query call
- Added `get_event_invites_with_profiles` query call
- Added `get_group_members_with_profiles` query call
- Added `get_group_member_with_profile` query call
- Added `get_group_invites_with_profiles` query call
- Added `get_incoming_friend_requests_with_profile` query call
- Added `get_outgoing_friend_requests_with_profile` query call
- Added `get_relations_with_profiles` query call
- Added `SubjectResponse` to pass back reported objects
- Added `clear` methods to the stores
- added `_dev_clear_notifications` to clear the notifications from the frontend

### Changed

- Changed `get_boosted_groups` response from `Vec<(u64, Boost)>` to `Vec<GroupResponse>`
- Changed `get_boosted_events` response from `Vec<(u64, Boost)>` to `Vec<EventResponse>`
- changed `get_boosts_by_subject` parameter from `subject: Subject` to `subject: SubjectType`
- Change `get_notifications` response to `Vec<NotificationResponse>`
- Change `get_unread_notifications` response to `Vec<NotificationResponse>`
- Changed `Subject` on `ReportResponse` to `SubjectResponse` which passes back the reported object
- changed `get_pinned_by_subject_type` response to `Vec<SubjectResponse>`
- Changed `GroupNotificationType` `JoinGroupUserRequestAccept` and `JoinGroupUserRequestDecline` to return `InviteMemberResponse`
- Changed `GroupNotificationType` `JoinGroupOwnerRequestAccept` and `JoinGroupUserRequestDecline` to return `InviteMemberResponse`
- Changed `EventNotificationType` `JoinEventUserRequestAccept` and `JoinEventUserRequestDecline` to return `InviteAttendeeResponse`
- Changed `EventNotificationType` `JoinEventOwnerRequestAccept` and `JoinEventUserRequestDecline` to return `InviteAttendeeResponse`

### Fixed

- Fixed `get_unread_notifications` which returned all notifications

### Removed

## [0.1.3]

### Added

- Added `GroupMemberStore` to improve lookup performance
- Added `GroupMemberStore` initialization when creating a new group
- Added `GroupEventStore` initialization when creating a new group
- Added `UserNotifications` initialization when creating a new profile
-
- Added `EventAttendeeStore` to improve loopup performance
- Added `EventAttendeeStore` initialization when creating a new event
-
- Added `MemberCollection` struct for usage in `GroupMemberStore` and `EventAttendeeStore`

- Added `GroupEventStore` to improve lookup performance
- Added `EventCollection` struct for usage in `GroupEventStore`
- Added migration logic to fill the stores
-

### Changed

- Notification now return a `NotificationResponse` which hold additional metadata instead of a `Notification`
- Change `id: u64` to `id: Option<u64>` in `NotificationResponse` where a non-set `id` means a silent notification
- change `FriendRequest` enum value from `FriendRequest` to `FriendRequestResponse`
- change `JoinEventUserRequest` enum value from `u64` to `InviteAttendeeResponse`
- change `JoinEventOwnerRequest` enum value from `u64` to `InviteAttendeeResponse`
- change `JoinGroupUserRequest` enum value from `u64` to `InviteMemberResponse`
- change `JoinGroupOwnerRequest` enum value from `u64` to `InviteMemberResponse`
- change `FriendRequestAccept` enum value from `u64` to `FriendRequestResponse`
- change `FriendRequestDecline` enum value from `u64` to `FriendRequestResponse`
- change `remove_notifications` to `remove_user_notifications` from `NotificationCalls`

### Fixed

- Fixed `event_count` and `member_count` by usage of the new stores

### Removed

- Removed `SilentNotification` and `SendSilentNotification` from `NotificationType` enum
- Removed `get_user_notifications` query call

## [0.1.2]

### Added

- Add query call `get_self_groups` that return all the groups that the user is part of
- Add query call `get_self_events` that return all the events that the user is part of
- Added `pinned: Vec<Subject>` to `Profile`
- Added `is_pinned` method to `Profile` implementation
- Added `is_pinned` variable to `GroupCallerData` implementation
- Added `add_pinned` query call + method
- Added `remove_pinned` query call + method
- Added `get_pinned_by_subject` query call + method
- Added `add_pinned` query call + method
- Added `remove_pinned` query call + method
- Added `hours_to_nanoseconds` method for date comparison
- Added `members_count` to `GroupResponse`
- Added `events_count` to `GroupResponse`
- Added `get_groups_count` to give back all different filtering counts

### Changed

- Changed query call `get_self_group` to `get_self_member` because this call returns member specific data
- Changed query call `get_self_event` to `get_self_attendee` because this call returns attendee specific data
- Changed `get_event_count` to give back all different filtering counts
- Changed `member_count` and `event_count` to always return `0` for the `get_groups` call because of the message execution limit

### Removed

- Removed not used empty variable from `TransactionNotificationType::Airdrop`
- Removed `get_starred_groups` query in favor of `get_starred_by_subject`
- Removed `get_starred_events` query in favor of `get_starred_by_subject`
- Removed `group_id` param from `get_event` query method
- Removed `group_id` param from `join_event` update method

### Fixed

## [0.1.1]

### Added

- add `get_type` implementation for `Subject` struct

### Changed

- Change method argument order on `*_calls.rs` files
- Replace `identifier` method argument with `u64` on `event_calls.ts`
- Replace `identifier` method argument with `u64` on `group_calls.ts`
- Replace `identifier` method argument with `u64` on `report_calls.tsx`
- Deprecate `identifiers` for `Boost` and replace it with `Subject`
- Change `starred: HashMap<Identifier, String>` to `Vec<Subject>`
- change `get_starred_by_kind` to `get_starred_by_subject`

## [0.1.0] - 2024-03-15

### Added

- Changelog with versioning
- Canister http call to get current changelog `/changelog`
- Canister http call to get current version `/version`
- `#[allow(unused)]` to deprecated migration methods

## Fixed

- Missing `notification_id` on migration models

[Unreleased]: https://github.com/Catalyze-Software/proxy/compare/0.1.8...HEAD
[0.1.9]: https://github.com/Catalyze-Software/proxy/compare/0.1.8...0.1.9
[0.1.8]: https://github.com/Catalyze-Software/proxy/compare/0.1.7...0.1.8
[0.1.7]: https://github.com/Catalyze-Software/proxy/compare/0.1.6...0.1.7
[0.1.6]: https://github.com/Catalyze-Software/proxy/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/Catalyze-Software/proxy/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/Catalyze-Software/proxy/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/Catalyze-Software/proxy/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/Catalyze-Software/proxy/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/Catalyze-Software/proxy/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/Catalyze-Software/proxy/releases/tag/0.1.0
