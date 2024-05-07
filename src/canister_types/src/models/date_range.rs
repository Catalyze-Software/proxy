use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct DateRange {
    start_date: u64,
    end_date: u64,
}

impl DateRange {
    pub fn new(start_date: u64, end_date: u64) -> Self {
        Self {
            start_date,
            end_date,
        }
    }

    pub fn start_date(&self) -> u64 {
        self.start_date
    }

    pub fn end_date(&self) -> u64 {
        self.end_date
    }

    pub fn is_within(&self, date: u64) -> bool {
        let is_within = date >= self.start_date;

        if self.end_date != 0 {
            return is_within && date <= self.end_date;
        }

        is_within
    }

    pub fn is_outside(&self, date: u64) -> bool {
        date < self.start_date || date > self.end_date
    }

    pub fn is_before(&self, date: u64) -> bool {
        date < self.start_date
    }

    pub fn is_after(&self, date: u64) -> bool {
        date > self.end_date
    }
}
