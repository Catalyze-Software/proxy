use std::fmt;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct StorageEntry {
    pub name: String,
    pub chunk: Vec<u8>,
    pub mime_type: String,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct StorageCanister {
    pub id: u32,
    pub principal: Principal,
    pub is_available: bool,
}

impl Default for StorageCanister {
    fn default() -> Self {
        Self {
            id: Default::default(),
            principal: Principal::anonymous(),
            is_available: Default::default(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ChunkData {
    pub index: u64,
    pub chunk_id: u64,
    pub canister: Principal,
}

impl Default for ChunkData {
    fn default() -> Self {
        Self {
            index: Default::default(),
            canister: Principal::anonymous(),
            chunk_id: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct Manifest {
    pub entries: Vec<ChunkData>,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub enum CanisterStorage {
    #[default]
    None,
    Chunk(ChunkData),
    Manifest(Manifest),
}

impl fmt::Display for CanisterStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CanisterStorage::*;
        match self {
            None => write!(f, "None"),
            Chunk(value) => {
                write!(f, "CanisterStorage - {:?}", serde_json::to_string(value))
            }
            Manifest(value) => write!(f, "NotFound - {:?}", serde_json::to_string(value)),
        }
    }
}
