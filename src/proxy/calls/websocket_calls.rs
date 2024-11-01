use candid::Principal;
use canister_types::models::websocket_message::WSMessage;
use ic_cdk::{query, update};
use ic_websocket_cdk::{
    CanisterWsCloseArguments, CanisterWsCloseResult, CanisterWsGetMessagesArguments,
    CanisterWsGetMessagesResult, CanisterWsMessageArguments, CanisterWsMessageResult,
    CanisterWsOpenArguments, CanisterWsOpenResult,
};

use crate::{helpers::guards::is_developer, logic::websocket_logic::Websocket};

#[update]
fn ws_open(args: CanisterWsOpenArguments) -> CanisterWsOpenResult {
    ic_websocket_cdk::ws_open(args)
}

#[update]
fn ws_close(args: CanisterWsCloseArguments) -> CanisterWsCloseResult {
    ic_websocket_cdk::ws_close(args)
}

#[update]
fn ws_message(
    args: CanisterWsMessageArguments,
    msg_type: Option<WSMessage>,
) -> CanisterWsMessageResult {
    ic_websocket_cdk::ws_message(args, msg_type)
}

#[query]
fn ws_get_messages(args: CanisterWsGetMessagesArguments) -> CanisterWsGetMessagesResult {
    ic_websocket_cdk::ws_get_messages(args)
}

#[query]
fn get_connected_clients() -> Vec<Principal> {
    Websocket::get_connected_clients()
}

#[query]
fn get_ws_errors() -> Vec<(u64, String)> {
    Websocket::get_ws_errors()
}

#[query]
fn get_ws_url() -> String {
    Websocket::get_ws_url()
}

#[update(guard = "is_developer")]
fn set_ws_url(url: String) {
    Websocket::set_ws_url(url)
}
