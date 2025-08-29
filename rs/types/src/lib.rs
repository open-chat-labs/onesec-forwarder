use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EvmAddress {
    pub chain: EvmChain,
    pub address: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EvmChain {
    Ethereum,
    Arbitrum,
    Base,
}

#[allow(non_camel_case_types)]
#[derive(CandidType, Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TokenContractAddress {
    pub token: Token,
    pub address: String,
}

impl Display for EvmChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            EvmChain::Ethereum => f.write_str("Ethereum"),
            EvmChain::Arbitrum => f.write_str("Arbitrum"),
            EvmChain::Base => f.write_str("Base"),
        }
    }
}
