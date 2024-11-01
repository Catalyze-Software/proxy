use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::{self};

#[derive(Clone, CandidType, Deserialize)]
pub enum ExtError {
    InsufficientBalance,
    InvalidToken(String),
    Other(String),
    Unauthorized(String),
}

#[derive(Clone, CandidType, Deserialize)]
pub struct Listing {
    seller: Principal,
    price: u64,
    locked: Option<u64>,
}

pub type AccountIdentifier = String;
pub type TokenIndex = u32;

#[allow(non_camel_case_types)]
#[derive(Clone, CandidType, Deserialize)]
pub enum ExtResult {
    ok(Vec<(TokenIndex, Option<Listing>, Option<Vec<u8>>)>),
    err(ExtError),
}

#[derive(CandidType, Deserialize)]
pub enum DipNftError {
    UnauthorizedOwner,
    UnauthorizedOperator,
    OwnerNotFound,
    OperatorNotFound,
    TokenNotFound,
    ExistedNFT,
    SelfApprove,
    SelfTransfer,
}

pub async fn ext_balance_of(canister: Principal, account_identifier: String) -> u32 {
    let call: Result<(ExtResult,), _> =
        api::call::call(canister, "tokens_ext", (account_identifier,)).await;
    match call {
        Ok(response) => match response.0 {
            ExtResult::ok(_res) => _res.len() as u32,
            ExtResult::err(_) => 0,
        },
        Err(_) => 0,
    }
}

pub async fn dip20_balance_of(canister: Principal, principal: &Principal) -> u32 {
    let call: Result<(u32,), _> = api::call::call(canister, "balanceOf", (principal,)).await;
    match call {
        Ok(response) => response.0,
        Err(_) => 0,
    }
}

pub async fn legacy_dip721_balance_of(canister: Principal, principal: &Principal) -> u32 {
    let call: Result<(Result<u32, DipNftError>,), _> =
        api::call::call(canister, "balanceOf", (principal,)).await;
    match call {
        Ok(response) => response.0.unwrap_or_default(),
        Err(_) => 0,
    }
}

pub async fn dip721_balance_of(canister: Principal, principal: &Principal) -> u32 {
    let call: Result<(Result<u32, DipNftError>,), _> =
        api::call::call(canister, "dip721_balance_of", (principal,)).await;
    match call {
        Ok(response) => response.0.unwrap_or_default(),
        Err(_) => 0,
    }
}

pub async fn icrc_balance_of(canister: Principal, principal: &Principal) -> u128 {
    let call: Result<(u128,), _> =
        api::call::call(canister, "icrc1_balance_of", (principal,)).await;
    match call {
        Ok(response) => response.0,
        Err(_) => 0,
    }
}
