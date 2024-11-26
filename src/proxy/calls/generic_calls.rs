use crate::{
    helpers::guards::{is_developer, is_prod_developer},
    logic::{boost_logic::BoostCalls, id_logic::IDLogic, websocket_logic::Websocket},
    storage::{
        reward_canister_storage::RewardCanisterStorage, storage_api::StorageQueryable,
        AttendeeStore, BoostedStore, CellStorage, EventAttendeeStore, EventStore,
        FriendRequestStore, GroupEventsStore, GroupMemberStore, GroupStore, HistoryCanisterStorage,
        LoggerStore, MemberStore, NotificationStore, ProfileStore, ReportStore, RewardBufferStore,
        RewardTimerStore, StorageUpdateable, UserNotificationStore,
    },
};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    http_types::{HttpRequest, HttpResponse},
    icrc28_trusted_origin::Icrc28TrustedOriginsResponse,
};
use ic_cdk::{
    api::{
        canister_balance128,
        management_canister::main::{create_canister, CanisterSettings, CreateCanisterArgument},
    },
    id, init, post_upgrade, pre_upgrade, query, update,
};

#[post_upgrade]
pub fn post_upgrade() {
    Websocket::init();
    RewardTimerStore::start();
    BoostCalls::start_timers_after_upgrade();
}

#[pre_upgrade]
pub fn pre_upgrade() {}

#[init]
pub fn init() {
    Websocket::init();
    RewardTimerStore::start();
}

#[query]
fn icts_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

#[query]
fn icts_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[update]
fn icrc28_trusted_origins() -> Icrc28TrustedOriginsResponse {
    let trusted_origins = vec![
        String::from("https://www.catalyze.one"),
        String::from("https://catalyze.live"),
        String::from("https://h4xrq-fiaaa-aaaal-ajecq-cai.icp0.io"),
    ];

    Icrc28TrustedOriginsResponse { trusted_origins }
}

#[query(guard = "is_developer")]
pub fn _dev_check_member_sync(
    principal: Principal,
    group_id: u64,
) -> ((String, bool), (String, bool)) {
    let mut member_store_check: (String, bool) = ("MemberStore".to_string(), false);
    let mut group_member_store_check: (String, bool) = ("GroupMemberStore".to_string(), false);

    member_store_check.1 = MemberStore::get(principal).is_ok();
    let group_members = GroupMemberStore::get(group_id);
    group_member_store_check.1 = match group_members {
        Ok((_, group_members)) => group_members.is_member(&principal),
        Err(_) => false,
    };

    (member_store_check, group_member_store_check)
}

#[query(guard = "is_developer")]
pub fn _dev_check_attendees_sync(
    principal: Principal,
    event_id: u64,
) -> ((String, bool), (String, bool)) {
    let mut attendee_store_check: (String, bool) = ("AttendeeStore".to_string(), false);
    let mut event_attendee_store_check: (String, bool) = ("EventAttendeeStore".to_string(), false);

    attendee_store_check.1 = AttendeeStore::get(principal).is_ok();
    let group_members = GroupMemberStore::get(event_id);
    event_attendee_store_check.1 = match group_members {
        Ok((_, group_members)) => group_members.is_member(&principal),
        Err(_) => false,
    };

    (attendee_store_check, event_attendee_store_check)
}

#[query(guard = "is_developer")]
pub fn _dev_check_events_sync(event_id: u64, group_id: u64) -> ((String, bool), (String, bool)) {
    let mut event_store_check: (String, bool) = ("EventStore".to_string(), false);
    let mut group_event_store_check: (String, bool) = ("GroupEventStore".to_string(), false);

    event_store_check.1 = EventStore::get(event_id).is_ok();
    let group_events = GroupEventsStore::get(group_id);
    group_event_store_check.1 = match group_events {
        Ok((_, group_events)) => group_events.has_event(&event_id),
        Err(_) => false,
    };

    (event_store_check, group_event_store_check)
}

#[update(guard = "is_developer")]
pub fn _dev_clear_notifications() {
    UserNotificationStore::clear();
    NotificationStore::clear();
}

#[update(guard = "is_prod_developer")]
async fn _dev_create_canister(controllers: Vec<Principal>) -> Result<Principal, String> {
    let arg = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            wasm_memory_limit: None,
            log_visibility: None,
        }),
    };

    let current_cycles = canister_balance128();
    if current_cycles < 10_000_000_000_000 {
        return Err("Pleas make sure there are more then 10T cycles available".to_string());
    }

    let cycles: u128 = 3_000_000_000_000;

    match create_canister(arg, cycles).await {
        Ok((canister,)) => Ok(canister.canister_id),
        Err((_, err)) => Err(err),
    }
}

#[query(guard = "is_developer")]
fn _dev_get_all_ids() -> Vec<(String, u64)> {
    IDLogic::get_all()
}

#[update(guard = "is_prod_developer")]
fn _dev_prod_init() -> Result<(), ApiError> {
    if id().to_string() != "2jvhk-5aaaa-aaaap-ahewa-cai" {
        return Err(
            ApiError::unsupported().add_message("This canister is not the production canister")
        );
    }

    let _ =
        HistoryCanisterStorage::set(Principal::from_text("inc34-eqaaa-aaaap-ahl2a-cai").unwrap());
    let _ =
        RewardCanisterStorage::set(Principal::from_text("zgfl7-pqaaa-aaaap-accpa-cai").unwrap());
    Ok(())
}

#[update(guard = "is_prod_developer")]
fn _dev_clear() {
    ProfileStore::clear();
    FriendRequestStore::clear();
    GroupStore::clear();
    MemberStore::clear();
    EventStore::clear();
    AttendeeStore::clear();
    ReportStore::clear();
    BoostedStore::clear();
    NotificationStore::clear();
    UserNotificationStore::clear();
    LoggerStore::clear();
    RewardBufferStore::clear();
    GroupMemberStore::clear();
    GroupEventsStore::clear();
    EventAttendeeStore::clear();
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let mut path: Vec<&str> = req.url.as_str().split('/').collect();
    path.retain(|p| !p.is_empty());

    let permission_denied = HttpResponse {
        status_code: 403,
        headers: vec![],
        body: vec![],
    };
    match req.method.as_str() {
        "GET" => match path[0] {
            "version" => HttpResponse {
                status_code: 200,
                headers: vec![],
                body: env!("CARGO_PKG_VERSION").as_bytes().to_vec(),
            },
            "changelog" => HttpResponse {
                status_code: 200,
                headers: vec![],
                body: include_bytes!("../../../CHANGELOG.md").to_vec(),
            },
            _ => permission_denied,
        },
        _ => permission_denied,
    }
}
