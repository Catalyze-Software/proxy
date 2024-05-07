use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};

use ic_cdk::{api::time, caller};
use serde::Serialize;

use crate::{
    impl_storable_for,
    models::{date_range::DateRange, sort_direction::SortDirection},
};

use super::{
    api_error::ApiError,
    subject::{Subject, SubjectResponse, SubjectType},
};

impl_storable_for!(Report);

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, Eq)]
pub struct Report {
    pub reported_by: Principal,
    pub subject: Subject,
    pub message: String,
    pub group_id: Option<u64>,
    pub notification_id: Option<u64>,
    pub created_on: u64,
}

impl Default for Report {
    fn default() -> Self {
        Self {
            reported_by: Principal::anonymous(),
            subject: Subject::default(),
            message: Default::default(),
            group_id: Default::default(),
            notification_id: Default::default(),
            created_on: Default::default(),
        }
    }
}

impl Report {
    pub fn new(value: PostReport) -> Self {
        Self {
            reported_by: caller(),
            subject: value.subject,
            message: value.message,
            notification_id: None,
            group_id: Some(value.group_id),
            created_on: time(),
        }
    }

    pub fn set_notification_id(&mut self, notification_id: u64) {
        self.notification_id = Some(notification_id);
    }

    pub fn remove_notification_id(&mut self) {
        self.notification_id = None;
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PostReport {
    pub subject: Subject,
    pub message: String,
    pub group_id: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ReportResponse {
    pub id: u64,
    pub reported_by: Principal,
    pub subject: SubjectResponse,
    pub message: String,
    pub created_on: u64,
}

impl ReportResponse {
    pub fn new(id: u64, report: Report, subject_response: SubjectResponse) -> Self {
        Self {
            id,
            reported_by: report.reported_by,
            subject: subject_response,
            message: report.message,
            created_on: report.created_on,
        }
    }

    pub fn from_result(
        report_result: Result<(u64, Report), Box<ApiError>>,
        subject_response: SubjectResponse,
    ) -> Result<Self, Box<ApiError>> {
        match report_result {
            Err(err) => Err(err),
            Ok((id, report)) => Ok(Self::new(id, report, subject_response)),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ReportSort {
    ReportedBy(SortDirection),
    Subject(SortDirection),
    SubjectType(SortDirection),
    CreatedOn(SortDirection),
}

impl Default for ReportSort {
    fn default() -> Self {
        ReportSort::CreatedOn(SortDirection::Asc)
    }
}

impl ReportSort {
    pub fn sort(&self, reports: HashMap<u64, Report>) -> Vec<(u64, Report)> {
        let mut reports: Vec<(u64, Report)> = reports.into_iter().collect();
        use ReportSort::*;
        use SortDirection::*;
        match self {
            ReportedBy(direction) => match direction {
                Asc => reports.sort_by(|a, b| a.1.reported_by.cmp(&b.1.reported_by)),
                Desc => reports.sort_by(|a, b| b.1.reported_by.cmp(&a.1.reported_by)),
            },
            Subject(direction) => match direction {
                Asc => reports.sort_by(|a, b| a.1.subject.cmp(&b.1.subject)),
                Desc => reports.sort_by(|a, b| b.1.subject.cmp(&a.1.subject)),
            },
            SubjectType(direction) => match direction {
                Asc => reports.sort_by(|a, b| a.1.subject.cmp(&b.1.subject)),
                Desc => reports.sort_by(|a, b| b.1.subject.cmp(&a.1.subject)),
            },
            CreatedOn(direction) => match direction {
                Asc => reports.sort_by(|a, b| a.1.created_on.cmp(&b.1.created_on)),
                Desc => reports.sort_by(|a, b| b.1.created_on.cmp(&a.1.created_on)),
            },
        }
        reports
    }
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub enum ReportFilter {
    #[default]
    None,
    Subject(Subject),
    GroupId(u64),
    SubjectType(SubjectType),
    CreatedOn(DateRange),
    ReportedBy(Principal),
}

impl ReportFilter {
    pub fn is_match(&self, id: &u64, report: &Report) -> bool {
        use crate::models::subject::Subject;
        use crate::models::subject::SubjectType;
        use ReportFilter::*;
        match self {
            None => true,
            Subject(subject) => subject == &report.subject,
            SubjectType(subject_type) => {
                let _subject_type = match report.subject {
                    Subject::Group(_) => SubjectType::Group,
                    Subject::Event(_) => SubjectType::Event,
                    Subject::Profile(_) => SubjectType::Profile,
                    Subject::Member(_) => SubjectType::Member,
                    Subject::Attendee(_) => SubjectType::Attendee,
                    Subject::None => SubjectType::None,
                };
                _subject_type == *subject_type
            }
            CreatedOn(date_range) => date_range.is_within(report.created_on),
            ReportedBy(principal) => principal == &report.reported_by,
            GroupId(group_id) => group_id == &report.group_id.unwrap_or_default(),
        }
    }
}
