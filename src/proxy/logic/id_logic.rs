use crate::storage::IDStore;

pub struct IDLogic;

impl IDLogic {
    pub fn get_all() -> Vec<(String, u64)> {
        IDStore::get_all()
    }
}
