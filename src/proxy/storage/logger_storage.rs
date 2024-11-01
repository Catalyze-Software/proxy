use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdateable, LOGS,
        LOGS_MEMORY_ID,
    },
    ID_KIND_LOGS,
};
use canister_types::models::{
    api_error::ApiError,
    log::{Logger, PostLog},
};
use ic_stable_structures::memory_manager::MemoryId;

pub struct LoggerStore;

pub const MAX_LOGS: u64 = 10_000;

// Logging constants
pub const LOGIN_EVENT: &str = "LoginEvent";

impl Storage<u64, Logger> for LoggerStore {
    const NAME: &'static str = ID_KIND_LOGS;

    fn storage() -> StaticStorageRef<u64, Logger> {
        &LOGS
    }

    fn memory_id() -> MemoryId {
        LOGS_MEMORY_ID
    }
}

impl LoggerStore {
    /// Create a new logger from a post log
    /// # Arguments
    /// * `post_log` - The post log to create the logger from
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if created, otherwise an error
    pub fn new_from_post_log(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
        let log = Logger::from_post_log(post_log);
        Self::insert(log)
    }

    /// Create a new logger from a post log with the caller
    /// # Arguments
    /// * `post_log` - The post log to create the logger from
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if created, otherwise an error
    pub fn new_from_post_log_with_caller(post_log: PostLog) -> Result<(u64, Logger), ApiError> {
        let log = Logger::from_post_log_with_caller(post_log);
        Self::insert(log)
    }

    pub fn size() -> u64 {
        Self::storage().with(|logs| logs.borrow().len())
    }

    fn new_key() -> u64 {
        Self::storage().with(|logs| match logs.borrow().first_key_value() {
            Some((key, _)) => key - 1,
            None => u64::MAX,
        })
    }

    /// Get the latest logs from most recent to oldest
    /// # Arguments
    /// * `amount` - The number of logs to get
    /// # Returns
    /// * `Result<Vec<(u64, Logger)>, ApiError>` - The logs if found, otherwise an error
    pub fn get_latest_logs(amount: u64) -> Vec<Logger> {
        // keys are added in descending order so just take the first n
        Self::storage().with(|logs| {
            logs.borrow()
                .iter()
                .take(amount as usize)
                .map(|(_, log)| log.clone())
                .collect()
        })
    }

    pub fn logged_in_past_5_minutes() -> bool {
        let now = ic_cdk::api::time();
        let five_minutes_ago = now - 300_000_000_000;

        let logged_in = LOGS.with(|logs| {
            for log in logs.borrow().iter() {
                let within_5_minutes = log.1.created_on > five_minutes_ago;
                if !within_5_minutes {
                    break;
                }

                let login_event = log.1.description == LOGIN_EVENT;
                let same_principal = log.1.principal == Some(ic_cdk::caller());

                if within_5_minutes && login_event && same_principal {
                    return Some(true);
                }
            }
            None
        });

        logged_in.unwrap_or_default()
    }
}

impl StorageQueryable<u64, Logger> for LoggerStore {}
impl StorageUpdateable<u64, Logger> for LoggerStore {}
impl StorageInsertable<Logger> for LoggerStore {
    /// # Arguments
    /// * `logger` - The logger to insert
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if inserted, otherwise an error
    fn insert(logger: Logger) -> Result<(u64, Logger), ApiError> {
        let key = Self::new_key();

        Self::storage().with(|logs| logs.borrow_mut().insert(key, logger.clone()));

        while Self::size() > MAX_LOGS {
            Self::storage().with(|logs| {
                let mut logs = logs.borrow_mut();
                let last_key_val = logs
                    .last_key_value()
                    .expect("Failed to get first key value");
                logs.remove(&last_key_val.0);
            });
        }

        Ok((key, logger))
    }
}
