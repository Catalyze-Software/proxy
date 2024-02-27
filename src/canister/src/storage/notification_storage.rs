use std::thread::LocalKey;

use super::storage_api::{StorageMethods, StorageRef};
use canister_types::models::{api_error::ApiError, notification::Notification};

pub struct NotificationStore<'a> {
    store: &'a LocalKey<StorageRef<u64, Notification>>,
}

impl<'a> NotificationStore<'a> {
    pub fn new(store: &'a LocalKey<StorageRef<u64, Notification>>) -> Self {
        Self { store }
    }
}

pub const NAME: &str = "notification";

impl StorageMethods<u64, Notification> for NotificationStore<'static> {
    /// Get a single notification by key
    /// # Arguments
    /// * `key` - The key of the notification to get
    /// # Returns
    /// * `Result<Notification, ApiError>` - The notification if found, otherwise an error
    fn get(&self, key: u64) -> Result<(u64, Notification), ApiError> {
        self.store.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple notifications by key
    /// # Arguments
    /// * `ids` - The keys of the notifications to get
    /// # Returns
    /// * `Vec<Notification>` - The notification if found, otherwise an empty vector
    fn get_many(&self, keys: Vec<u64>) -> Vec<(u64, Notification)> {
        self.store.with(|data| {
            let mut notification = Vec::new();
            for key in keys {
                if let Some(value) = data.borrow().get(&key) {
                    notification.push((key, value));
                }
            }
            notification
        })
    }

    /// Find a single notification by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, Notification)>` - The notification if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(u64, Notification)>
    where
        F: Fn(&u64, &Notification) -> bool,
    {
        self.store
            .with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all notifications by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, Notification)>` - The notification if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(u64, Notification)>
    where
        F: Fn(&u64, &Notification) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    /// Insert a single notification
    /// # Arguments
    /// * `value` - The notification to insert
    /// # Returns
    /// * `Result<Notification, ApiError>` - The inserted notification if successful, otherwise an error
    /// # Note
    /// Does check if a notification with the same key already exists, if so returns an error
    fn insert(&mut self, value: Notification) -> Result<(u64, Notification), ApiError> {
        self.store.with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(0);

            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert_by_key(
        &mut self,
        _key: u64,
        _value: Notification,
    ) -> Result<(u64, Notification), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert_by_key") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value does not require a key to be inserted, use `insert` instead"))
    }

    /// Update a single notification by key
    /// # Arguments
    /// * `key` - The key of the notification to update
    /// * `value` - The notification to update
    /// # Returns
    /// * `Result<Notification, ApiError>` - The updated notification if successful, otherwise an error
    /// # Note
    /// Does check if a notification with the same key already exists, if not returns an error
    fn update(&mut self, key: u64, value: Notification) -> Result<(u64, Notification), ApiError> {
        self.store.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(NAME)
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Remove a single notification by key
    /// # Arguments
    /// * `key` - The key of the notification to remove
    /// # Returns
    /// * `bool` - True if the notification was removed, otherwise false
    fn remove(&mut self, key: u64) -> bool {
        self.store
            .with(|data| data.borrow_mut().remove(&key).is_some())
    }
}