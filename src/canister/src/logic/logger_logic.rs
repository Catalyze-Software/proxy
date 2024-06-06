use catalyze_shared::api_error::ApiError;
use ic_cdk::{api::time, caller};

use canister_types::models::log::Logger;

use crate::storage::{LoggerStore, LOGIN_EVENT, StorageInsertable};

pub struct LoginEvent;

impl LoginEvent {
    pub fn log_event() -> Result<(u64, Logger), ApiError> {
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
