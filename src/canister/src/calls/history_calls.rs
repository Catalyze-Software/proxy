use catalyze_shared::api_error::ApiError;
use ic_cdk::query;

use crate::storage::{CellStorage, HistoryPointStorage};

#[query]
fn get_history_point() -> Result<u64, ApiError> {
    HistoryPointStorage::get()
}
