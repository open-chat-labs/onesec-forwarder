use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EvmAddress {
    pub chain: EvmChain,
    pub address: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvmChain {
    Ethereum,
    Arbitrum,
    Base,
}

#[allow(non_camel_case_types)]
#[derive(CandidType, Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Token {
    BOB,
    ICP,
    GLDT,
    USDC,
    USDT,
    cbBTC,
    ckBTC,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum IcpAccount {
    ICRC(IcrcAccount),
    AccountId(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct IcrcAccount {
    pub owner: Principal,
    pub subaccount: Option<[u8; 32]>,
}
