use crate::storage::{LoggerStore, StorageInsertable, LOGIN_EVENT};
use canister_types::models::{api_error::ApiError, log::Logger};
use ic_cdk::{api::time, caller};

pub struct LoginEvent;

impl LoginEvent {
    pub fn new() -> Result<(u64, Logger), ApiError> {
        // check log store for recent logins
        if LoggerStore::logged_in_past_5_minutes() {
            return Err(ApiError::duplicate());
        };

        // create log
        let log = Logger {
            description: LOGIN_EVENT.to_string(),
            source: None,
            principal: Some(caller()),
            data: None,
            created_on: time(),
        };

        // insert log into store
        LoggerStore::insert(log)
    }
}
