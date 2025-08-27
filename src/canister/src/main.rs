use one_sec_deposit_notifier_canister as lib;

use candid::CandidType;
use ic_cdk::{init, query, update};
use lib::{DefaultPendingDepositNotifier, DefaultTrackedAddresses};
use serde::{Deserialize, Serialize};

fn main() {}

#[init]
fn init() {
    lib::init(
        DefaultTrackedAddresses::default(),
        DefaultPendingDepositNotifier::default(),
    );
}

#[update]
fn track_address(args: TrackAddressArgs) {
    lib::track_address(args.address);
}

#[query]
fn filter_tracked_addresses(args: FilterTrackedAddressesArgs) -> FilterTrackedAddressesResult {
    let tracked_addresses = args
        .addresses
        .into_iter()
        .filter(|a| lib::is_address_tracked(&a))
        .collect();

    FilterTrackedAddressesResult { tracked_addresses }
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct TrackAddressArgs {
    pub address: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct FilterTrackedAddressesArgs {
    pub addresses: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct FilterTrackedAddressesResult {
    pub tracked_addresses: Vec<String>,
}
