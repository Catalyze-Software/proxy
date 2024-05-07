use core::fmt;

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(
    CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub enum ApplicationRole {
    Owner,
    Admin,
    Moderator,
    Leader,
    #[default]
    Member,
    Watcher,
    Guest,
    Blocked,
    Banned,
}

impl fmt::Display for ApplicationRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApplicationRole::*;
        match self {
            Owner => write!(f, "Owner"),
            Admin => write!(f, "Admin"),
            Moderator => write!(f, "Moderator"),
            Leader => write!(f, "Leader"),
            Member => write!(f, "Member"),
            Watcher => write!(f, "OwnerWatcher"),
            Guest => write!(f, "Guest"),
            Blocked => write!(f, "Blocked"),
            Banned => write!(f, "Banned"),
        }
    }
}
