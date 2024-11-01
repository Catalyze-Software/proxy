use candid::{encode_args, CandidType, Decode, Principal};

use canister_types::models::{
    api_error::ApiError,
    group::{GroupResponse, PostGroup},
    profile::{PostProfile, ProfileResponse},
    profile_privacy::ProfilePrivacy,
    reward::RewardableActivityResponse,
};
use pocket_ic::{CanisterSettings, PocketIc, PocketIcBuilder, WasmResult};

use serde::de::DeserializeOwned;

use crate::{sender::Sender, utils::generate_principal};

pub static PROD_DEVELOPER_PRINCIPAL: &str =
    "ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe";
pub static PROXY_CANISTER_ID: &str = "2jvhk-5aaaa-aaaap-ahewa-cai";
pub static REWARD_CANISTER_ID: &str = "zgfl7-pqaaa-aaaap-accpa-cai";

pub struct Context {
    pub pic: PocketIc,
    pub user_principal: Principal,
    pub ref_principal: Principal,
    pub prod_developer_principal: Principal,
    pub proxy_canister_id: Principal,
    pub reward_canister_id: Principal,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        let pic = PocketIcBuilder::new().with_application_subnet().build();
        let user_principal = generate_principal();
        let ref_principal = generate_principal();
        let prod_developer_principal = Principal::from_text(PROD_DEVELOPER_PRINCIPAL).unwrap();
        let proxy_canister_id = Principal::from_text(PROXY_CANISTER_ID).unwrap();
        let reward_canister_id = Principal::from_text(REWARD_CANISTER_ID).unwrap();

        let proxy_canister_id = Self::create_canister(
            &pic,
            proxy_canister_id,
            include_bytes!("../../wasm/proxy.wasm.gz").to_vec(),
        )
        .expect("Failed to create proxy canister");

        let reward_canister_id = Self::create_canister(
            &pic,
            reward_canister_id,
            include_bytes!("../../wasm/rewards.wasm.gz").to_vec(),
        )
        .expect("Failed to create reward canister");

        let context = Context {
            pic,
            user_principal,
            ref_principal,
            prod_developer_principal,
            proxy_canister_id,
            reward_canister_id,
        };

        Self::reward_update::<bool>(
            &context,
            Sender::ProductionDeveloper,
            "_dev_set_proxy",
            Some(encode_args((context.proxy_canister_id,)).unwrap()),
        )
        .expect("Failed to set proxy canister");

        Self::proxy_update::<Result<Principal, ApiError>>(
            &context,
            Sender::ProductionDeveloper,
            "_dev_set_reward_canister",
            Some(encode_args((context.reward_canister_id,)).unwrap()),
        )
        .expect("Failed to set reward canister")
        .expect("Failed to set reward canister");

