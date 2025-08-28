use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub use one_sec_deposit_notifier_types::*;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum InitOrUpgradeArgs {
    Init(InitArgs),
    Upgrade(UpgradeArgs),
}

impl InitOrUpgradeArgs {
    pub fn init(self) -> InitArgs {
        let InitOrUpgradeArgs::Init(init) = self else {
            panic!("Not of type Init");
        };
        init
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub whitelisted_callers: Vec<Principal>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UpgradeArgs {}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct FilterAddressesArgs {
    pub addresses: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct FilterAddressesResult {
    pub tracked_addresses: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct IsTrackingAddressArgs {
    pub address: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NotifyMinterArgs {
    pub addresses: Vec<EvmAddress>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct TrackAddressArgs {
    pub address: String,
}
