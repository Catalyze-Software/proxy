use super::{
    attendee_logic::AttendeeCalls, member_logic::MemberCalls, notification_logic::NotificationCalls,
};
use crate::{
    helpers::validator::Validator,
    storage::{
        AttendeeStore, EventAttendeeStore, EventStore, GroupMemberStore, GroupStore, MemberStore,
        ProfileStore, StorageInsertableByKey, StorageQueryable, StorageUpdateable,
        UserNotificationStore,
    },
};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    document_details::DocumentDetails,
    member_collection::MemberCollection,
    profile::{PostProfile, Profile, ProfileResponse, UpdateProfile},
    relation_type::RelationType,
    subject::{Subject, SubjectResponse, SubjectType},
    user_notifications::UserNotifications,
    validation::{ValidateField, ValidationType},
    wallet::{PostWallet, Wallet},
};
use ic_cdk::{api::time, caller};

pub struct ProfileCalls;
pub struct ProfileValidation;

impl ProfileCalls {
    pub fn add_profile(post_profile: PostProfile) -> Result<ProfileResponse, ApiError> {
        ProfileValidation::validate_post_profile(&post_profile)?;

        let post_profile_username = post_profile.username.to_lowercase();
        if ProfileStore::find(|_, p| p.username.to_lowercase() == post_profile_username).is_some() {
            return Err(ApiError::duplicate().add_message("Username already exists"));
        }

        let new_profile = Profile::from(post_profile);
        let stored_profile = ProfileStore::insert_by_key(caller(), new_profile);

        let _ = MemberCalls::create_empty_member(caller());
        let _ = AttendeeCalls::create_empty_attendee(caller());
        let _ = UserNotificationStore::insert_by_key(caller(), UserNotifications::new());

        ProfileResponse::from_result(stored_profile)
    }

    pub fn update_profile(update_profile: UpdateProfile) -> Result<ProfileResponse, ApiError> {
        ProfileValidation::validate_update_profile(&update_profile)?;

        let (_, existing_profile) = ProfileStore::get(caller())?;
        let updated_profile = existing_profile.update(update_profile);

        let updated_profile_result = ProfileStore::update(caller(), updated_profile);
        ProfileResponse::from_result(updated_profile_result)
    }

    pub fn add_wallet_to_profile(post_wallet: PostWallet) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = ProfileStore::get(caller())?;

        if existing_profile
            .wallets
            .contains_key(&post_wallet.principal)
        {
            return Err(ApiError::duplicate().add_message("Wallet already exists"));
        }

        existing_profile.wallets.insert(
            post_wallet.principal,
            Wallet {
                provider: post_wallet.provider,
                is_primary: existing_profile.wallets.is_empty(),
            },
        );

        let updated_profile = ProfileStore::update(caller(), existing_profile);

