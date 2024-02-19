use std::{cell::RefCell, collections::HashMap, time::Duration};

use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    boosted::{Boosted, Subject},
    identifier::Identifier,
};
use ic_cdk::{api::time, caller};
use ic_cdk_timers::{clear_timer, set_timer, TimerId};
use ic_ledger_types::Tokens;

use crate::{
    storage::storage_api::{boosted, StorageMethods},
    E8S_PER_DAY_BOOST_COST,
};

use super::ledger_logic::Ledger;

thread_local! {
    pub static LAST_BLOCK_HEIGHT: RefCell<u64> = RefCell::new(u64::default());
    pub static TIMERS: RefCell<HashMap<u64, TimerId>> = RefCell::new(HashMap::default());
}

pub struct BoostCalls;

impl BoostCalls {
    pub async fn boost(subject: Subject, blockheight: u64) -> Result<u64, ApiError> {
        let tokens = Ledger::validate_transaction(caller(), blockheight).await?;
        if blockheight > Self::get_last_block_height() {
            Self::set_last_block_height(blockheight);
        } else {
            return Err(ApiError::bad_request()
                .add_message("Blockheight is lower than the last blockheight"));
        }

        let days = Self::calculate_days(tokens);
        let seconds = Self::get_seconds_from_days(days);

        match boosted().find(|_, boosted| boosted.subject == subject) {
            None => Self::new_boost(subject, seconds, caller(), blockheight),
            // If there is an existing boost
            Some((updating_boost_id, updating_boosted)) => {
                Self::update_exisiting_boost(updating_boost_id, updating_boosted, seconds)
            }
        }
    }

    fn new_boost(
        subject: Subject,
        seconds: u64,
        owner: Principal,
        blockheight: u64,
    ) -> Result<u64, ApiError> {
        let boost = Boosted::new(subject, seconds, owner, blockheight);

        let (new_boost_id, new_boost) = boosted().insert(boost)?;

        let timer_id = set_timer(Duration::from_secs(seconds), move || {
            Self::remove_boost(new_boost_id)
        });

        Self::set_timer_id(new_boost_id, timer_id);

        Ok(seconds)
    }

    fn update_exisiting_boost(
        boost_id: u64,
        mut boost: Boosted,
        seconds: u64,
    ) -> Result<u64, ApiError> {
        // Get and clear the existing timer
        if let Some(existing_timer_id) = Self::get_timer_id(boost_id) {
            clear_timer(existing_timer_id);
        }

        // Update the boost with the purchased seconds
        let remaining_seconds = Self::get_seconds_left_for_boosted(boost_id)?;
        let new_seconds = remaining_seconds + seconds;

        boost.seconds = new_seconds;
        boost.updated_at = time();

        boosted().update(boost_id, boost.clone())?;

        // Remove the old timer and set a new timer with the updated seconds
        let timer_id = set_timer(Duration::from_secs(new_seconds), move || {
            boosted().remove(boost_id);
        });

        Self::set_timer_id(boost_id, timer_id);
        Ok(new_seconds)
    }

    pub fn remove_boost(boost_id: u64) {
        boosted().remove(boost_id);
        Self::remove_timer_id(&boost_id);
    }

    pub fn calculate_days(tokens: Tokens) -> u64 {
        let days = ((tokens.e8s() as f64) / (E8S_PER_DAY_BOOST_COST as f64)).round() as u64;
        days
    }

    pub fn get_seconds_from_days(days: u64) -> u64 {
        days * 24 * 60 * 60
    }

    pub fn set_last_block_height(block_height: u64) {
        LAST_BLOCK_HEIGHT.with(|b| {
            *b.borrow_mut() = block_height;
        });
    }

    pub fn get_last_block_height() -> u64 {
        LAST_BLOCK_HEIGHT.with(|b| b.borrow().clone())
    }

    pub fn set_timer_id(boost_id: u64, timer_id: TimerId) {
        TIMERS.with(|t| {
            t.borrow_mut().insert(boost_id, timer_id);
        });
    }

    pub fn get_timer_id(boost_id: u64) -> Option<TimerId> {
        TIMERS.with(|t| t.borrow().get(&boost_id).cloned())
    }

    pub fn remove_timer_id(boost_id: &u64) {
        TIMERS.with(|t| {
            t.borrow_mut().remove(boost_id);
        });
    }

    pub fn get_seconds_left_for_boosted(boost_id: u64) -> Result<u64, ApiError> {
        let (_, boosted) = boosted().get(boost_id)?;
        let time_left: u64 = Duration::from_nanos(boosted.updated_at).as_secs() + boosted.seconds;
        Ok(time_left - Duration::from_nanos(time()).as_secs())
    }

    pub fn start_timers_after_upgrade() -> Result<(), ApiError> {
        boosted().get_all().into_iter().for_each(|(boost_id, _)| {
            let seconds_left = Self::get_seconds_left_for_boosted(boost_id).unwrap_or(0);
            let timer_id = set_timer(Duration::from_secs(seconds_left), move || {
                Self::remove_boost(boost_id)
            });

            Self::set_timer_id(boost_id, timer_id);
        });

        Ok(())
    }
}
