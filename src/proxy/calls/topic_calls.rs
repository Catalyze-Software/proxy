use crate::{helpers::guards::is_developer, logic::topic_logic::TopicCalls};
use canister_types::models::{
    api_error::ApiError,
    topic::{Topic, TopicKind},
};
use ic_cdk::{query, update};

/// Add a topic to the canister  - [`[update]`](update)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Category, Skill)
/// * `value` - The topic to add
/// # Returns
/// * `Topic` - The added topic
/// # Errors
/// * `ApiError` - If something went wrong while adding the topic
/// # Note
/// This function is guarded by the [`is_developer`](is_developer) function.
#[update(guard = "is_developer")]
pub async fn add_topic(kind: TopicKind, value: String) -> Result<Topic, ApiError> {
    TopicCalls::add(kind, value)
}

/// Remove a topic from the canister  - [`[update]`](update)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Category, Skill)
/// * `id` - The topic id to remove
/// # Returns
/// * `bool` - if the removal was successful
/// # Note
/// This function is guarded by the [`is_developer`](is_developer) function.
#[update(guard = "is_developer")]
pub async fn remove_topic(kind: TopicKind, id: u64) -> bool {
    TopicCalls::remove(kind, id)
}

/// Add many topics to the canister  - [`[update]`](update)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Category, Skill)
/// * `Vec<value>` - The topics to add
/// # Returns
/// * `Result<Topic, ApiError>` - The result for each added topic
/// # Errors
/// * `ApiError` - If something went wrong while adding the topic
/// # Note
/// This function is guarded by the [`is_developer`](is_developer) function.
#[update(guard = "is_developer")]
pub async fn add_topics(kind: TopicKind, value: Vec<String>) -> Vec<Result<Topic, ApiError>> {
    value
        .into_iter()
        .map(|v| TopicCalls::add(kind.clone(), v))
        .collect()
}

/// Get a topic - [`[query]`](query)
/// # Arguments
/// * `kind` - The kind of the topic (Tag, Category, Skill)
/// * `id` - The identifier of the topic
/// # Returns
/// * `Topic` - The topic
/// # Errors
/// * `ApiError` - If something went wrong while getting the topic
#[query]
pub fn get_topic(kind: TopicKind, id: u64) -> Result<Topic, ApiError> {
    TopicCalls::get(kind, id)
}

/// Get topics by their identifiers and kind - [`[query]`](query)
/// # Arguments
/// * `kind` - The kind of the topics (Tag, Category, Skill)
/// * `ids` - The identifiers of the topics
/// # Returns
/// * `Vec<Topic>` - The topics
/// # Errors
/// * `ApiError` - If something went wrong while getting the topics
#[query]
pub fn get_topics(kind: TopicKind, ids: Vec<u64>) -> Result<Vec<Topic>, ApiError> {
    TopicCalls::get_many(kind, ids)
}

/// Get all topics by kind - [`[query]`](query)
/// # Arguments
/// * `kind` - The kind of the topics (Tag, Category, Skill)
/// # Returns
/// * `Vec<Topic>` - The topics
/// # Errors
/// * `ApiError` - If something went wrong while getting the topics
#[query]
pub fn get_all_topics(kind: TopicKind) -> Result<Vec<Topic>, ApiError> {
    TopicCalls::get_all(kind)
}
