use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    misc::role_misc::default_roles,
    models::{
        asset::Asset, date_range::DateRange, location::Location, privacy::Privacy, role::Role,
        sort_direction::SortDirection,
    },
};

use super::{
    api_error::ApiError,
    boosted::Boost,
    member::{InviteMemberResponse, JoinedMemberResponse},
    member_collection::MemberCollection,
    permission::Permission,
    relation_type::RelationType,
};

impl_storable_for!(Group);

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: Privacy,
    pub owner: Principal,
    pub created_by: Principal,
    pub matrix_space_id: String,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
    pub privacy_gated_type_amount: Option<u64>,
    pub roles: Vec<Role>,
    pub is_deleted: bool,
    pub notification_id: Option<u64>,
    pub special_members: HashMap<Principal, String>,
    pub wallets: HashMap<Principal, String>,
    pub updated_on: u64,
    pub created_on: u64,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            website: Default::default(),
            location: Default::default(),
            privacy: Default::default(),
            owner: Principal::anonymous(),
            created_by: Principal::anonymous(),
            matrix_space_id: Default::default(),
            image: Default::default(),
            banner_image: Default::default(),
            tags: Default::default(),
            wallets: Default::default(),
            roles: Vec::default(),
            is_deleted: Default::default(),
            notification_id: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            privacy_gated_type_amount: Default::default(),
            special_members: Default::default(),
        }
    }
}

impl Group {
    pub fn from(group: PostGroup) -> Self {
        Self {
            name: group.name,
            description: group.description,
            website: group.website,
            location: group.location,
            privacy: group.privacy,
            owner: caller(),
            created_by: caller(),
            matrix_space_id: group.matrix_space_id,
            image: group.image,
            banner_image: group.banner_image,
            tags: group.tags,
            wallets: Default::default(),
            roles: Vec::default(),
            is_deleted: false,
            notification_id: None,
            updated_on: time(),
            created_on: time(),
            privacy_gated_type_amount: group.privacy_gated_type_amount,
            special_members: HashMap::default(),
        }
    }

    pub fn update(&mut self, group: UpdateGroup) {
        self.name = group.name;
        self.description = group.description;
        self.website = group.website;
        self.location = group.location;
        self.privacy = group.privacy;
        self.image = group.image;
        self.banner_image = group.banner_image;
        self.tags = group.tags;
        self.privacy_gated_type_amount = group.privacy_gated_type_amount;
        self.updated_on = time();
    }

    pub fn set_owner(&mut self, owner: Principal) -> Self {
        self.owner = owner;
        self.updated_on = time();
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = time();
        self.clone()
    }

    pub fn get_roles(&self) -> Vec<Role> {
        // set the default roles
        let mut roles = self.roles.clone();

        // append the custom roles stored on the group
        roles.append(&mut default_roles());
        roles
    }

    pub fn get_role_permissions(&self, role: String) -> Vec<Permission> {
        let roles = self.get_roles();
        let role = roles.iter().find(|r| r.name == role);
        if let Some(role) = role {
            return role.permissions.clone();
        }
        vec![]
    }

    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }

    pub fn add_special_member(&mut self, member: Principal, relation: RelationType) {
        self.special_members.insert(member, relation.to_string());
    }

    pub fn remove_special_member_from_group(&mut self, member: Principal) {
        self.special_members.remove(&member);
    }

    pub fn is_banned_member(&self, member: Principal) -> bool {
        self.special_members
            .get(&member)
            .map(|relation| relation == &RelationType::Blocked.to_string())
            .unwrap_or(false)
    }
}

