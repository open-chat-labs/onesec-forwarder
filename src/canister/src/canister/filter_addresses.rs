use candid::CandidType;
use ic_cdk::query;
use serde::{Deserialize, Serialize};

#[query]
fn filter_addresses(args: FilterAddressesArgs) -> FilterAddressesResult {
    let tracked_addresses = args
        .addresses
        .into_iter()
        .filter(|a| crate::lib::is_address_tracked(&a))
        .collect();

    FilterAddressesResult { tracked_addresses }
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct FilterAddressesArgs {
    pub addresses: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct FilterAddressesResult {
    pub tracked_addresses: Vec<String>,
}
