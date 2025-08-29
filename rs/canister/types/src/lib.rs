use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use onesec_forwarder_types::*;

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
