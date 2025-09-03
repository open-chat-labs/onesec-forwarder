use crate::onesec_minter_canister::GetMetadataResponse;
use candid::{CandidType, Principal};
use ic_agent::Agent;
use ic_agent::agent::AgentBuilder;
use onesec_forwarder_canister_types::{ForwardingAddressesArgs, ForwardingAddressesResult};
use onesec_forwarder_lambda_core::{
    OneSecForwarderClient, OneSecMinterClient, TokenContractAddressesReader,
};
use onesec_forwarder_types::*;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CanisterClient {
    agent: Agent,
    canister_id: Principal,
}

impl CanisterClient {
    pub fn new(canister_id: Principal) -> Self {
        CanisterClient {
            agent: AgentBuilder::default().build().unwrap(),
            canister_id,
        }
    }
}

fn encode_args<A: CandidType>(args: A) -> Vec<u8> {
    candid::encode_one(args).unwrap()
}

fn decode_response<R: CandidType + DeserializeOwned>(bytes: &[u8]) -> Result<R, String> {
    candid::decode_one(bytes).map_err(|e| e.to_string())
}

impl OneSecMinterClient for CanisterClient {
    async fn forward_evm_to_icp(
        &self,
        token: Token,
        evm_address: EvmAddress,
        icp_account: IcpAccount,
    ) -> Result<(), String> {
        let args = onesec_minter_canister::ForwardEvmToIcpArgs {
            token,
            chain: evm_address.chain,
            address: evm_address.address,
            receiver: icp_account,
        };

        self.agent
            .update(&self.canister_id, "forward_evm_to_icp")
            .with_arg(encode_args(args))
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl TokenContractAddressesReader for CanisterClient {
    async fn get(&self) -> Result<HashMap<EvmChain, Vec<TokenContractAddress>>, String> {
        let response: GetMetadataResponse = self
            .agent
            .query(&self.canister_id, "get_metadata")
            .await
            .map_err(|e| e.to_string())
            .and_then(|r| decode_response::<GetMetadataResponse>(&r))?;

        let mut map: HashMap<EvmChain, Vec<TokenContractAddress>> = HashMap::new();
        for token_metadata in response.tokens {
            let Some(chain) = token_metadata.chain else {
                continue;
            };
            let Some(token) = token_metadata.token else {
                continue;
            };
            map.entry(chain).or_default().push(TokenContractAddress {
                token,
                address: token_metadata.contract,
            });
        }
        Ok(map)
    }
}

impl OneSecForwarderClient for CanisterClient {
    async fn forwarding_addresses(
        &self,
        evm_addresses: Vec<String>,
    ) -> Result<HashMap<String, IcpAccount>, String> {
        let args = ForwardingAddressesArgs { evm_addresses };

        self.agent
            .query(&self.canister_id, "forwarding_addresses")
            .with_arg(encode_args(args))
            .await
            .map_err(|e| e.to_string())
            .and_then(|r| decode_response::<ForwardingAddressesResult>(&r))
            .map(|r| r.forwarding_addresses)
    }
}

mod onesec_minter_canister {
    use super::*;

    #[derive(CandidType)]
    pub struct ForwardEvmToIcpArgs {
        pub token: Token,
        pub chain: EvmChain,
        pub address: String,
        pub receiver: IcpAccount,
    }

    #[derive(CandidType, Deserialize)]
    pub struct GetMetadataResponse {
        pub tokens: Vec<TokenMetadata>,
    }

    #[derive(CandidType, Deserialize)]
    pub struct TokenMetadata {
        pub token: Option<Token>,
        pub contract: String,
        pub chain: Option<EvmChain>,
    }
}
