use candid::Principal;

use crate::context::PROD_DEVELOPER_PRINCIPAL;

pub enum Sender {
    ProductionDeveloper,
    Other(Principal),
}

impl Sender {
    pub fn principal(&self) -> Principal {
        match self {
            Sender::ProductionDeveloper => Principal::from_text(PROD_DEVELOPER_PRINCIPAL).unwrap(),
            Sender::Other(principal) => *principal,
        }
    }
}