        context
    }

    fn create_canister(
        pic: &PocketIc,
        canister_id: Principal,
        wasm_bytes: Vec<u8>,
    ) -> Result<Principal, String> {
        pic.create_canister_with_id(
            None,
            Some(CanisterSettings {
                controllers: Some(vec![canister_id]),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
                reserved_cycles_limit: None,
            }),
            canister_id,
        )?;

        pic.add_cycles(canister_id, 10_000_000_000_000);

        pic.install_canister(
            canister_id,
            wasm_bytes,
            encode_args(()).unwrap(),
            Some(canister_id),
        );
        Ok(canister_id)
    }

    pub fn proxy_query<T: DeserializeOwned + CandidType>(
        &self,
        sender: Sender,
        method: &str,
        args: Option<Vec<u8>>,
    ) -> Result<T, String> {
        let args = args.unwrap_or(encode_args(()).unwrap());
        let res = self
            .pic
            .query_call(self.proxy_canister_id, sender.principal(), method, args)
            .expect("Failed to call canister");

        match res {
            // expected
            WasmResult::Reply(res_bytes) => Ok(Decode!(res_bytes.as_slice(), T).unwrap()),
            // unexpected or method guard
            WasmResult::Reject(res) => Err(res),
        }
    }

    pub fn proxy_update<T: DeserializeOwned + CandidType>(
        &self,
        sender: Sender,
        method: &str,
        args: Option<Vec<u8>>,
    ) -> Result<T, String> {
        let args = args.unwrap_or(encode_args(()).unwrap());
        let res = self
            .pic
            .update_call(self.proxy_canister_id, sender.principal(), method, args)
            .expect("Failed to call canister");

        match res {
            // expected
            WasmResult::Reply(res_bytes) => Ok(Decode!(res_bytes.as_slice(), T).unwrap()),
            // unexpected or method guard
            WasmResult::Reject(res) => Err(res),
        }
    }

    pub fn reward_query<T: DeserializeOwned + CandidType>(
        &self,
        sender: Sender,
        method: &str,
        args: Option<Vec<u8>>,
    ) -> Result<T, String> {
        let args = args.unwrap_or(encode_args(()).unwrap());
        let res = self
            .pic
            .query_call(self.reward_canister_id, sender.principal(), method, args)
            .expect("Failed to call canister");

        match res {
            // expected
            WasmResult::Reply(res_bytes) => Ok(Decode!(res_bytes.as_slice(), T).unwrap()),
            // unexpected or method guard
            WasmResult::Reject(res) => Err(res),
        }
    }

    pub fn reward_update<T: DeserializeOwned + CandidType>(
        &self,
        sender: Sender,
        method: &str,
        args: Option<Vec<u8>>,
    ) -> Result<T, String> {
        let args = args.unwrap_or(encode_args(()).unwrap());
        let res = self
            .pic
            .update_call(self.reward_canister_id, sender.principal(), method, args)
            .expect("Failed to call canister");

        match res {
            // expected
            WasmResult::Reply(res_bytes) => Ok(Decode!(res_bytes.as_slice(), T).unwrap()),
            // unexpected or method guard
            WasmResult::Reject(res) => Err(res),
        }
    }

    pub fn create_default_profile_for_user(&self) -> ProfileResponse {
        let profile = self
            .proxy_update::<Result<ProfileResponse, ApiError>>(
                Sender::Other(self.user_principal),
                "add_profile",
                Some(
                    encode_args((PostProfile {
                        username: "default".to_string(),
                        display_name: "default".to_string(),
                        first_name: "default".to_string(),
                        last_name: "default".to_string(),
                        privacy: ProfilePrivacy::Public,
                        extra: "default".to_string(),
                    },))
                    .unwrap(),
                ),
            )
            .expect("Failed to create profile");

        assert!(profile.is_ok());
        let coc = self
            .proxy_update::<Result<bool, ApiError>>(
                Sender::Other(self.user_principal),
                "approve_code_of_conduct",
                Some(encode_args((0u64,)).unwrap()),
            )
            .expect("Failed to approve code of conduct");

        assert!(coc.is_ok());
        let pp = self
            .proxy_update::<Result<bool, ApiError>>(
                Sender::Other(self.user_principal),
                "approve_privacy_policy",
                Some(encode_args((0u64,)).unwrap()),
            )
            .expect("Failed to approve code of conduct");

        assert!(pp.is_ok());
        let tos = self
            .proxy_update::<Result<bool, ApiError>>(
                Sender::Other(self.user_principal),
                "approve_terms_of_service",
                Some(encode_args((0u64,)).unwrap()),
            )
            .expect("Failed to approve code of conduct");

        assert!(tos.is_ok());

        profile.unwrap()
    }

    pub fn create_profile_with_referral(
        &self,
        principal: Principal,
        post_profile: PostProfile,
    ) -> ProfileResponse {
        let profile = self
            .proxy_update::<Result<ProfileResponse, ApiError>>(
                Sender::Other(principal),
                "add_profile_by_referral",
                Some(encode_args((post_profile, self.ref_principal)).unwrap()),
            )
            .expect("Failed to create profile");

        assert!(profile.is_ok());
        let coc = self
            .proxy_update::<Result<bool, ApiError>>(
                Sender::Other(principal),
                "approve_code_of_conduct",
                Some(encode_args((0u64,)).unwrap()),
            )
            .expect("Failed to approve code of conduct");

        assert!(coc.is_ok());
        let pp = self
            .proxy_update::<Result<bool, ApiError>>(
                Sender::Other(principal),
                "approve_privacy_policy",
                Some(encode_args((0u64,)).unwrap()),
            )
            .expect("Failed to approve code of conduct");

        assert!(pp.is_ok());
        let tos = self
            .proxy_update::<Result<bool, ApiError>>(
                Sender::Other(principal),
                "approve_terms_of_service",
                Some(encode_args((0u64,)).unwrap()),
            )
            .expect("Failed to approve code of conduct");

        assert!(tos.is_ok());

        profile.unwrap()
    }

    pub fn create_profile(
        &self,
        principal: Principal,
        post_profile: PostProfile,
    ) -> ProfileResponse {
        let profile = self
            .proxy_update::<Result<ProfileResponse, ApiError>>(
                Sender::Other(principal),
                "add_profile",
                Some(encode_args((post_profile,)).unwrap()),
            )
            .expect("Failed to create profile");

        assert!(profile.is_ok());
        let coc = self
            .proxy_update::<Result<bool, ApiError>>(
                Sender::Other(principal),
                "approve_code_of_conduct",
                Some(encode_args((0u64,)).unwrap()),
            )
            .expect("Failed to approve code of conduct");

        assert!(coc.is_ok());
        let pp = self
            .proxy_update::<Result<bool, ApiError>>(
                Sender::Other(principal),
                "approve_privacy_policy",
                Some(encode_args((0u64,)).unwrap()),
            )
            .expect("Failed to approve code of conduct");

        assert!(pp.is_ok());
        let tos = self
            .proxy_update::<Result<bool, ApiError>>(
                Sender::Other(principal),
                "approve_terms_of_service",
                Some(encode_args((0u64,)).unwrap()),
            )
            .expect("Failed to approve code of conduct");

        assert!(tos.is_ok());

        profile.unwrap()
    }

    pub fn get_referred_by(&self, principal: Principal) -> Result<Principal, ApiError> {
        self.proxy_query::<Result<Principal, ApiError>>(
            Sender::Other(principal),
            "get_referred_by",
            None,
        )
        .expect("Failed to get referrals")
    }

    pub fn get_reward_buffer(&self) -> Vec<RewardableActivityResponse> {
        self.proxy_query::<Vec<RewardableActivityResponse>>(
            Sender::ProductionDeveloper,
            "read_reward_buffer",
            None,
        )
        .expect("Failed to get reward buffer")
    }

    pub fn create_group(&self, owner: Principal, post_group: PostGroup) -> GroupResponse {
        self.proxy_update::<Result<GroupResponse, ApiError>>(
            Sender::Other(owner),
            "add_group",
            Some(encode_args((post_group,)).unwrap()),
        )
        .expect("Failed to create group")
        .expect("Failed to create group")
    }
}
