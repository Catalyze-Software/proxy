use super::{storage_api::LOGS, StorageMethods};
use canister_types::models::{
    api_error::ApiError,
    log::{Logger, PostLog},
};

pub struct LoggerStore;

pub const NAME: &str = "logs";
pub const MAX_LOGS: u64 = 10_000;

// Logging constants
pub const LOGIN_EVENT: &str = "LoginEvent";

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
        LOGS.with(|logs| logs.borrow().len() as u64)
    }

    fn new_key() -> u64 {
        LOGS.with(|logs| match logs.borrow().first_key_value() {
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
        LOGS.with(|logs| {
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

        logged_in.unwrap_or(false)
    }
}

impl StorageMethods<u64, Logger> for LoggerStore {
    /// Get a single logger by id
    /// # Arguments
    /// * `key` - The key of the report to get
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if found, otherwise an error
    fn get(key: u64) -> Result<(u64, Logger), ApiError> {
        LOGS.with(|logs| {
            logs.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|log| (key, log.clone()))
        })
    }

    fn get_many(_: Vec<u64>) -> Vec<(u64, Logger)> {
        todo!()
    }

    /// Find a single logger by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, Logger)>` - The logger if found, otherwise None
    fn find<F>(filter: F) -> Option<(u64, Logger)>
    where
        F: Fn(&u64, &Logger) -> bool,
    {
        LOGS.with(|logs| logs.borrow().iter().find(|(id, log)| filter(id, log)))
    }

    /// Find all loggers by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, Logger)>` - The loggers if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(u64, Logger)>
    where
        F: Fn(&u64, &Logger) -> bool,
    {
        LOGS.with(|logs| {
            logs.borrow()
                .iter()
                .filter(|(id, log)| filter(id, log))
                .collect()
        })
    }

    /// # Arguments
    /// * `logger` - The logger to insert
    /// # Returns
    /// * `Result<(u64, Logger), ApiError>` - The logger if inserted, otherwise an error
    fn insert(logger: Logger) -> Result<(u64, Logger), ApiError> {
        let key = Self::new_key();

        LOGS.with(|logs| logs.borrow_mut().insert(key, logger.clone()));

        while Self::size() > MAX_LOGS {
            LOGS.with(|logs| {
                let mut logs = logs.borrow_mut();
                let last_key_val = logs
                    .last_key_value()
                    .expect("Failed to get first key value");
                logs.remove(&last_key_val.0);
            });
        }

        Ok((key, logger))
    }

    fn insert_by_key(_: u64, _: Logger) -> Result<(u64, Logger), ApiError> {
        todo!()
    }

    fn update(_: u64, _: Logger) -> Result<(u64, Logger), ApiError> {
        todo!()
    }

    fn remove(_: u64) -> bool {
        todo!()
    }

    fn clear() -> () {
        todo!()
    }
}
