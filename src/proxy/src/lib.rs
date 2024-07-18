// should all be removed after implementation
#![allow(deprecated)]

use candid::Principal;
use ic_cdk::query;

pub static CATALYZE_MULTI_SIG: &str = "fcygz-gqaaa-aaaap-abpaa-cai";
pub static MULTISIG_INDEX: &str = "o7ouu-niaaa-aaaap-ahhdq-cai";
pub static E8S_PER_DAY_BOOST_COST: u64 = 3500000;
pub static USER_GROUP_CREATION_LIMIT: usize = 10;

pub mod calls;
pub mod helpers;
pub mod logic;
mod migration;
pub mod storage;

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;

    use canister_types::models::api_error::*;
    use canister_types::models::attendee::*;
    use canister_types::models::event::*;
    use canister_types::models::friend_request::*;
    use canister_types::models::group::*;
    use canister_types::models::group_transfer_request::GroupTransferRequest;
    use canister_types::models::http_types::HttpRequest;
    use canister_types::models::log::*;
    use canister_types::models::member::*;
    use canister_types::models::notification::*;
    use canister_types::models::paged_response::*;
    use canister_types::models::permission::*;
    use canister_types::models::profile::*;
    use canister_types::models::relation_type::*;
    use canister_types::models::report::*;
    use canister_types::models::reward::*;
    use canister_types::models::role::*;
    use canister_types::models::subject::*;
    use canister_types::models::topic::*;
    use canister_types::models::transaction_data::*;
    use canister_types::models::user_notifications::*;
    use canister_types::models::wallet::*;
    use canister_types::models::websocket_message::WSMessage;
    use ic_cdk::api::management_canister::http_request::HttpResponse;
    use ic_websocket_cdk::types::*;

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
    write(dir.join("proxy.did"), __export_did_tmp_()).expect("Write failed.");
}
