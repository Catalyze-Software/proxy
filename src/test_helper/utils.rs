use candid::Principal;
use rand::Rng;

pub fn generate_principal() -> Principal {
    let random_bytes: Vec<u8> = (0..29).map(|_| rand::thread_rng().gen()).collect();
    Principal::from_slice(&random_bytes)
}
