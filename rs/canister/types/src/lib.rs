use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use onesec_forwarder_types::*;

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
pub struct EnableForwardingArgs {
    pub icp_account: IcpAccount,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ForwardingAddressesArgs {
    pub evm_addresses: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ForwardingAddressesResult {
    pub forwarding_addresses: HashMap<String, IcpAccount>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct IsForwardingArgs {
    pub evm_address: String,
}
