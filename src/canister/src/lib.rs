// should all be removed after implementation
#![allow(deprecated)]
#![allow(unused_variables)]

use candid::Principal;
use ic_cdk::query;

pub static CATALYZE_MULTI_SIG: &str = "fcygz-gqaaa-aaaap-abpaa-cai";
pub static E8S_PER_DAY_BOOST_COST: u64 = 3500000;

pub mod calls;
pub mod helpers;
pub mod logic;
pub mod storage;

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;
    use canister_types::models::api_error::*;
    use canister_types::models::attendee::*;
    use canister_types::models::boosted::*;
    use canister_types::models::event::*;
    use canister_types::models::filter_type::*;
    use canister_types::models::friend_request::*;
    use canister_types::models::group::*;
    use canister_types::models::member::*;
    use canister_types::models::paged_response::*;
    use canister_types::models::permission::*;
    use canister_types::models::privacy::*;
    use canister_types::models::profile::*;
    use canister_types::models::relation_type::*;
    use canister_types::models::report::*;
    use canister_types::models::role::*;
    use canister_types::models::wallet::*;
    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = dir.parent().unwrap().parent().unwrap().join("candid");
    write(dir.join(format!("canister.did")), __export_did_tmp_()).expect("Write failed.");
}