        ProfileResponse::from_result(updated_profile)
    }

    pub fn remove_wallet_from_profile(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = ProfileStore::get(caller())?;

        if !existing_profile.wallets.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Wallet does not exist"));
        }

        if existing_profile
            .wallets
            .get(&principal)
            .is_some_and(|w| w.is_primary)
        {
            return Err(ApiError::bad_request().add_message("Cannot remove primary wallet"));
        }

        existing_profile.wallets.remove(&principal);

        let updated_profile = ProfileStore::update(caller(), existing_profile);

        ProfileResponse::from_result(updated_profile)
    }

    pub fn set_wallet_as_primary(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = ProfileStore::get(caller())?;

        if !existing_profile.wallets.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Wallet does not exist"));
        }

        for (_principal, wallet) in existing_profile.wallets.iter_mut() {
            wallet.is_primary = false;
        }

        existing_profile
            .wallets
            .get_mut(&principal)
            .unwrap()
            .is_primary = true;

        let updated_profile = ProfileStore::update(caller(), existing_profile);

        ProfileResponse::from_result(updated_profile)
    }

    pub fn get_profile(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let profile_result = ProfileStore::get(principal);
        ProfileResponse::from_result(profile_result)
    }

    pub fn get_profiles(principals: Vec<Principal>) -> Vec<ProfileResponse> {
        let profiles_result = ProfileStore::get_many(principals);
        profiles_result
            .into_iter()
            .map(|profile| ProfileResponse::new(profile.0, profile.1))
            .collect()
    }

    pub fn query_profiles(query: String) -> Vec<ProfileResponse> {
        if query.len() < 3 {
            return vec![];
        }

        let _query = query.to_string().to_lowercase();

        let profiles_result =
            ProfileStore::filter(|_, p| p.username.to_lowercase().starts_with(&_query));
        profiles_result
            .into_iter()
            .map(|profile| ProfileResponse::new(profile.0, profile.1))
            .collect()
    }

    pub fn add_starred(subject: Subject) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = ProfileStore::get(caller())?;

        if existing_profile.starred.contains(&subject) {
            return Err(ApiError::duplicate().add_message("already starred"));
        }

        match subject {
            Subject::Group(id) => {
                let group_members = GroupMemberStore::get(id)
                    .map_or(MemberCollection::new(), |(_, members)| members);
                if !group_members.is_member(&caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only star joined groups")
                    );
                }
            }
            Subject::Event(id) => {
                let event_attendees = EventAttendeeStore::get(id)
                    .map_or(MemberCollection::new(), |(_, members)| members);
                if !event_attendees.is_member(&caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only star joined events")
                    );
                }
            }
            _ => return Err(ApiError::not_implemented().add_message("Subject type not supported")),
        };

        existing_profile.starred.push(subject);

        let updated_profile = ProfileStore::update(caller(), existing_profile);

        ProfileResponse::from_result(updated_profile)
    }

    pub fn remove_starred(subject: Subject) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = ProfileStore::get(caller())?;

        if !existing_profile.starred.contains(&subject) {
            return Err(ApiError::not_found().add_message("not starred"));
        }

        existing_profile.starred.retain(|s| s != &subject);

        let updated_profile = ProfileStore::update(caller(), existing_profile);

        ProfileResponse::from_result(updated_profile)
    }

    pub fn get_starred_by_subject(subject: SubjectType) -> Vec<u64> {
        if let Ok((_, profile)) = ProfileStore::get(caller()) {
            return profile
                .starred
                .iter()
                .filter(|s| s.get_type() == subject)
                .map(|s| *s.get_id())
                .collect();
        }
        vec![]
    }

    pub fn add_pinned(subject: Subject) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = ProfileStore::get(caller())?;

        if existing_profile.pinned.contains(&subject) {
            return Err(ApiError::duplicate().add_message("already pinned"));
        }

        match subject {
            Subject::Group(id) => {
                let group_members = GroupMemberStore::get(id)
                    .map_or(MemberCollection::new(), |(_, members)| members);
                if !group_members.is_member(&caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only pin joined groups")
                    );
                }
            }
            Subject::Event(id) => {
                let event_attendees = EventAttendeeStore::get(id)
                    .map_or(MemberCollection::new(), |(_, members)| members);
                if !event_attendees.is_member(&caller()) {
                    return Err(
                        ApiError::unauthorized().add_message("You can only pin joined events")
                    );
                }
            }
            _ => return Err(ApiError::not_implemented().add_message("Subject type not supported")),
        };

        existing_profile.pinned.push(subject);

        let updated_profile = ProfileStore::update(caller(), existing_profile);

        ProfileResponse::from_result(updated_profile)
    }

    pub fn remove_pinned(subject: Subject) -> Result<ProfileResponse, ApiError> {
        let (_, mut existing_profile) = ProfileStore::get(caller())?;

        if !existing_profile.pinned.contains(&subject) {
            return Err(ApiError::not_found().add_message("not pinned"));
        }

        existing_profile.pinned.retain(|s| s != &subject);

        let updated_profile = ProfileStore::update(caller(), existing_profile);

        ProfileResponse::from_result(updated_profile)
    }

    pub fn get_pinned_by_subject(subject: SubjectType) -> Vec<SubjectResponse> {
        if let Ok((_, profile)) = ProfileStore::get(caller()) {
            return profile
                .pinned
                .iter()
                .filter(|s| s.get_type() == subject)
                .map(Self::get_subject_response_by_subject)
                .collect();
        }
        vec![]
    }

    pub fn remove_friend(principal: Principal) -> Result<ProfileResponse, ApiError> {
        // Remove the friend from the caller profile
        let (_, mut caller_profile) = ProfileStore::get(caller())?;

        if !caller_profile.relations.contains_key(&principal) {
            return Err(ApiError::not_found().add_message("Friend does not exist"));
        }

        caller_profile.relations.remove(&principal);
        let updated_caller_profile = ProfileStore::update(caller(), caller_profile);

        let (_, mut friend_profile) = ProfileStore::get(principal)?;

        // Remove the caller from the friend profile
        if !friend_profile.relations.contains_key(&caller()) {
            return Err(ApiError::not_found().add_message("Friend does not exist"));
        }

        friend_profile.relations.remove(&caller());

        let _ = ProfileStore::update(principal, friend_profile);

        NotificationCalls::notification_remove_friend(principal, caller());
        ProfileResponse::from_result(updated_caller_profile)
    }

    pub fn block_user(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let (_, mut caller_profile) = ProfileStore::get(caller())?;

        caller_profile
            .relations
            .insert(principal, RelationType::Blocked.to_string());

        let updated_profile = ProfileStore::update(caller(), caller_profile);

        let (_, mut friend_profile) = ProfileStore::get(principal)?;

        // In case the friend has the caller as a friend, remove it
        if friend_profile.relations.contains_key(&caller()) {
            friend_profile.relations.remove(&caller());
            let _ = ProfileStore::update(principal, friend_profile);
        }

        ProfileResponse::from_result(updated_profile)
    }

    pub fn unblock_user(principal: Principal) -> Result<ProfileResponse, ApiError> {
        let (_, mut caller_profile) = ProfileStore::get(caller())?;

        if caller_profile
            .relations
            .get(&principal)
            .is_some_and(|data| data == &RelationType::Blocked.to_string())
        {
            caller_profile.relations.remove(&principal);
            let updated_profile = ProfileStore::update(caller(), caller_profile);
            return ProfileResponse::from_result(updated_profile);
        }

        Err(ApiError::not_found().add_message("User not blocked"))
    }

    pub fn get_relations(principal: Principal, relation_type: RelationType) -> Vec<Principal> {
        if let Ok((_, profile)) = ProfileStore::get(principal) {
            return profile
                .relations
                .iter()
                .filter_map(|(principal, r)| {
                    if r == &relation_type.to_string() {
                        Some(*principal)
                    } else {
                        None
                    }
                })
                .collect();
        }
        vec![]
    }

    pub fn get_relations_with_profiles(
        principal: Principal,
        relation_type: RelationType,
    ) -> Vec<ProfileResponse> {
        Self::get_profiles(ProfileCalls::get_relations(principal, relation_type))
    }

    // TODO: add logic to check the current version of these documents and add something to prompt the user to approve the latest version
    pub fn approve_code_of_conduct(version: u64) -> Result<bool, ApiError> {
        let (_, mut profile) = ProfileStore::get(caller())?;

        profile.code_of_conduct = Some(DocumentDetails::new(version, time()));
        let updated_profile = ProfileStore::update(caller(), profile);

        Ok(updated_profile.is_ok())
    }

    pub fn approve_privacy_policy(version: u64) -> Result<bool, ApiError> {
        let (_, mut profile) = ProfileStore::get(caller())?;

        profile.privacy_policy = Some(DocumentDetails::new(version, time()));
        let updated_profile = ProfileStore::update(caller(), profile);

        Ok(updated_profile.is_ok())
    }

    pub fn approve_terms_of_service(version: u64) -> Result<bool, ApiError> {
        let (_, mut profile) = ProfileStore::get(caller())?;

        profile.terms_of_service = Some(DocumentDetails::new(version, time()));
        let updated_profile = ProfileStore::update(caller(), profile);

        Ok(updated_profile.is_ok())
    }

    pub fn get_subject_response_by_subject(subject: &Subject) -> SubjectResponse {
        match subject.clone() {
            Subject::Group(id) => SubjectResponse::Group(GroupStore::get(id).ok()),
            Subject::Event(id) => SubjectResponse::Event(EventStore::get(id).ok()),
            Subject::Profile(id) => SubjectResponse::Profile(ProfileStore::get(id).ok()),
            Subject::Member(id) => SubjectResponse::Member(MemberStore::get(id).ok()),
            Subject::Attendee(id) => SubjectResponse::Attendee(AttendeeStore::get(id).ok()),
            Subject::None => SubjectResponse::None,
        }
    }
}

