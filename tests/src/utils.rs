use std::{env, fs, path::Path};

use candid::{
    utils::{ArgumentDecoder, ArgumentEncoder},
    Principal,
};
use canister_types::models::api_error::ApiError;
use elliptic_curve::SecretKey;
use eyre::Result;
use ic_agent::{identity::Secp256k1Identity, Identity};
use pocket_ic::{update_candid_as, CallError, PocketIc};

pub struct Context {
    pub pic: PocketIc,
    pub canister_id: Principal,
}

pub fn random_principal() -> Principal {
    let private_key = SecretKey::random(&mut rand::thread_rng());
    Secp256k1Identity::from_private_key(private_key)
        .sender()
        .expect("Failed to get sender")
}

pub fn setup() -> Context {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let path = env::var("WASM_PATH").unwrap_or_else(|_| "../wasm/canister.wasm.gz".to_string());
    let wasm_bytes = fs::read(Path::new(&path)).unwrap_or_else(|_| {
        panic!("Failed to read wasm file at path: {}", path);
    });
    pic.install_canister(canister_id, wasm_bytes, vec![], None);

    Context { pic, canister_id }
}

pub trait FallibleCall<T> {
    fn assert_success(self) -> T;
}

impl<T> FallibleCall<T> for Result<T, ApiError> {
    fn assert_success(self) -> T {
        match self {
            Ok(resp) => resp,
            Err(e) => panic!("Failed to call: {}", e),
        }
    }
}

pub fn call<Input, Output>(ctx: &Context, sender: Principal, method: &str, input: Input) -> Output
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
{
    let resp = update_candid_as::<Input, Output>(&ctx.pic, ctx.canister_id, sender, method, input);

    match resp {
        Ok(resp) => resp,
        Err(e) => {
            let msg = match e {
                CallError::Reject(msg) => msg,
                CallError::UserError(resp) => resp.to_string(),
            };
            panic!("Failed to call \"{}\": {}", method, msg);
        }
    }
}