#[derive(Clone, CandidType, Deserialize)]
pub struct PostGroup {
    pub name: String,
    pub description: String,
    pub website: String,
    pub matrix_space_id: String,
    pub location: Location,
    pub privacy: Privacy,
    pub privacy_gated_type_amount: Option<u64>,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
}

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct UpdateGroup {
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: Privacy,
    pub image: Asset,
    pub privacy_gated_type_amount: Option<u64>,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupCallerData {
    pub joined: Option<JoinedMemberResponse>,
    pub invite: Option<InviteMemberResponse>,
    pub is_starred: bool,
    pub is_pinned: bool,
}

impl GroupCallerData {
    pub fn new(
        joined: Option<JoinedMemberResponse>,
        invite: Option<InviteMemberResponse>,
        is_starred: bool,
        is_pinned: bool,
    ) -> Self {
        Self {
            joined,
            invite,
            is_starred,
            is_pinned,
        }
    }
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GroupResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub website: String,
    pub location: Location,
    pub privacy: Privacy,
    pub created_by: Principal,
    pub owner: Principal,
    pub matrix_space_id: String,
    pub image: Asset,
    pub banner_image: Asset,
    pub tags: Vec<u32>,
    pub roles: Vec<Role>,
    pub wallets: Vec<(Principal, String)>,
    pub is_deleted: bool,
    pub privacy_gated_type_amount: Option<u64>,
    pub updated_on: u64,
    pub created_on: u64,
    pub boosted: Option<Boost>,
    pub events_count: u64,
    pub members_count: u64,
    pub caller_data: Option<GroupCallerData>,
}

impl GroupResponse {
    pub fn new(
        id: u64,
        group: Group,
        boosted: Option<Boost>,
        events_count: u64,
        members_count: u64,
        caller_data: Option<GroupCallerData>,
    ) -> Self {
        let mut roles = default_roles();
        roles.append(&mut group.roles.clone());
        Self {
            id,
            name: group.name,
            description: group.description,
            website: group.website,
            location: group.location,
            privacy: group.privacy,
            created_by: group.created_by,
            owner: group.owner,
            matrix_space_id: group.matrix_space_id,
            image: group.image,
            banner_image: group.banner_image,
            tags: group.tags,
            roles,
            wallets: group.wallets.into_iter().collect(),
            is_deleted: group.is_deleted,
            caller_data,
            privacy_gated_type_amount: group.privacy_gated_type_amount,
            boosted,
            updated_on: group.updated_on,
            created_on: group.created_on,
            events_count,
            members_count,
        }
    }

    pub fn from_result(
        group_result: Result<(u64, Group), Box<ApiError>>,
        boosted: Option<Boost>,
        events_count: u64,
        members_count: u64,
        caller_data: Option<GroupCallerData>,
    ) -> Result<Self, Box<ApiError>> {
        match group_result {
            Err(err) => Err(err),
            Ok((id, group)) => Ok(Self::new(
                id,
                group,
                boosted,
                events_count,
                members_count,
                caller_data,
            )),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum GroupSort {
    Name(SortDirection),
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
    MemberCount(SortDirection),
}

impl Default for GroupSort {
    fn default() -> Self {
        GroupSort::CreatedOn(SortDirection::Asc)
    }
}

impl GroupSort {
    pub fn sort(
        &self,
        groups: HashMap<u64, Group>,
        group_members: HashMap<u64, MemberCollection>,
    ) -> Vec<(u64, Group)> {
        let mut groups: Vec<(u64, Group)> = groups.into_iter().collect();
        use GroupSort::*;
        use SortDirection::*;
        match self {
            Name(Asc) => {
                groups.sort_by(|(_, a), (_, b)| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            }
            Name(Desc) => {
                groups.sort_by(|(_, a), (_, b)| b.name.to_lowercase().cmp(&a.name.to_lowercase()))
            }
            CreatedOn(Asc) => groups.sort_by(|(_, a), (_, b)| a.created_on.cmp(&b.created_on)),
            CreatedOn(Desc) => groups.sort_by(|(_, a), (_, b)| b.created_on.cmp(&a.created_on)),
            UpdatedOn(Asc) => groups.sort_by(|(_, a), (_, b)| a.updated_on.cmp(&b.updated_on)),
            UpdatedOn(Desc) => groups.sort_by(|(_, a), (_, b)| b.updated_on.cmp(&a.updated_on)),
            MemberCount(Asc) => groups.sort_by(|(a_id, a), (b_id, b)| {
                let a_members = group_members
                    .get(a_id)
                    .map(|m| m.get_member_count())
                    .unwrap_or_default();
                let b_members = group_members
                    .get(b_id)
                    .map(|m| m.get_member_count())
                    .unwrap_or_default();
                a_members.cmp(&b_members)
            }),
            MemberCount(Desc) => groups.sort_by(|(a_id, a), (b_id, b)| {
                let a_members = group_members
                    .get(a_id)
                    .map(|m| m.get_member_count())
                    .unwrap_or_default();
                let b_members = group_members
                    .get(b_id)
                    .map(|m| m.get_member_count())
                    .unwrap_or_default();
                b_members.cmp(&a_members)
            }),
        }
        groups
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct GroupsCount {
    pub total: u64,
    pub joined: u64,
    pub invited: u64,
    pub starred: u64,
    pub new: u64,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub enum GroupFilter {
    #[default]
    None,
    Name(String),
    Owner(Principal),
    Ids(Vec<u64>),
    Tag(u32),
    UpdatedOn(DateRange),
    CreatedOn(DateRange),
}

impl GroupFilter {
    pub fn is_match(&self, id: &u64, group: &Group) -> bool {
        match self {
            GroupFilter::None => true,
            GroupFilter::Name(name) => group.name.to_lowercase().contains(&name.to_lowercase()),
            GroupFilter::Owner(owner) => group.owner == *owner,
            GroupFilter::Ids(ids) => ids.contains(id),
            GroupFilter::Tag(tag) => group.tags.contains(tag),
            GroupFilter::UpdatedOn(range) => {
                if range.end_date() > 0 {
                    range.is_within(group.updated_on)
                } else {
                    range.is_after(group.updated_on)
                }
            }
            GroupFilter::CreatedOn(range) => {
                if range.end_date() > 0 {
                    range.is_within(group.updated_on)
                } else {
                    range.is_after(group.updated_on)
                }
            }
        }
    }
}
