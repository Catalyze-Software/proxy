#![allow(unused)]
use crate::{ENV, SENDER};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    filter_type::FilterType,
    paged_response::PagedResponse,
    report::{PostReport, ReportFilter, ReportResponse, ReportSort},
};
use pocket_ic::{query_candid_as, update_candid_as};

pub fn add_report(
    value: PostReport,
    group_identifier: Principal,
    member_identifier: Principal,
) -> ReportResponse {
    update_candid_as::<(PostReport, Principal, Principal), (Result<ReportResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "add_report",
        (value, group_identifier, member_identifier),
    )
    .expect("Failed to call add_report from pocket ic")
    .0
    .expect("Failed to call add_report")
}

pub fn get_report(report_id: u64, group_id: u64) -> Result<ReportResponse, ApiError> {
    query_candid_as::<(u64, u64), (Result<ReportResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_report",
        (report_id, group_id),
    )
    .expect("Failed to call get_report from pocket ic")
    .0
}

pub fn get_reports(
    limit: usize,
    page: usize,
    sort: ReportSort,
    filters: Vec<FilterType<ReportFilter>>,
    group_id: u64,
) -> Result<PagedResponse<ReportResponse>, ApiError> {
    query_candid_as::<
        (usize, usize, ReportSort, Vec<FilterType<ReportFilter>>, u64),
        (Result<PagedResponse<ReportResponse>, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_reports",
        (limit, page, sort, filters, group_id),
    )
    .expect("Failed to call get_reports from pocket ic")
    .0
}
