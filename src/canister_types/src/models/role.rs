use candid::{CandidType, Deserialize};
use serde::Serialize;

use super::permission::{Permission, PostPermission};

#[derive(Clone, CandidType, Default, Serialize, Deserialize, Debug)]
pub struct Role {
    pub name: String,
    pub protected: bool,
    pub permissions: Vec<Permission>,
    pub color: String,
    pub index: Option<u64>,
}

impl From<PostRole> for Role {
    fn from(post_role: PostRole) -> Self {
        Self {
            name: post_role.name,
            protected: false,
            permissions: post_role
                .permissions
                .into_iter()
                .map(Permission::from)
                .collect(),
            color: post_role.color,
            index: post_role.index,
        }
    }
}

impl Role {
    pub fn new(
        name: String,
        protected: bool,
        permissions: Vec<Permission>,
        color: String,
        index: Option<u64>,
    ) -> Self {
        Self {
            name,
            protected,
            permissions,
            color,
            index,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn permissions(&self) -> Vec<Permission> {
        self.permissions.clone()
    }
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct PostRole {
    pub name: String,
    pub permissions: Vec<PostPermission>,
    pub color: String,
    pub index: Option<u64>,
}
