use super::{
    storage_api::{
        StaticStorageRef, Storage, StorageQueryable, StorageUpdateable, REPORTS, REPORTS_MEMORY_ID,
    },
    StorageInsertable, ID_KIND_REPORTS,
};
use canister_types::models::report::Report;
use ic_stable_structures::memory_manager::MemoryId;

pub struct ReportStore;

impl Storage<u64, Report> for ReportStore {
    const NAME: &'static str = ID_KIND_REPORTS;

    fn storage() -> StaticStorageRef<u64, Report> {
        &REPORTS
    }

    fn memory_id() -> MemoryId {
        REPORTS_MEMORY_ID
    }
}

impl StorageQueryable<u64, Report> for ReportStore {}
impl StorageUpdateable<u64, Report> for ReportStore {}
impl StorageInsertable<Report> for ReportStore {}
