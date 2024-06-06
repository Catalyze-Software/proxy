use catalyze_shared::{
    api_error::ApiError,
    validation::{ValidateField, ValidationType},
};

use canister_types::models::topic::{Topic, TopicKind};

use crate::{
    helpers::validator::Validator,
    storage::{InterestStore, SkillStore, StorageInsertable, StorageQueryable, TagStore},
};

pub struct TopicCalls;

impl TopicCalls {
    pub fn add(kind: TopicKind, topic: String) -> Result<Topic, ApiError> {
        let topic = handle_topic(kind.clone(), topic)?;

        let raw = match kind {
            TopicKind::Tag => TagStore::insert(topic),
            TopicKind::Interest => InterestStore::insert(topic),
            TopicKind::Skill => SkillStore::insert(topic),
        }?;

        Topic::from((raw, kind)).into()
    }

    pub fn get_all(kind: TopicKind) -> Result<Vec<Topic>, ApiError> {
        let result = match kind {
            TopicKind::Tag => TagStore::get_all(),
            TopicKind::Interest => InterestStore::get_all(),
            TopicKind::Skill => SkillStore::get_all(),
        }
        .into_iter()
        .map(|raw| Topic::from((raw, kind.clone())))
        .collect();

        Ok(result)
    }

    pub fn get(kind: TopicKind, id: u64) -> Result<Topic, ApiError> {
        let raw = match kind {
            TopicKind::Tag => TagStore::get(id),
            TopicKind::Interest => InterestStore::get(id),
            TopicKind::Skill => SkillStore::get(id),
        }?;

        Topic::from((raw, kind)).into()
    }

    pub fn get_many(kind: TopicKind, ids: Vec<u64>) -> Result<Vec<Topic>, ApiError> {
        let result = match kind {
            TopicKind::Tag => TagStore::get_many(ids),
            TopicKind::Interest => InterestStore::get_many(ids),
            TopicKind::Skill => SkillStore::get_many(ids),
        }
        .into_iter()
        .map(|raw| Topic::from((raw, kind.clone())))
        .collect();

        Ok(result)
    }
}

const TOPIC_MAX_LENGTH: usize = 32;
const TOPIC_MIN_LENGTH: usize = 1;

fn handle_topic(kind: TopicKind, topic: String) -> Result<String, ApiError> {
    let topic = topic.to_lowercase();
    let topic = topic.trim();

    Validator::new(vec![ValidateField(
        ValidationType::StringLength(topic.to_string(), TOPIC_MIN_LENGTH, TOPIC_MAX_LENGTH),
        kind.to_string(),
    )])
    .validate()?;

    let existing = match kind {
        TopicKind::Tag => TagStore::find(|_, value| value == topic),
        TopicKind::Interest => InterestStore::find(|_, value| value == topic),
        TopicKind::Skill => SkillStore::find(|_, value| value == topic),
    };

    if let Some(existing) = existing {
        return Err(ApiError::duplicate()
            .add_tag(&kind.to_string())
            .add_message(&format!(
                "{kind} \"{}\" already exists with ID: {}",
                existing.1, existing.0
            )));
    }

    Ok(topic.to_string())
}
