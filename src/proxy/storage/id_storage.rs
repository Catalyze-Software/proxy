use super::storage_api::{StaticStorageRef, Storage, StorageRef, IDS, IDS_MEMORY_ID};
use canister_types::models::api_error::ApiError;
use ic_stable_structures::{memory_manager::MemoryId, Storable};

pub const ID_KIND_BOOSTED: &str = "boosted";
pub const ID_KIND_EVENT_ATTENDEES: &str = "event_attendees";
pub const ID_KIND_EVENTS: &str = "events";
pub const ID_KIND_FRIEND_REQUESTS: &str = "friend_requests";
pub const ID_KIND_GROUP_EVENTS: &str = "group_events";
pub const ID_KIND_GROUP_MEMBERS: &str = "group_members";
pub const ID_KIND_GROUPS: &str = "groups";
pub const ID_KIND_LOGS: &str = "logs";
pub const ID_KIND_NOTIFICATIONS: &str = "notifications";
pub const ID_KIND_REPORTS: &str = "reports";
pub const ID_KIND_TAGS: &str = "tags";
pub const ID_KIND_CATEGORIES: &str = "categories";
pub const ID_KIND_REWARDS_BUFFER: &str = "rewards_buffer";
pub const ID_KIND_SKILLS: &str = "skills";

#[derive(Debug, Clone)]
pub enum IDKind {
    BoostedKind,
    EventAttendees,
    Events,
    FriendRequests,
    GroupEvents,
    GroupMembers,
    Groups,
    Logs,
    Notifications,
    Reports,
    Tags,
    Categories,
    Skills,
    RewardBuffer,
}

impl std::fmt::Display for IDKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IDKind::BoostedKind => write!(f, "{ID_KIND_BOOSTED}"),
            IDKind::EventAttendees => write!(f, "{ID_KIND_EVENT_ATTENDEES}"),
            IDKind::Events => write!(f, "{ID_KIND_EVENTS}"),
            IDKind::FriendRequests => write!(f, "{ID_KIND_FRIEND_REQUESTS}"),
            IDKind::GroupEvents => write!(f, "{ID_KIND_GROUP_EVENTS}"),
            IDKind::GroupMembers => write!(f, "{ID_KIND_GROUP_MEMBERS}"),
            IDKind::Groups => write!(f, "{ID_KIND_GROUPS}"),
            IDKind::Logs => write!(f, "{ID_KIND_LOGS}"),
            IDKind::Notifications => write!(f, "{ID_KIND_NOTIFICATIONS}"),
            IDKind::Reports => write!(f, "{ID_KIND_REPORTS}"),
            IDKind::Tags => write!(f, "{ID_KIND_TAGS}"),
            IDKind::Categories => write!(f, "{ID_KIND_CATEGORIES}"),
            IDKind::Skills => write!(f, "{ID_KIND_SKILLS}"),
            IDKind::RewardBuffer => write!(f, "{ID_KIND_REWARDS_BUFFER}"),
        }
    }
}

impl std::str::FromStr for IDKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ID_KIND_BOOSTED => Ok(IDKind::BoostedKind),
            ID_KIND_EVENT_ATTENDEES => Ok(IDKind::EventAttendees),
            ID_KIND_EVENTS => Ok(IDKind::Events),
            ID_KIND_FRIEND_REQUESTS => Ok(IDKind::FriendRequests),
            ID_KIND_GROUP_EVENTS => Ok(IDKind::GroupEvents),
            ID_KIND_GROUP_MEMBERS => Ok(IDKind::GroupMembers),
            ID_KIND_GROUPS => Ok(IDKind::Groups),
            ID_KIND_LOGS => Ok(IDKind::Logs),
            ID_KIND_NOTIFICATIONS => Ok(IDKind::Notifications),
            ID_KIND_REPORTS => Ok(IDKind::Reports),
            ID_KIND_TAGS => Ok(IDKind::Tags),
            ID_KIND_CATEGORIES => Ok(IDKind::Categories),
            ID_KIND_SKILLS => Ok(IDKind::Skills),
            ID_KIND_REWARDS_BUFFER => Ok(IDKind::RewardBuffer),
            _ => Err(format!("Unknown IDKind: {s}")),
        }
    }
}

pub struct IDStore;

impl Storage<String, u64> for IDStore {
    const NAME: &'static str = "ids";

    fn storage() -> StaticStorageRef<String, u64> {
        &IDS
    }

    fn memory_id() -> MemoryId {
        IDS_MEMORY_ID
    }
}

impl IDStore {
    pub fn next(kind: &str) -> Result<u64, ApiError> {
        let kind = kind.parse::<IDKind>().map_err(|e| {
            ApiError::unexpected()
                .add_message(&format!("Failed to perform storage action, error: {e}"))
        })?;

        Self::increment(kind)
    }

    pub fn get_all() -> Vec<(String, u64)> {
        Self::storage().with(|data| data.borrow().iter().collect())
    }

    // Use the old id if the id store is empty (only needed for existing data)
    fn increment(k: IDKind) -> Result<u64, ApiError> {
        let id = Self::get(k.clone()).unwrap_or_else(|| Self::get_last(k.clone())) + 1;

        Self::storage().with(|data| {
            data.borrow_mut().insert(k.to_string(), id);
            Ok(id)
        })
    }

    fn get(kind: IDKind) -> Option<u64> {
        Self::storage().with(|data| data.borrow().get(&kind.to_string()))
    }

    fn get_last(kind: IDKind) -> u64 {
        match kind {
            IDKind::BoostedKind => super::BoostedStore::storage().with(last_key),
            IDKind::EventAttendees => super::EventAttendeeStore::storage().with(last_key),
            IDKind::Events => super::EventStore::storage().with(last_key),
            IDKind::FriendRequests => super::FriendRequestStore::storage().with(last_key),
            IDKind::GroupEvents => super::GroupEventsStore::storage().with(last_key),
            IDKind::GroupMembers => super::GroupMemberStore::storage().with(last_key),
            IDKind::Groups => super::GroupStore::storage().with(last_key),
            IDKind::Logs => super::LoggerStore::storage().with(last_key),
            IDKind::Notifications => super::NotificationStore::storage().with(last_key),
            IDKind::Reports => super::ReportStore::storage().with(last_key),
            IDKind::Tags => super::TagStore::storage().with(last_key),
            IDKind::Categories => super::CategoryStore::storage().with(last_key),
            IDKind::Skills => super::SkillStore::storage().with(last_key),
            IDKind::RewardBuffer => super::RewardBufferStore::storage().with(last_key),
        }
    }
}

fn last_key<T: Storable>(data: &StorageRef<u64, T>) -> u64 {
    data.borrow().last_key_value().map(|(k, _)| k).unwrap_or(1)
}
