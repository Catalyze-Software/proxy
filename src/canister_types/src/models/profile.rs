use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    models::{
        application_role::ApplicationRole, asset::Asset, date_range::DateRange,
        sort_direction::SortDirection,
    },
};

use super::{
    api_error::ApiError,
    document_details::DocumentDetails,
    profile_privacy::ProfilePrivacy,
    wallet::{Wallet, WalletResponse},
};

impl_storable_for!(Profile);

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Profile {
    pub principal: Principal,
    pub member_identifier: Principal,
    pub username: String,
    pub display_name: String,
    pub application_role: ApplicationRole,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub about: String,
    pub email: String,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub code_of_conduct: DocumentDetails,
    pub privacy_policy: Option<DocumentDetails>,
    pub terms_of_service: Option<DocumentDetails>,
    pub wallets: HashMap<Principal, Wallet>,
    pub starred: HashMap<Principal, String>,
    pub relations: HashMap<Principal, String>,
    pub extra: String,
    pub notification_id: Option<u64>,
    pub updated_on: u64,
    pub created_on: u64,
}

impl Profile {
    pub fn default() -> Self {
        Self {
            principal: Principal::anonymous(),
            member_identifier: Principal::anonymous(),
            username: Default::default(),
            display_name: Default::default(),
            application_role: Default::default(),
            first_name: Default::default(),
            last_name: Default::default(),
            privacy: Default::default(),
            about: Default::default(),
            email: Default::default(),
            date_of_birth: Default::default(),
            city: Default::default(),
            state_or_province: Default::default(),
            country: Default::default(),
            profile_image: Default::default(),
            banner_image: Default::default(),
            skills: Default::default(),
            interests: Default::default(),
            causes: Default::default(),
            website: Default::default(),
            code_of_conduct: Default::default(),
            wallets: Default::default(),
            starred: Default::default(),
            relations: Default::default(),
            extra: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            notification_id: Default::default(),
            privacy_policy: None,
            terms_of_service: None,
        }
    }

    pub fn update(self, profile: UpdateProfile) -> Self {
        Self {
            principal: self.principal,
            username: self.username,
            display_name: profile.display_name,
            application_role: self.application_role,
            first_name: profile.first_name,
            last_name: profile.last_name,
            privacy: profile.privacy,
            about: profile.about,
            email: profile.email.unwrap_or("".to_string()),
            date_of_birth: profile.date_of_birth,
            city: profile.city,
            state_or_province: profile.state_or_province,
            country: profile.country,
            profile_image: profile.profile_image,
            banner_image: profile.banner_image,
            skills: profile.skills,
            interests: profile.interests,
            causes: profile.causes,
            website: profile.website,
            wallets: self.wallets,
            starred: self.starred,
            relations: self.relations,
            code_of_conduct: self.code_of_conduct,
            extra: profile.extra,
            updated_on: time(),
            notification_id: self.notification_id,
            created_on: self.created_on,
            member_identifier: self.member_identifier,
            privacy_policy: self.privacy_policy,
            terms_of_service: self.terms_of_service,
        }
    }

    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }
}

