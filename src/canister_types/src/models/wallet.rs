use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, CandidType, Deserialize)]
pub struct Wallet {
    pub provider: String,
    pub is_primary: bool,
}

impl From<PostWallet> for Wallet {
    fn from(post_wallet: PostWallet) -> Self {
        Self {
            provider: post_wallet.provider,
            is_primary: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, CandidType, Deserialize)]
pub struct PostWallet {
    pub provider: String,
    pub principal: Principal,
}

#[derive(Clone, Debug, Serialize, CandidType, Deserialize)]
pub struct WalletResponse {
    pub provider: String,
    pub principal: Principal,
    pub is_primary: bool,
}
