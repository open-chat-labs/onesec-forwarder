use onesec_forwarder_lambda_core::{GetRecipientsResult, RecipientContractAddress};
use onesec_forwarder_types::EvmChain;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct EthRpcClient {
    client: Client,
    api_key: String,
    max_blocks_per_request: u32,
}

impl EthRpcClient {
    pub fn new(api_key: String, max_blocks_per_request: u32) -> Self {
        EthRpcClient {
            client: Client::new(),
            api_key,
            max_blocks_per_request,
        }
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
    async fn get_recipients(
        &self,
        chain: EvmChain,
        from_block: u64,
        contract_addresses: Vec<String>,
    ) -> Result<GetRecipientsResult, String> {
        let to_block = from_block - 1 + self.max_blocks_per_request as u64;

        let params = GetLogsParams {
            from_block: format_block_height(from_block),
            to_block: format_block_height(to_block),
            address: contract_addresses,
            topics: Vec::new(),
        };

        let logs_response: EthRpcResponse<Vec<LogResponse>> = self
            .client
            .post(self.url(chain))
            .json(&EthRpcRequest::new("eth_getLogs", vec![params]))
            .send()
            .await
            .map_err(|e| format!("Failed to send request to ETH RPC API: {e}"))?
            .json()
            .await
            .map_err(|e| format!("Failed to process response from ETH RPC API: {e}"))?;

        let recipients = logs_response
            .result
            .into_iter()
            .filter_map(|mut l| {
                if l.topics.len() == 3 {
                    Some(RecipientContractAddress {
                        recipient_address: format!("0x{}", &l.topics.pop().unwrap()[26..]),
                        contract_address: l.address,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(GetRecipientsResult {
            recipients,
            next_block_height: to_block + 1,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EthRpcRequest<T> {
    jsonrpc: String,
    id: u32,
    method: &'static str,
    params: T,
}

impl<T> EthRpcRequest<T> {
    fn new(method: &'static str, params: T) -> Self {
        EthRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method,
            params,
        }
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
}

fn format_block_height(block: u64) -> String {
    format!("0x{block:x}")
}