impl From<PostProfile> for Profile {
    fn from(profile: PostProfile) -> Self {
        Self {
            principal: caller(),
            username: profile.username,
            display_name: profile.display_name,
            application_role: ApplicationRole::default(),
            first_name: profile.first_name,
            last_name: profile.last_name,
            privacy: profile.privacy,
            about: "".to_string(),
            email: "".to_string(),
            date_of_birth: 0,
            city: "".to_string(),
            state_or_province: "".to_string(),
            country: "".to_string(),
            profile_image: Asset::None,
            banner_image: Asset::None,
            skills: vec![],
            interests: vec![],
            causes: vec![],
            website: "".to_string(),
            wallets: HashMap::new(),
            starred: HashMap::new(),
            relations: HashMap::new(),
            code_of_conduct: DocumentDetails::new(0, 0),
            extra: profile.extra,
            updated_on: time(),
            created_on: time(),
            notification_id: None,
            member_identifier: Principal::anonymous(),
            privacy_policy: None,
            terms_of_service: None,
        }
    }
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct PostProfile {
    pub username: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub extra: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UpdateProfile {
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub about: String,
    pub email: Option<String>,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub extra: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ProfileResponse {
    pub identifier: Principal,
    pub principal: Principal,
    pub member_identifier: Principal,
    pub username: String,
    pub display_name: String,
    pub application_role: ApplicationRole,
    pub first_name: String,
    pub last_name: String,
    pub privacy: ProfilePrivacy,
    pub about: String,
    pub email: String,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: Asset,
    pub banner_image: Asset,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub code_of_conduct: DocumentDetails,
    pub privacy_policy: Option<DocumentDetails>,
    pub terms_of_service: Option<DocumentDetails>,
    pub wallets: Vec<WalletResponse>,
    pub extra: String,
    pub updated_on: u64,
    pub created_on: u64,
}

impl ProfileResponse {
    pub fn new(id: Principal, profile: Profile) -> Self {
        let wallets = profile
            .wallets
            .into_iter()
            .map(|(principal, wallet)| WalletResponse {
                provider: wallet.provider,
                principal,
                is_primary: wallet.is_primary,
            })
            .collect();

        Self {
            username: profile.username,
            display_name: profile.display_name,
            about: profile.about,
            city: profile.city,
            country: profile.country,
            website: profile.website,
            skills: profile.skills,
            interests: profile.interests,
            causes: profile.causes,
            email: profile.email,
            identifier: profile.principal,
            principal: profile.principal,
            member_identifier: profile.member_identifier,
            application_role: profile.application_role,
            first_name: profile.first_name,
            last_name: profile.last_name,
            privacy: profile.privacy,
            date_of_birth: profile.date_of_birth,
            state_or_province: profile.state_or_province,
            profile_image: profile.profile_image,
            banner_image: profile.banner_image,
            code_of_conduct: profile.code_of_conduct,
            privacy_policy: profile.privacy_policy,
            terms_of_service: profile.terms_of_service,
            wallets,
            extra: profile.extra,
            updated_on: profile.updated_on,
            created_on: profile.created_on,
        }
    }

    pub fn from_result(
        profile_result: Result<(Principal, Profile), ApiError>,
    ) -> Result<Self, ApiError> {
        match profile_result {
            Err(err) => Err(err),
            Ok((_, profile)) => {
                let wallets = profile
                    .wallets
                    .into_iter()
                    .map(|(principal, wallet)| WalletResponse {
                        provider: wallet.provider,
                        principal,
                        is_primary: wallet.is_primary,
                    })
                    .collect();

                let result = Self {
                    username: profile.username,
                    display_name: profile.display_name,
                    about: profile.about,
                    city: profile.city,
                    country: profile.country,
                    website: profile.website,
                    skills: profile.skills,
                    interests: profile.interests,
                    causes: profile.causes,
                    email: profile.email,
                    identifier: profile.principal,
                    principal: profile.principal,
                    member_identifier: profile.member_identifier,
                    application_role: profile.application_role,
                    first_name: profile.first_name,
                    last_name: profile.last_name,
                    privacy: profile.privacy,
                    date_of_birth: profile.date_of_birth,
                    state_or_province: profile.state_or_province,
                    profile_image: profile.profile_image,
                    banner_image: profile.banner_image,
                    code_of_conduct: profile.code_of_conduct,
                    privacy_policy: profile.privacy_policy,
                    terms_of_service: profile.terms_of_service,
                    wallets,
                    extra: profile.extra,
                    updated_on: profile.updated_on,
                    created_on: profile.created_on,
                };
                Ok(result)
            }
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProfileSort {
    Username(SortDirection),
    DisplayName(SortDirection),
    FirstName(SortDirection),
    LastName(SortDirection),
    Email(SortDirection),
    City(SortDirection),
    StateOrProvince(SortDirection),
    Country(SortDirection),
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProfileFilter {
    Username(String),
    DisplayName(String),
    FirstName(String),
    LastName(String),
    Email(String),
    City(String),
    StateOrProvince(String),
    Country(String),
    UpdatedOn(DateRange),
    Skill(u32),
    Interest(u32),
    Cause(u32),
    CreatedOn(DateRange),
}
