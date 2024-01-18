use crate::models::{
    permission::{Permission, PermissionActions, PermissionType},
    role::Role,
};

pub fn read_only_permission_actions() -> PermissionActions {
    PermissionActions::new(false, true, false, false)
}

pub fn add_edit_permission_actions() -> PermissionActions {
    PermissionActions::new(true, true, true, false)
}

pub fn add_permissions_actions() -> PermissionActions {
    PermissionActions::new(true, true, false, false)
}

pub fn all_access_permission_actions() -> PermissionActions {
    PermissionActions::new(true, true, true, true)
}

pub fn default_roles() -> Vec<Role> {
    vec![
        Role::new("owner", true, all_access_permissions(), "#FFB800", None),
        Role::new("admin", true, all_access_permissions(), "#F56E00", None),
        Role::new("moderator", true, moderator_permissions(), "#06E143", None),
        Role::new("member", true, read_only_permissions(), "#7A9CF2", None),
    ]
}

pub fn moderator_permissions() -> Vec<Permission> {
    use PermissionType::*;
    vec![
        create_permission(&Task(Some(all_access_permission_actions()))),
        create_permission(&Event(Some(add_permissions_actions()))),
        create_permission(&Attendee(Some(add_permissions_actions()))),
        create_permission(&Group(Some(read_only_permission_actions()))),
        create_permission(&Invite(Some(all_access_permission_actions()))),
        create_permission(&Member(Some(all_access_permission_actions()))),
        create_permission(&ChatNotification(Some(all_access_permission_actions()))),
        create_permission(&ChatMessage(Some(all_access_permission_actions()))),
        create_permission(&ChatChannel(Some(all_access_permission_actions()))),
        create_permission(&ChatEmbed(Some(all_access_permission_actions()))),
        create_permission(&ChatFiles(Some(all_access_permission_actions()))),
        create_permission(&ChatMention(Some(all_access_permission_actions()))),
        create_permission(&ChatPolls(Some(all_access_permission_actions()))),
    ]
}

fn all_access_permissions() -> Vec<Permission> {
    use PermissionType::*;
    vec![
        create_permission(&Task(Some(all_access_permission_actions()))),
        create_permission(&Event(Some(all_access_permission_actions()))),
        create_permission(&Attendee(Some(all_access_permission_actions()))),
        create_permission(&Group(Some(all_access_permission_actions()))),
        create_permission(&Invite(Some(all_access_permission_actions()))),
        create_permission(&Member(Some(all_access_permission_actions()))),
        create_permission(&ChatNotification(Some(all_access_permission_actions()))),
        create_permission(&ChatMessage(Some(all_access_permission_actions()))),
        create_permission(&ChatChannel(Some(all_access_permission_actions()))),
        create_permission(&ChatEmbed(Some(all_access_permission_actions()))),
        create_permission(&ChatFiles(Some(all_access_permission_actions()))),
        create_permission(&ChatMention(Some(all_access_permission_actions()))),
        create_permission(&ChatPolls(Some(all_access_permission_actions()))),
    ]
}

fn read_only_permissions() -> Vec<Permission> {
    use PermissionType::*;
    vec![
        create_permission(&Task(None)),
        create_permission(&Event(None)),
        create_permission(&Attendee(None)),
        create_permission(&Group(None)),
        create_permission(&Invite(None)),
        create_permission(&Member(None)),
        create_permission(&ChatNotification(None)),
        create_permission(&ChatMessage(None)),
        create_permission(&ChatChannel(None)),
        create_permission(&ChatEmbed(None)),
        create_permission(&ChatFiles(None)),
        create_permission(&ChatMention(None)),
        create_permission(&ChatPolls(None)),
    ]
}

pub fn create_permission(permission: &PermissionType) -> Permission {
    use PermissionType::*;
    match permission {
        Task(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        Event(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        Group(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        Invite(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        Member(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        ChatNotification(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        ChatMessage(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        ChatChannel(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        ChatEmbed(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        ChatFiles(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        ChatMention(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        ChatPolls(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
        Attendee(actions) => Permission::new(
            permission.to_string(),
            true,
            actions.unwrap_or(read_only_permission_actions()),
        ),
    }
}
