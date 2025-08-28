use crate::onesec_minter_canister::GetMetadataResponse;
use candid::{CandidType, Principal};
use onesec_forwarder_lambda_core::{OneSecMinterClient, TokenContractAddressesReader};
use onesec_forwarder_types::*;
use serde::Deserialize;
use std::rc::Rc;

pub struct OneSecMinterAgent {
    agent: Rc<ic_agent::Agent>,
    canister_id: Principal,
}

impl OneSecMinterClient for OneSecMinterAgent {
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
            .with_arg(candid::encode_one(&args).unwrap())
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl TokenContractAddressesReader for OneSecMinterAgent {
    async fn get(&self) -> Result<Vec<(Token, EvmAddress)>, String> {
        let response_bytes = self
            .agent
            .query(&self.canister_id, "get_metadata")
            .await
            .map_err(|e| e.to_string())?;

        let metadata: GetMetadataResponse =
            candid::decode_one(&response_bytes).map_err(|e| e.to_string())?;

        Ok(metadata
            .tokens
            .into_iter()
            .filter_map(|t| {
                let token = t.token?;
                let chain = t.chain?;

                Some((
                    token,
                    EvmAddress {
                        chain,
                        address: t.contract,
                    },
                ))
            })
            .collect())
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