impl ProfileValidation {
    pub fn validate_post_profile(post_profile: &PostProfile) -> Result<(), ApiError> {
        let validator_fields = vec![
            ValidateField(
                ValidationType::StringLength(post_profile.username.clone(), 3, 64),
                "username".to_string(),
            ),
            ValidateField(
                ValidationType::StringLength(post_profile.display_name.clone(), 3, 64),
                "display_name".to_string(),
            ),
        ];

        Validator::new(validator_fields).validate()
    }

    pub fn validate_update_profile(update_profile: &UpdateProfile) -> Result<(), ApiError> {
        let mut validator_fields = vec![
            ValidateField(
                ValidationType::StringLength(update_profile.display_name.clone(), 3, 32),
                "display_name".to_string(),
            ),
            ValidateField(
                ValidationType::StringLength(update_profile.about.clone(), 0, 1000),
                "about".to_string(),
            ),
            ValidateField(
                ValidationType::StringLength(update_profile.city.clone(), 0, 64),
                "city".to_string(),
            ),
            ValidateField(
                ValidationType::StringLength(update_profile.country.clone(), 0, 64),
                "country".to_string(),
            ),
            ValidateField(
                ValidationType::StringLength(update_profile.website.clone(), 0, 200),
                "website".to_string(),
            ),
            ValidateField(
                ValidationType::Count(update_profile.skills.len(), 0, 50),
                "skills".to_string(),
            ),
            ValidateField(
                ValidationType::Count(update_profile.interests.len(), 0, 50),
                "interests".to_string(),
            ),
            ValidateField(
                ValidationType::Count(update_profile.causes.len(), 0, 50),
                "causes".to_string(),
            ),
        ];

        match update_profile.email.clone() {
            None => {}
            Some(_email) => validator_fields.push(ValidateField(
                ValidationType::Email(_email),
                "email".to_string(),
            )),
        }

        Validator::new(validator_fields).validate()
    }
}
