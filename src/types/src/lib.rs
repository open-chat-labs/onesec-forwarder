use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EvmAddress {
    chain: EvmChain,
    address: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvmChain {
    Ethereum,
    Arbitrum,
    Base,
}
