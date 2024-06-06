use catalyze_shared::api_error::ApiError;
use ic_cdk::{query, update};

use canister_types::models::topic::{Topic, TopicKind};

use crate::{helpers::guards::has_access, logic::topic_logic::TopicCalls};

/// Add a topic to the canister  - [`[update]`](update)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Interest, Skill)
/// * `value` - The topic to add
/// # Returns
/// * `Topic` - The added topic
/// # Errors
/// * `ApiError` - If something went wrong while adding the topic
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub async fn add_topic(kind: TopicKind, value: String) -> Result<Topic, ApiError> {
    TopicCalls::add(kind, value)
}

/// Get a topic - [`[query]`](query)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Interest, Skill)
/// * `id` - The identifier of the topic
/// # Returns
/// * `Topic` - The topic
/// # Errors
/// * `ApiError` - If something went wrong while getting the topic
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_topic(kind: TopicKind, id: u64) -> Result<Topic, ApiError> {
    TopicCalls::get(kind, id)
}

/// Get topics by their identifiers and kind - [`[query]`](query)
/// # Arguments
/// * `kind` - The kind of the topics (Tag, Interest, Skill)
/// * `ids` - The identifiers of the topics
/// # Returns
/// * `Vec<Topic>` - The topics
/// # Errors
/// * `ApiError` - If something went wrong while getting the topics
#[query(guard = "has_access")]
pub fn get_topics(kind: TopicKind, ids: Vec<u64>) -> Result<Vec<Topic>, ApiError> {
    TopicCalls::get_many(kind, ids)
}

/// Get all topics by kind - [`[query]`](query)
/// # Arguments
/// * `kind` - The kind of the topics (Tag, Interest, Skill)
/// # Returns
/// * `Vec<Topic>` - The topics
/// # Errors
/// * `ApiError` - If something went wrong while getting the topics
#[query(guard = "has_access")]
pub fn get_all_topics(kind: TopicKind) -> Result<Vec<Topic>, ApiError> {
    TopicCalls::get_all(kind)
}
