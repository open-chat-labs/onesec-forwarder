use onesec_forwarder_lambda_core::{GetRecipientsResult, RecipientContractAddress};
use onesec_forwarder_types::EvmChain;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::collections::HashMap;

pub struct EthRpcClient {
    client: Client,
    api_key: String,
    max_blocks_per_request: HashMap<EvmChain, u32>,
}

impl EthRpcClient {
    pub fn new(api_key: String, max_blocks_per_request: HashMap<EvmChain, u32>) -> Self {
        EthRpcClient {
            client: Client::new(),
            api_key,
            max_blocks_per_request,
        }
    }

    async fn send<T: Serialize, R: DeserializeOwned>(
        &self,
        chain: EvmChain,
        request: EthRpcRequest<T>,
    ) -> Result<EthRpcResponse<R>, String> {
        self.client
            .post(self.url(chain))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request to ETH RPC API: {e}"))?
            .json()
            .await
            .map_err(|e| format!("Failed to process response from ETH RPC API: {e}"))
    }

    fn url(&self, chain: EvmChain) -> String {
        let chain_name = match chain {
            EvmChain::Ethereum => "eth",
            EvmChain::Arbitrum => "arb",
            EvmChain::Base => "base",
        };

        format!(
            "https://{chain_name}-mainnet.g.alchemy.com/v2/{}",
            self.api_key
        )
    }
}

impl onesec_forwarder_lambda_core::EthRpcClient for EthRpcClient {
    async fn latest_block(&self, chain: EvmChain) -> Result<u64, String> {
        let request: EthRpcRequest = EthRpcRequest::new("eth_blockNumber");
        let response: EthRpcResponse<String> = self.send(chain, request).await?;

        Ok(hex_decode(&response.result))
    }

    async fn get_recipients(
        &self,
        chain: EvmChain,
        from_block: u64,
        contract_addresses: Vec<String>,
    ) -> Result<GetRecipientsResult, String> {
        let to_block =
            from_block - 1 + self.max_blocks_per_request.get(&chain).copied().unwrap() as u64;

        let params = GetLogsParams {
            from_block: hex_encode(from_block),
            to_block: hex_encode(to_block),
            address: contract_addresses,
            topics: Vec::new(),
        };

        let request = EthRpcRequest::new("eth_getLogs").with_params(params);
        let response: EthRpcResponse<Vec<LogResponse>> = self.send(chain, request).await?;

        let mut recipients = Vec::new();
        let mut max_block = 0;

        for log in response.result {
            if log.topics.len() == 3 {
                recipients.push(RecipientContractAddress {
                    recipient_address: format!("0x{}", &log.topics.last().unwrap()[26..]),
                    contract_address: log.address,
                });
            }
            let block = hex_decode(&log.block_number);
            if block > max_block {
                max_block = block;
            }
        }

        Ok(GetRecipientsResult {
            recipients,
            next_block_height: max(from_block, max_block + 1),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EthRpcRequest<T = ()> {
    jsonrpc: String,
    id: u32,
    method: &'static str,
    params: Vec<T>,
}

impl<T> EthRpcRequest<T> {
    fn new(method: &'static str) -> Self {
        EthRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method,
            params: Vec::new(),
        }
    }

    fn with_params(mut self, params: T) -> Self {
        self.params.push(params);
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct EthRpcResponse<T> {
    jsonrpc: String,
    id: u32,
    result: T,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GetLogsParams {
    #[serde(rename = "fromBlock")]
    from_block: String,
    #[serde(rename = "toBlock")]
    to_block: String,
    address: Vec<String>,
    topics: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct LogResponse {
    address: String,
    topics: Vec<String>,
    #[serde(rename = "blockNumber")]
    block_number: String,
}

fn hex_encode(block: u64) -> String {
    format!("0x{block:x}")
}

fn hex_decode(s: &str) -> u64 {
    u64::from_str_radix(s.strip_prefix("0x").unwrap(), 16).unwrap()
}
