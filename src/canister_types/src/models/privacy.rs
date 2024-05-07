use std::fmt;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(
    CandidType, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug,
)]
pub enum Privacy {
    #[default]
    Public,
    Private,
    InviteOnly,
    Gated(GatedType),
}

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GatedType {
    Token(Vec<TokenGated>),
    Neuron(Vec<NeuronGated>),
}

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct NeuronGated {
    pub name: String,
    pub description: String,
    pub governance_canister: Principal,
    pub ledger_canister: Principal,
    pub rules: Vec<NeuronGatedRules>,
}

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NeuronGatedRules {
    MinAge(u64),
    MinStake(u64),
    MinDissolveDelay(u64),
    IsDisolving(bool),
}

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TokenGated {
    pub name: String,
    pub description: String,
    pub standard: String,
    pub principal: Principal,
    pub amount: u64,
}

impl fmt::Display for Privacy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Privacy::*;
        match self {
            Public => write!(f, "None"),
            Private => write!(f, "Private"),
            InviteOnly => write!(f, "InviteOnly"),
            Gated(_) => write!(f, "NftGated"),
        }
    }
}
