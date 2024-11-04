use candid::Principal;
use csv::ReaderBuilder;
use ic_ledger_types::{AccountIdentifier, DEFAULT_SUBACCOUNT};
use serde_json::{from_reader, Value};
use std::{collections::HashMap, fs::File, io::BufReader};

#[test]
pub fn test_alpha_mapping() {
    let original_holders = get_web3_original_holder_accounts();
    let alpha_holders = get_web3_holder_accounts();
    // let mut found = 0;
    // let mut not_found = 0;

    println!("Original holders: {:?}", original_holders.len());
    println!("Alpha holders: {:?}", alpha_holders.len());

    // for (principal, account) in profile_data {
    //     if alpha_holders.contains(&account) {
    //         println!("Principal: {:?} Account: {}", principal, account);
    //         found += 1;
    //     } else {
    //         println!("Principal: {:?} Account: {} not found", principal, account);
    //         not_found += 1;
    //     }
    // }

    // println!("Found: {} Not Found: {}", found, not_found);
}

pub fn get_profile_principals_and_accounts() -> HashMap<Principal, String> {
    let mut principals: HashMap<Principal, String> = HashMap::new();
    // Open the JSON file
    let file = File::open("/Users/rem.codes/Documents/rem.codes/Projects/Catalyze/canisters/proxy/src/proxy/tests/airdrop_data/profiles.json").expect("Failed to open profiles.json");
    let reader = BufReader::new(file);

    let data: Vec<Value> = from_reader(reader).expect("Failed to deserialize profiles.json");

    // Iterate through each entry and extract the "0" field as a string
    for entry in data {
        if let Some(principal_str) = entry.get("0").and_then(|v| v.as_str()) {
            let principal = Principal::from_text(principal_str)
                .expect("Failed to convert principal string to principal");
            let acc = AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT).to_hex();
            principals.insert(principal, acc);
        };
    }

    principals
}

pub fn get_web3_holder_accounts() -> Vec<String> {
    // Open the CSV file
    let file_path = "/Users/rem.codes/Documents/rem.codes/Projects/Catalyze/canisters/proxy/src/proxy/tests/airdrop_data/alphas_holders.csv";
    let file = File::open(file_path).expect("Failed to open web-3-alphas_holders.csv");

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Collect the accountIdentifier column values
    let account_identifiers: Vec<String> = rdr
        .records()
        .filter_map(|result| result.ok()) // Filter out any errors
        .filter_map(|record| record.get(0).map(|s| s.to_string())) // Get accountIdentifier
        .collect();

    account_identifiers
}

pub fn get_web3_original_holder_accounts() -> Vec<String> {
    // Open the CSV file
    let file_path = "/Users/rem.codes/Documents/rem.codes/Projects/Catalyze/canisters/proxy/src/proxy/tests/airdrop_data/alpha_original.csv";
    let file = File::open(file_path).expect("Failed to open web-3-alphas_holders.csv");

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Collect the accountIdentifier column values
    let account_identifiers: Vec<String> = rdr
        .records()
        .filter_map(|result| result.ok()) // Filter out any errors
        .filter_map(|record| {
            record.get(0).map(|s| {
                AccountIdentifier::new(&Principal::from_text(s).unwrap(), &DEFAULT_SUBACCOUNT)
                    .to_hex()
            })
        }) // Get accountIdentifier
        .collect();

    account_identifiers
}
