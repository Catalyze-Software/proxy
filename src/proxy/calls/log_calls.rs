use crate::storage::LoggerStore;
use crate::{
    helpers::guards::{has_access, is_developer, is_monitor},
    logic::logger_logic::LoginEvent,
};
use canister_types::models::log::LogType;
use canister_types::models::{
    api_error::ApiError,
    log::{Logger, PostLog},
};
use ic_cdk::{query, update};

// Update functions
#[update(guard = "has_access")]
fn log(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
    LoggerStore::new_from_post_log(post_log)
}

#[update(guard = "has_access")]
fn log_with_caller(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
    LoggerStore::new_from_post_log_with_caller(post_log)
}

#[update(guard = "has_access")]
fn log_login() -> Result<(u64, Logger), ApiError> {
    LoginEvent::log_event()
}

#[update(guard = "is_developer")]
fn test_log() {
    let log = PostLog {
        log_type: LogType::Info,
        description: "Test log".to_string(),
        source: None,
        data: None,
    };
    LoggerStore::new_from_post_log_with_caller(log).expect("Failed to log test log");
}

// Query functions
#[query(guard = "is_developer")]
fn get_latest_logs(count: u64) -> Vec<Logger> {
    LoggerStore::get_latest_logs(count)
}

#[query(guard = "is_monitor")]
fn log_size() -> u64 {
    LoggerStore::size()
}
