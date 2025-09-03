use onesec_forwarder_lambda_core::RecipientContractAddress;
use onesec_forwarder_types::EvmChain;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct EthRpcClient {
    client: Client,
    api_key: String,
}

impl EthRpcClient {
    pub fn new(api_key: String) -> Self {
        EthRpcClient {
            client: Client::new(),
            api_key,
        }
    }

    fn url(&self, chain: EvmChain) -> String {
        let chain_name = match chain {
            EvmChain::Ethereum => "eth",
            EvmChain::Arbitrum => "arbitrum",
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
        to_block: u64,
        contract_addresses: Vec<String>,
    ) -> Result<Vec<RecipientContractAddress>, String> {
        let params = GetLogsParams {
            from_block: from_block.to_string(),
            to_block: to_block.to_string(),
            address: contract_addresses,
            topics: Vec::new(),
        };

        let logs_response: EthRpcResponse<Vec<LogResponse>> = self
            .client
            .post(self.url(chain))
            .header("content-type", "application/json")
            .body(serde_json::to_vec(&EthRpcRequest::new("eth_getLogs", params)).unwrap())
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        Ok(logs_response
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
            .collect())
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
    from_block: String,
    to_block: String,
    address: Vec<String>,
    topics: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct LogResponse {
    address: String,
    topics: Vec<String>,
}
