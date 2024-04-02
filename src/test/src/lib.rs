// use std::cell::RefCell;
// use candid::Principal;
// use ic_cdk::api::management_canister::provisional::CanisterId;
// use lazy_static::lazy_static;
// use pocket_ic::PocketIc;

// #[cfg(test)]
// mod calls;
// #[cfg(test)]
// mod flow1;
// #[cfg(test)]
// mod flow2;
#[cfg(test)]
pub mod mocks;

// pub struct TestEnv {
//     pub pic: PocketIc,
//     pub canister_id: CanisterId,
// }

// lazy_static! {
//     pub static ref ENV: TestEnv = init_pocket_ic();
// }

// thread_local! {
//     pub static SENDER: RefCell<Option<Principal>> = RefCell::new(None);
//     pub static CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
//     pub static GROUP_ID: RefCell<Option<Principal>> = RefCell::new(None);
// }

// fn init_pocket_ic() -> TestEnv {
//     let pic = PocketIc::new();

//     // Create an empty canister as the anonymous principal.
//     let canister_id = pic.create_canister();

//     // Add cycles to the canister.
//     pic.add_cycles(canister_id, 2_000_000_000_000); // 2T cycles

//     // Install the wasm module into the canister.
//     let wasm_bytes =
//         std::fs::read("../../wasm/canister.wasm.gz").expect("Failed to read wasm file");
//     pic.install_canister(canister_id, wasm_bytes, vec![], None);

//     TestEnv { pic, canister_id }
// }

// #[test]
// fn run_tests() {
//     flow1::flow1();
//     flow2::flow2();
// }
