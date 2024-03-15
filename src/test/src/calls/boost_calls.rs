#![allow(unused)]

use crate::{ENV, SENDER};
use candid::Principal;
use canister_types::models::boosted::Boost;
use pocket_ic::{query_candid_as, update_candid_as};

pub fn get_boosted_groups() -> Vec<Boost> {
    query_candid_as::<(), (Vec<Boost>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_boosted_groups",
        (),
    )
    .expect("Failed to call get_boosted_groups from pocket ic")
    .0
}

pub fn get_boosted_events() -> Vec<Boost> {
    query_candid_as::<(), (Vec<Boost>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_boosted_events",
        (),
    )
    .expect("Failed to call get_boosted_events from pocket ic")
    .0
}

pub fn get_e8s_per_day_boost_cost() -> u64 {
    query_candid_as::<(), (u64,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_e8s_per_day_boost_cost",
        (),
    )
    .expect("Failed to call get_e8s_per_day_boost_cost from pocket ic")
    .0
}

pub fn boost(identifier: Principal, blockheight: u64) -> u64 {
    update_candid_as::<(Principal, u64), (u64,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "boost",
        (identifier, blockheight),
    )
    .expect("Failed to call boost from pocket ic")
    .0
}

pub fn get_remaining_boost_time_in_seconds(identifier: Principal) -> u64 {
    query_candid_as::<(Principal,), (u64,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_remaining_boost_time_in_seconds",
        (identifier,),
    )
    .expect("Failed to call get_remaining_boost_time_in_seconds from pocket ic")
    .0
}
