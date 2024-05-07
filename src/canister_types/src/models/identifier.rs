use std::fmt;

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use super::api_error::ApiError;

// TODO: Remove allow once clean deployment will be available
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Clone, Debug, CandidType, Serialize, Deserialize, Eq, Hash)]
pub struct Identifier {
    id: u64,
    principal: Principal,
    kind: String,
    valid: bool,
}

pub static PROFILE_CANISTER_ID: &str = "4vy4w-gaaaa-aaaap-aa4pa-cai";
pub static MEMBER_CANISTER_ID: &str = "5nrjv-iaaaa-aaaap-aa4la-cai";
pub static GROUP_CANISTER_ID: &str = "5rvte-7aaaa-aaaap-aa4ja-cai";
pub static EVENT_CANISTER_ID: &str = "zocah-aqaaa-aaaap-aa4qa-cai";
pub static EVENT_ATTENDEE_CANISTER_ID: &str = "zaanp-3aaaa-aaaap-aa4ra-cai";
pub static REPORT_CANISTER_ID: &str = "zsg2w-xqaaa-aaaap-aa4sa-cai";

pub enum IdentifierKind {
    Profile(u64),
    Member(u64),
    Group(u64),
    Event(u64),
    EventAttendee(u64),
    Report(u64),
}

impl Identifier {
    pub fn new(id: u64, principal: Principal, kind: String) -> Result<Identifier, Box<ApiError>> {
        if kind.len() != 3 {
            return Err(ApiError::bad_request()
                .add_message("Invalid identifier: 'kind' length needs to be 3")
                .boxed());
        }

        Ok(Identifier {
            id,
            principal,
            kind,
            valid: true,
        })
    }

    #[deprecated = "Should be deprecated in the future, this only exists for backward compatibility"]
    pub fn generate(identifier: IdentifierKind) -> Identifier {
        let new_identifier = match identifier {
            IdentifierKind::Profile(id) => Identifier::new(
                id,
                Principal::from_text(PROFILE_CANISTER_ID).unwrap(),
                "pfe".to_string(),
            ),
            IdentifierKind::Member(id) => Identifier::new(
                id,
                Principal::from_text(MEMBER_CANISTER_ID).unwrap(),
                "mbr".to_string(),
            ),
            IdentifierKind::Group(id) => Identifier::new(
                id,
                Principal::from_text(GROUP_CANISTER_ID).unwrap(),
                "grp".to_string(),
            ),
            IdentifierKind::Event(id) => Identifier::new(
                id,
                Principal::from_text(EVENT_CANISTER_ID).unwrap(),
                "evt".to_string(),
            ),
            IdentifierKind::EventAttendee(id) => Identifier::new(
                id,
                Principal::from_text(EVENT_ATTENDEE_CANISTER_ID).unwrap(),
                "eae".to_string(),
            ),
            IdentifierKind::Report(id) => Identifier::new(
                id,
                Principal::from_text(REPORT_CANISTER_ID).unwrap(),
                "rpt".to_string(),
            ),
        };

        // We can unwrap here because we know that the identifier is valid
        new_identifier.unwrap()
    }

    pub fn to_principal(&self) -> Result<Principal, Box<ApiError>> {
        if self.kind.len() != 3 {
            return Err(ApiError::serialize()
                .add_message("Invalid identifier: 'kind' length needs to be 3")
                .boxed());
        }
        let mut array = Vec::new();
        array.extend_from_slice(b"\x0Acat");
        array.extend_from_slice(self.kind.as_bytes());
        array.extend_from_slice(self.principal.as_slice());
        array.extend_from_slice(&Self::to_u32_be_bytes(self.id));
        Ok(Principal::from_slice(&array))
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn canister(&self) -> Principal {
        self.principal
    }

    pub fn kind(&self) -> String {
        self.kind.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    fn to_u32_be_bytes(n: u64) -> [u8; 4] {
        [(n >> 24) as u8, (n >> 16) as u8, (n >> 8) as u8, n as u8]
    }

    fn from32bits(ba: &[u8]) -> u64 {
        let mut value = 0;
        for item in ba.iter().take(4) {
            value = (value << 8) | (*item as u64);
        }
        value
    }
}

impl From<Principal> for Identifier {
    fn from(principal: Principal) -> Self {
        let mut p = principal.as_slice().to_vec();
        let custom_identifier = p[..4].to_vec();

        if custom_identifier == b"\x0Acat".to_vec() {
            p.drain(..4);
            let kind = String::from_utf8(p[..3].to_vec()).unwrap();
            p.drain(..3);

            let index_bytes = p.drain(p.len() - 4..).collect::<Vec<u8>>();
            let index = Self::from32bits(&index_bytes);

            return Identifier {
                id: index,
                principal: Principal::from_slice(&p),
                kind,
                valid: true,
            };
        }

        Identifier {
            id: 0,
            principal,
            kind: "principal".to_string(),
            valid: false,
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {} - canister: {} - kind: {}",
            self.id, self.principal, self.kind,
        )
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.to_principal().map_or(false, |s| {
            Self::to_principal(other).map_or(false, |o| s == o)
        })
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            id: Default::default(),
            principal: Principal::anonymous(),
            kind: Default::default(),
            valid: Default::default(),
        }
    }
}

// #[test]
// fn test_decode() {
//     let x = Identifier::new(
//         0,
//         Principal::from_text("ve3v4-o7xuv-ijejl-vcyfx-hjy3b-owwtx-jte2k-2bciw-spskd-jgmvd-rqe")
//             .unwrap(),
//         "prl".to_string(),
//     )
//     .unwrap()
//     .to_principal();
//     match x {
//         Ok(_id) => println!("{:?}", _id.to_string()),
//         Err(_err) => println!("{:?}", _err),
//     }
//     // let (id, principal, kind) = Identifier::decode(
//     //     &Principal::from_text("ve3v4-o7xuv-ijejl-vcyfx-hjy3b-owwtx-jte2k-2bciw-spskd-jgmvd-rqe")
//     //         .unwrap(),
//     // );
//     // println!(
//     //     "id: {}\nprincipal: {}\nkind: {}",
//     //     id,
//     //     principal.to_string(),
//     //     kind
//     // );
// }
