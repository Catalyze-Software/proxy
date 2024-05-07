use std::fmt;

use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Default, CandidType, Serialize, Deserialize, Debug)]
pub struct Permission {
    name: String,
    protected: bool,
    actions: PermissionActions,
}

impl From<PostPermission> for Permission {
    fn from(post_permission: PostPermission) -> Self {
        Self {
            name: post_permission.name,
            protected: false,
            actions: post_permission.actions,
        }
    }
}

impl Permission {
    pub fn new(name: String, protected: bool, actions: PermissionActions) -> Self {
        Self {
            name: name.to_string(),
            protected,
            actions,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn actions(&self) -> PermissionActions {
        self.actions
    }
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct PostPermission {
    name: String,
    actions: PermissionActions,
}

#[derive(Clone, Copy, Default, CandidType, Serialize, Deserialize, Debug)]
pub struct PermissionActions {
    write: bool,
    read: bool,
    edit: bool,
    delete: bool,
}

impl PermissionActions {
    pub fn new(write: bool, read: bool, edit: bool, delete: bool) -> Self {
        Self {
            write,
            read,
            edit,
            delete,
        }
    }

    pub fn write(&self) -> bool {
        self.write
    }

    pub fn read(&self) -> bool {
        self.read
    }

    pub fn edit(&self) -> bool {
        self.edit
    }

    pub fn delete(&self) -> bool {
        self.delete
    }
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub enum PermissionActionType {
    Write,
    Read,
    Edit,
    Delete,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum PermissionType {
    // Canister
    Task(Option<PermissionActions>),
    Event(Option<PermissionActions>),
    Group(Option<PermissionActions>),
    Member(Option<PermissionActions>),
    Invite(Option<PermissionActions>),
    Attendee(Option<PermissionActions>),

    // Frontend chat
    ChatNotification(Option<PermissionActions>),
    ChatMessage(Option<PermissionActions>),
    ChatChannel(Option<PermissionActions>),
    ChatEmbed(Option<PermissionActions>),
    ChatFiles(Option<PermissionActions>),
    ChatMention(Option<PermissionActions>),
    ChatPolls(Option<PermissionActions>),
}

impl fmt::Display for PermissionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PermissionType::*;
        match &self {
            Task(_) => write!(f, "task"),
            Event(_) => write!(f, "event"),
            Group(_) => write!(f, "group"),
            Member(_) => write!(f, "member"),
            Invite(_) => write!(f, "invite"),
            Attendee(_) => write!(f, "attendee"),
            ChatNotification(_) => write!(f, "chat_notification"),
            ChatMessage(_) => write!(f, "chat_message"),
            ChatChannel(_) => write!(f, "chat_channel"),
            ChatEmbed(_) => write!(f, "chat_embed"),
            ChatFiles(_) => write!(f, "chat_files"),
            ChatMention(_) => write!(f, "chat_mention"),
            ChatPolls(_) => write!(f, "chat_polls"),
        }
    }
}
