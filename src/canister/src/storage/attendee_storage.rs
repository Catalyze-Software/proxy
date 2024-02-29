use super::storage_api::{
    IdentifierRefMethods, PrincipalIdentifier, StorageMethods, ATTENDEES, ATTENDEES_IDENTIFIER_REF,
};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    attendee::Attendee,
    identifier::{Identifier, IdentifierKind},
};
use ic_cdk::caller;

pub struct AttendeeStore;

pub const NAME: &str = "attendees";

impl IdentifierRefMethods<PrincipalIdentifier> for AttendeeStore {
    /// get a new identifier
    /// # Returns
    /// * `PrincipalIdentifier` - The new identifier
    fn new_identifier() -> PrincipalIdentifier {
        let id = ATTENDEES_IDENTIFIER_REF.with(|data| {
            data.borrow()
                .last_key_value()
                .map(|(k, _)| Identifier::from(k).id() + 1)
                .unwrap_or(0)
        });

        Identifier::generate(IdentifierKind::Profile(id))
            .to_principal()
            .unwrap()
    }

    /// Get the key by identifier
    /// # Arguments
    /// * `key` - The identifier to get the key for
    /// # Returns
    /// * `Option<Principal>` - The key if found, otherwise None
    fn get_id_by_identifier(key: &PrincipalIdentifier) -> Option<Principal> {
        ATTENDEES_IDENTIFIER_REF.with(|data| data.borrow().get(key))
    }

    /// Get the identifier by key
    /// # Arguments
    /// * `value` - The value to get the identifier for
    /// # Returns
    /// * `Option<PrincipalIdentifier>` - The identifier if found, otherwise None
    fn get_identifier_by_id(value: &Principal) -> Option<PrincipalIdentifier> {
        ATTENDEES_IDENTIFIER_REF.with(|data| {
            data.borrow()
                .iter()
                .find(|(_, v)| v == value)
                .map(|(k, _)| k.clone())
        })
    }

    /// Insert an identifier reference with the caller as value
    /// # Arguments
    /// * `key` - The increment value to insert
    /// # Returns
    /// * `Result<Principal, ApiError>` - The inserted principal if successful, otherwise an error
    fn insert_identifier_ref(key: PrincipalIdentifier) -> Result<Principal, ApiError> {
        ATTENDEES_IDENTIFIER_REF.with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_identifier_ref")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, caller());
            Ok(caller())
        })
    }

    /// Remove an identifier reference
    /// # Arguments
    /// * `key` - The identifier to remove
    /// # Returns
    /// * `bool` - True if the identifier was removed, otherwise false
    fn remove_identifier_ref(key: &PrincipalIdentifier) -> bool {
        ATTENDEES_IDENTIFIER_REF.with(|data| data.borrow_mut().remove(key).is_some())
    }
}

impl StorageMethods<Principal, Attendee> for AttendeeStore {
    /// Get a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to get
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The attendee if found, otherwise an error
    fn get(key: Principal) -> Result<(Principal, Attendee), ApiError> {
        ATTENDEES.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple attendees by key
    /// # Arguments
    /// * `ids` - The keys of the attendees to get
    /// # Returns
    /// * `Vec<Attendee>` - The reports if found, otherwise an empty vector
    fn get_many(keys: Vec<Principal>) -> Vec<(Principal, Attendee)> {
        ATTENDEES.with(|data| {
            let mut attendees = Vec::new();
            for key in keys {
                if let Some(attendee) = data.borrow().get(&key) {
                    attendees.push((key, attendee));
                }
            }
            attendees
        })
    }

    /// Find a single attendee by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, Attendee)>` - The attendee if found, otherwise None
    fn find<F>(filter: F) -> Option<(Principal, Attendee)>
    where
        F: Fn(&Principal, &Attendee) -> bool,
    {
        ATTENDEES.with(|data| {
            data.borrow()
                .iter()
                .find(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
        })
    }

    /// Find all attendees by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, Attendee)>` - The attendees if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(Principal, Attendee)>
    where
        F: Fn(&Principal, &Attendee) -> bool,
    {
        ATTENDEES.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
                .collect()
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(_value: Attendee) -> Result<(Principal, Attendee), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to insert
    /// * `value` - The attendee to insert
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The inserted attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if so returns an error
    fn insert_by_key(key: Principal, value: Attendee) -> Result<(Principal, Attendee), ApiError> {
        ATTENDEES.with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Update a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to update
    /// * `value` - The attendee to update
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The updated attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if not returns an error
    fn update(key: Principal, value: Attendee) -> Result<(Principal, Attendee), ApiError> {
        ATTENDEES.with(|data| {
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

    /// Remove a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to remove
    /// # Returns
    /// * `bool` - True if the attendee was removed, otherwise false
    /// # Note
    fn remove(key: Principal) -> bool {
        ATTENDEES.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
